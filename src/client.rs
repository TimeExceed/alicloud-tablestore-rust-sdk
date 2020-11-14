use crate::{Endpoint, Credential, ClientOptions, Error, types};
use crate::client_impl;
use log::*;
use tokio::sync::{mpsc, oneshot};

#[derive(Clone)]
pub struct Client {
    cmd_sender: mpsc::Sender<client_impl::Cmd>,
}

impl Client {
    pub fn new(
        endpoint: Endpoint,
        credential: Credential,
        opts: ClientOptions,
    ) -> Result<Client, Error> {
        let tx = client_impl::ClientImpl::new(endpoint, credential, opts);
        let res = Client{
            cmd_sender: tx,
        };
        Ok(res)
    }

    pub async fn list_table(&self) -> Result<types::ListTableResponse, Error> {
        debug!("Issue ListTable");
        let req = types::ListTableRequest{};
        let (tx, rx) = oneshot::channel();
        let cmd = client_impl::Cmd::ListTable(req, tx);
        self.cmd_sender.clone().send(cmd).await.unwrap();
        rx.await.unwrap()
    }

    pub async fn create_table(
        &self,
        req: types::CreateTableRequest,
    ) -> Result<types::CreateTableResponse, Error> {
        let (tx, rx) = oneshot::channel();
        let cmd = client_impl::Cmd::CreateTable(req, tx);
        self.cmd_sender.clone().send(cmd).await.unwrap();
        rx.await.unwrap()
    }

    pub async fn delete_table<T: ToString>(
        &self,
        name: T,
    ) -> Result<types::DeleteTableResponse, Error> {
        let req = types::DeleteTableRequest{
            name: types::Name::new(name),
        };
        let (tx, rx) = oneshot::channel();
        let cmd = client_impl::Cmd::DeleteTable(req, tx);
        self.cmd_sender.clone().send(cmd).await.unwrap();
        rx.await.unwrap()
    }

    pub async fn put_row(
        &self,
        req: types::PutRowRequest,
    ) -> Result<types::PutRowResponse, Error> {
        let (tx, rx) = oneshot::channel();
        let cmd = client_impl::Cmd::PutRow(req, tx);
        self.cmd_sender.clone().send(cmd).await.unwrap();
        rx.await.unwrap()
    }

    pub async fn get_range(
        &self,
        req: types::GetRangeRequest,
    ) -> Result<types::GetRangeResponse, Error> {
        let (tx, rx) = oneshot::channel();
        let cmd = client_impl::Cmd::GetRange(req, tx);
        self.cmd_sender.clone().send(cmd).await.unwrap();
        rx.await.unwrap()
    }

    pub fn scan(
        &self,
        req: types::GetRangeRequest,
        buffered_rows: usize,
    ) -> Result<mpsc::Receiver<types::Row>, Error> {
        let (tx, rx) = mpsc::channel(buffered_rows);
        let client = self.clone();
        tokio::spawn(async move {
            let forward = req.inclusive_start < req.exclusive_end;
            let mut req = req;
            let mut tx = tx;
            loop {
                match client.get_range(req.clone()).await {
                    Err(err) => {
                        error!("Got error while scanning table.\
                            \ttable: {}\
                            \terror: {:?}",
                            req.table_name,
                            err);
                        break;
                    }
                    Ok(resp) => {
                        for r in resp.rows.into_iter() {
                            tx.send(r).await
                                .unwrap_or_default();
                        }
                        if let Some(next_row_key) = resp.next_row_key {
                            if forward {
                                if next_row_key >= req.exclusive_end {
                                    break;
                                }
                            } else {
                                if next_row_key <= req.exclusive_end {
                                    break;
                                }
                            }
                            req.inclusive_start = next_row_key;
                        } else {
                            break;
                        }
                    }
                }
            }
        });
        Ok(rx)
    }
}

