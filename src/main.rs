use tablestore as ots;

async fn async_gogogo(
    ep: ots::Endpoint,
    cred: ots::Credential,
) -> Result<(), ots::Error> {
    let client = ots::Client::new(ep, cred)?;
    let resp = client.list_table().await?;
    for t in resp.tables.iter() {
        println!("table: {}", t);
    }
    Ok(())
}

fn gogogo() -> Result<(), ots::Error> {
    let ep = ots::Endpoint::new(
        "http://taoda-test.cn-hangzhou.ots.aliyuncs.com", 
        "taoda-test",
    )?;
    let cred = ots::Credential::new(
        "dZKNjrnI3IWuvDYm",
        "TgjKZlEKlQkHjBG0pi7Q6uCEG6jgnx",
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
