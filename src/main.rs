use tablestore as ots;

fn try_me<T>(v: Result<T, std::env::VarError>) -> Result<T, ots::Error> {
    match v {
        Ok(x) => Ok(x),
        Err(err) => {
            let err = ots::Error{
                code: ots::ErrorCode::ClientUnknown,
                message: format!("{:?}", err),
            };
            Err(err)
        }
    }
}

struct TableFinalizer {
    client: ots::Client,
    name: String,
}

impl TableFinalizer {
    fn new(client: ots::Client, name: String) -> TableFinalizer {
        TableFinalizer{
            client,
            name,
        }
    }
}

impl Drop for TableFinalizer {
    fn drop(&mut self) {
        let name = self.name.clone();
        let client = self.client.clone();
        tokio::spawn(async move {
            client.delete_table(&name).await.unwrap();
        });
    }
}

async fn async_gogogo(
    ep: ots::Endpoint,
    cred: ots::Credential,
) -> Result<(), ots::Error> {
    let opts = ots::ClientOptions::default();
    let client = ots::Client::new(ep, cred, opts)?;
    let table_name = "Smile".to_string();
    let pkey_names = vec!["pkey"];
    // let _x = TableFinalizer::new(client.clone(), table_name.clone());
    // {
    //     let meta = ots::TableMeta{
    //         name: table_name.clone().into(),
    //         schema: vec![
    //             ots::PkeyColumnSchema{
    //                 name: ots::Name::new(pkey_names[0]),
    //                 type_: ots::PkeyValueType::Str,
    //             },
    //         ],
    //     };
    //     let req = ots::CreateTableRequest::new(meta);
    //     let _resp = client.create_table(req).await?;
    // }
    // {
    //     let row = ots::Row{
    //         row_key: ots::RowKey(vec![ots::RowKeyColumn{
    //             name: ots::Name::new(pkey_names[0]),
    //             value: ots::RowKeyValue::Str("exist".to_string()),
    //         }]),
    //         attrs: vec![
    //             ots::Attribute{
    //                 name: ots::Name::new("attr"),
    //                 value: ots::AttrValue::Int(123),
    //                 timestamp: ots::AttrTimestamp::ClientAttach(ots::DateTime::now()),
    //             }
    //         ],
    //     };
    //     let req = ots::PutRowRequest::new(table_name.clone(), row)?;
    //     let resp = client.put_row(req).await?;
    //     println!("put row ok: {:?}", resp);
    // }
    {
        let start = ots::ExtendedRowKey::fill_with_infmin(&pkey_names, vec![])?;
        let end = ots::ExtendedRowKey::fill_with_infmax(&pkey_names, vec![])?;
        let req = ots::GetRangeRequest{
            table_name: table_name.clone().into(),
            inclusive_start: start,
            exclusive_end: end,
        };
        let mut row_stream = client.scan(req, 1)?;
        println!("get-range ok");
        while let Some(x) = row_stream.recv().await {
            println!("{:?}", x);
        }
    }
    Ok(())
}

fn gogogo() -> Result<(), ots::Error> {
    let ep = ots::Endpoint::new(
        try_me(std::env::var("OTS_ENDPOINT"))?,
        try_me(std::env::var("OTS_INSTANCE"))?,
    )?;
    let cred = ots::Credential::new(
        try_me(std::env::var("OTS_AK_ID"))?,
        try_me(std::env::var("OTS_AK_SECRET"))?,
    )?;
    let mut res: Result<(), ots::Error> = Ok(());
    {
        let mut rt = tokio::runtime::Builder::new()
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            res = async_gogogo(ep, cred).await;
        });
    }
    res
}

fn main() {
    flexi_logger::Logger
        ::with_env()
        .format(flexi_logger::colored_with_thread)
        .start()
        .unwrap();
    match gogogo() {
        Ok(_) => {
            println!("done");
        }
        Err(err) => {
            println!("{:?}", err);
            std::process::exit(1);
        }
    }
}
