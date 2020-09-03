use tablestore as ots;
mod utils;
use utils::*;

#[tokio::test]
async fn concurrency() -> Result<(), ots::Error> {
    let (ep, cred) = fetch_endpoint_credential()?;
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
