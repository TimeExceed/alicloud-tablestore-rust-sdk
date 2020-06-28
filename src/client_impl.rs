use bytes::Bytes;
use chrono::prelude::*;
use crate::{Endpoint, Credential, Error, ErrorCode, types};
use crypto::digest::Digest;
use crypto::mac::Mac;
use log::*;
use std::collections::BTreeMap;
use std::convert::{TryFrom, TryInto};
use tokio::stream::StreamExt;
use tokio::sync::{mpsc, oneshot};

#[derive(Clone)]
pub(crate) struct ClientImpl {
    endpoint: Endpoint,
    credential: Credential,
    http_clients: hyper::Client<hyper::client::HttpConnector<hyper::client::connect::dns::GaiResolver>, hyper::Body>,
}

impl ClientImpl {
    pub(crate) fn new(
        endpoint: Endpoint, 
        credential: Credential,
    ) -> mpsc::Sender<Cmd> {
        let (tx, rx) = mpsc::channel(1);
        let client = ClientImpl{
            endpoint,
            credential,
            http_clients: hyper::Client::new(),
        };
        tokio::spawn(client.run(rx));
        tx
    }

    async fn run(self, mut cmd_recv: mpsc::Receiver<Cmd>) {
        while let Some(cmd) = cmd_recv.recv().await {
            match cmd {
                Cmd::ListTable(api, req, resp_tx) => {
                    self.async_issue(api, req, resp_tx);
                }
            }
        }
    }

    fn async_issue<Req, Resp>(
        &self,
        api: types::Api,
        req: Req,
        resp_tx: oneshot::Sender<Result<Resp, Error>>,
    ) -> ()
    where 
        Req: 'static + Into<Bytes> + Send,
        Resp: 'static + TryFrom<Vec<u8>, Error=Error> + std::fmt::Debug + Send,
    {
        let client = self.clone();
        tokio::spawn(async move {
            let resp = client.issue(api, req).await;
            resp_tx.send(resp).unwrap()
        });
    }

    async fn issue<Req, Resp>(
        &self,
        api: types::Api,
        req: Req,
    ) -> Result<Resp, Error>
    where
        Req: Into<Bytes>,
        Resp: TryFrom<Vec<u8>, Error=Error> + std::fmt::Debug,
    {
        debug!("Issue a request.\
            \tpath: {}",
            api.path);
        let resp = self.issue_req(api, req).await;
        let resp = match resp {
            Ok(resp) => {
                debug!("Ok to get the response.\
                    \tpath: {}\
                    \tresponse: {:?}",
                    api.path,
                    resp);
                resp
            }
            Err(err) => {
                info!("Fail to send the request.\
                    \tpath: {}\
                    \terror: {:?}",
                    api.path,
                    err);
                return Err(err);
            }
        };
        match self.build_response(api, resp).await {
            Ok(resp) => {
                debug!("Ok to parse the response.\
                    \tpath: {}\
                    \tresponse: {:?}",
                    api.path,
                    resp);
                Ok(resp)
            }
            Err(err) => {
                info!("Fail to parse the response.\
                    \tpath: {}\
                    \terror: {:?}",
                    api.path,
                    err);
                Err(err)
            }
        }
    }

    async fn issue_req<Req>(
        &self,
        api: types::Api,
        req: Req,
    ) -> Result<http::Response<hyper::Body>, Error> 
    where
        Req: Into<Bytes>,
    {
        let url = format!("{}/{}",
            self.endpoint.address,
            api.path);
        let mut req_builder = http::Request::builder()
            .method(http::method::Method::POST)
            .uri(url);
        let body: Bytes = req.into();
        self.build_headers(req_builder.headers_mut().unwrap(), &body)?;
        let (mut sender, bd) = hyper::Body::channel();
        let req = req_builder.body(bd)?;
        let body = sender.send_data(body);
        let resp = self.http_clients.request(req);
        body.await?;
        let resp = resp.await?;
        Ok(resp)
    }

    fn build_headers(
        &self, 
        req_headers: &mut http::HeaderMap<http::HeaderValue>,
        body: &[u8],
    ) -> Result<(), Error> {
        let mut builder = HeaderBuilder::new(req_headers);
        builder.set_api_version();
        builder.set_user_agent();
        builder.set_content_type();
        builder.set_content_length(body.len())?;
        builder.set_ak(&self.credential)?;
        builder.set_instance(&self.endpoint.instance)?;
        builder.set_datetime()?;
        let body_digest = content_md5(body)?;
        builder.set_content_md5(body_digest)?;
        builder.sign(&self.credential.secret)?;
        Ok(())
    }

    async fn build_response<Resp>(
        &self, 
        _api: types::Api,
        resp: http::Response<hyper::Body>,
    ) -> Result<Resp, Error> 
    where
        Resp: TryFrom<Vec<u8>, Error=Error>,
    {
        let status = match resp.status().as_u16() {
            x if x >= 200 && x < 300 => StatusKind::Ok,
            x if x == 502 => StatusKind::ErrorFromMiddle,
            _ => StatusKind::ErrorFromService,
        };
        let resp_body = resp.into_body();
        let body = collect_body(resp_body).await?;
        match status {
            StatusKind::Ok => Ok(body.try_into()?),
            StatusKind::ErrorFromService => Err(body.as_slice().try_into()?),
            StatusKind::ErrorFromMiddle => {
                let message = match String::from_utf8(body) {
                    Ok(msg) => msg,
                    Err(err) => {
                        return Err(Error{
                            code: ErrorCode::OTSServerUnavailable,
                            message: err.to_string(),
                        });
                    }
                };
                let error = Error{
                    code: ErrorCode::OTSServerUnavailable,
                    message,
                };
                Err(error)
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum Cmd {
    ListTable(
        types::Api, 
        types::ListTableRequest,
        oneshot::Sender<Result<types::ListTableResponse, Error>>),
}

const HEADER_NAME_API_VERSION: &str = "x-ots-apiversion";
const HEADER_VALUE_API_VERSION: &str = "2015-12-31";
const HEADER_NAME_ACCESS_KEY_ID: &str = "x-ots-accesskeyid";
const HEADER_NAME_INSTANCE_NAME: &str = "x-ots-instancename";
const HEADER_NAME_USER_AGENT: &str = "User-Agent";
const HEADER_VALUE_USER_AGENT: &str = "aliyun-tablestore-sdk-rust/0.1.0(x86_64;linux)";
const HEADER_VALUE_MIME_TYPE: &str = "application/x.pb2";
const HEADER_NAME_ACCESS_TOKEN: &str = "x-ots-ststoken";
const HEADER_NAME_OTS_DATE: &str = "x-ots-date";
const HEADER_NAME_CONTENT_MD5: &str = "x-ots-contentmd5";
const HEADER_NAME_SIGNATURE: &str = "x-ots-signature";

enum StatusKind {
    Ok,
    ErrorFromService,
    ErrorFromMiddle,
}

async fn collect_body(mut resp_body: hyper::Body) -> Result<Vec<u8>, Error> {
    let mut body: Vec<u8> = vec![];
    while let Some(piece) = resp_body.next().await {
        let piece = piece?;
        body.extend_from_slice(piece.as_ref());
    }
    Ok(body)
}

struct HeaderBuilder<'a> {
    ordered: BTreeMap<&'static str, Bytes>,
    raw: &'a mut http::HeaderMap<http::HeaderValue>,
}

impl HeaderBuilder<'_> {
    fn new(raw: &mut http::HeaderMap<http::HeaderValue>) -> HeaderBuilder {
        HeaderBuilder{
            ordered: BTreeMap::new(),
            raw,
        }
    }

    fn set_api_version(&mut self) {
        self.raw.insert(
            HEADER_NAME_API_VERSION, 
            http::HeaderValue::from_static(HEADER_VALUE_API_VERSION));
        self.ordered.insert(
            HEADER_NAME_API_VERSION, 
            Bytes::from_static(HEADER_VALUE_API_VERSION.as_bytes()));
    }

    fn set_ak(&mut self, cred: &Credential) -> Result<(), Error> {
        self.raw.insert(
            HEADER_NAME_ACCESS_KEY_ID, 
            http::HeaderValue::from_bytes(&cred.id)?);
        self.ordered.insert(
            HEADER_NAME_ACCESS_KEY_ID, 
            cred.id.clone());
        if let Some(token) = cred.token.as_ref() {
            self.raw.insert(
                HEADER_NAME_ACCESS_TOKEN, 
                http::HeaderValue::from_bytes(token)?);
            self.ordered.insert(
                HEADER_NAME_INSTANCE_NAME, 
                token.clone());
        }
        Ok(())
    }

    fn set_instance(&mut self, inst_name: &str) -> Result<(), Error> {
        self.raw.insert(
            HEADER_NAME_INSTANCE_NAME, 
            http::HeaderValue::from_str(inst_name)?);
        self.ordered.insert(
            HEADER_NAME_INSTANCE_NAME, 
            Bytes::copy_from_slice(inst_name.as_bytes()));
        Ok(())
    }

    fn set_user_agent(&mut self) {
        self.raw.insert(
            HEADER_NAME_USER_AGENT, 
            http::HeaderValue::from_static(HEADER_VALUE_USER_AGENT));
    }

    fn set_content_type(&mut self) {
        self.raw.insert(
            http::header::CONTENT_TYPE, 
            http::HeaderValue::from_static(HEADER_VALUE_MIME_TYPE));
        self.raw.insert(
            http::header::ACCEPT, 
            http::HeaderValue::from_static(HEADER_VALUE_MIME_TYPE));
    }

    fn set_datetime(&mut self) -> Result<(), Error> {
        let tm = Utc::now();
        let tm = Utc.ymd(tm.year(), tm.month(), tm.day()).and_hms_micro(tm.hour(), tm.minute(), tm.second(), tm.nanosecond()/1000);
        let tm = format!("{:?}", tm);
        self.raw.insert(
            HEADER_NAME_OTS_DATE, 
            http::HeaderValue::from_str(&tm)?);
        self.ordered.insert(HEADER_NAME_OTS_DATE, Bytes::from(tm));
        Ok(())
    }

    fn set_content_length(&mut self, len: usize) -> Result<(), Error> {
        let len = format!("{}", len);
        self.raw.insert(
            http::header::CONTENT_LENGTH, 
            http::HeaderValue::from_str(&len)?);
        Ok(())
    }

    fn set_content_md5(&mut self, body_digest: String) -> Result<(), Error> {
        self.raw.insert(
            HEADER_NAME_CONTENT_MD5, 
            http::HeaderValue::from_str(&body_digest)?);
        self.ordered.insert(
            HEADER_NAME_CONTENT_MD5, 
            Bytes::from(body_digest));
        Ok(())
    }

    fn sign(self, secret: &[u8]) -> Result<(), Error> {
        let hasher = crypto::sha1::Sha1::new();
        let mut hmac = crypto::hmac::Hmac::new(hasher, secret);
        hmac.input(b"/ListTable\nPOST\n\n");
        for (k, v) in self.ordered.iter() {
            hmac.input(k.as_bytes());
            hmac.input(b":");
            hmac.input(v.as_ref());
            hmac.input(b"\n");
        }
        let signature: crypto::mac::MacResult = hmac.result();
        let signature = base64::encode(signature.code());
        self.raw.insert(
            HEADER_NAME_SIGNATURE, 
            http::HeaderValue::from_str(&signature)?);
        Ok(())
    }
}

fn content_md5(body: &[u8]) -> Result<String, Error> {
    let mut digest = [0u8; 16];
    let mut ctx = crypto::md5::Md5::new();
    ctx.input(body);
    ctx.result(&mut digest);
    let digest = base64::encode(&digest);
    Ok(digest)
}
