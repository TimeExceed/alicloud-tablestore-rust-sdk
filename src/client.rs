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
        let api = types::Api::new(types::LIST_TABLE);
        let (tx, rx) = oneshot::channel();
        let cmd = client_impl::Cmd::ListTable(api, req, tx);
        self.cmd_sender.clone().send(cmd).await.unwrap();
        rx.await.unwrap()
    }

    pub async fn create_table(
        &self,
        table_meta: types::TableMeta,
        options: types::TableOptions,
    ) -> Result<types::CreateTableResponse, Error> {
        let req = types::CreateTableRequest{
            table_meta,
            options,
        };
        let api = types::Api::new(types::CREATE_TABLE);
        let (tx, rx) = oneshot::channel();
        let cmd = client_impl::Cmd::CreateTable(api, req, tx);
        self.cmd_sender.clone().send(cmd).await.unwrap();
        rx.await.unwrap()
    }

    pub async fn delete_table(
        &self,
        name: String,
    ) -> Result<types::DeleteTableResponse, Error> {
        let req = types::DeleteTableRequest{
            name,
        };
        let api = types::Api::new(types::DELETE_TABLE);
        let (tx, rx) = oneshot::channel();
        let cmd = client_impl::Cmd::DeleteTable(api, req, tx);
        self.cmd_sender.clone().send(cmd).await.unwrap();
        rx.await.unwrap()
    }
}

