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

async fn async_gogogo(
    ep: ots::Endpoint,
    cred: ots::Credential,
) -> Result<(), ots::Error> {
    let opts = ots::ClientOptions::default();
    let client = ots::Client::new(ep, cred, opts)?;
    {
        let meta = ots::TableMeta{
            name: ots::Name::new("Smile"),
            schema: vec![
                ots::PkeyColumnSchema{
                    name: ots::Name::new("haha"),
                    type_: ots::PkeyValueType::Str,
                },
            ],
        };
        let req = ots::CreateTableRequest::new(meta);
        let _resp = client.create_table(req).await?;
    }
    {
        let resp = client.list_table().await?;
        for t in resp.tables.iter() {
            println!("table: {}", <&str>::from(t));
        }
    }
    {
        let _resp = client.delete_table("Smile").await?;
    }
    {
        let resp = client.list_table().await?;
        for t in resp.tables.iter() {
            println!("table: {}", <&str>::from(t));
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
