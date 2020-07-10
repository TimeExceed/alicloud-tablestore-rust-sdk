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

#[tokio::test]
async fn concurrency() -> Result<(), ots::Error> {
    let ep = ots::Endpoint::new(
        try_me(std::env::var("OTS_ENDPOINT"))?,
        try_me(std::env::var("OTS_INSTANCE"))?,
    )?;
    let cred = ots::Credential::new(
        try_me(std::env::var("OTS_AK_ID"))?,
        try_me(std::env::var("OTS_AK_SECRET"))?,
    )?;
    let mut opts = ots::ClientOptions::default();
    opts.concurrency = 2;
    let client = ots::Client::new(ep, cred, opts)?;
    let x0 = client.list_table();
    let x1 = client.list_table();
    let x2 = client.list_table();
    let (r0, r1, r2) = tokio::join!(x0, x1, x2);
    assert!(r0.is_ok());
    assert!(r1.is_ok());
    assert!(r2.is_err());
    Ok(())
}
