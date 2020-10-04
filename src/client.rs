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
}

