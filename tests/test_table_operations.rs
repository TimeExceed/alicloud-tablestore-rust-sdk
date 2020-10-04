use std::convert::TryInto;
use tablestore as ots;
mod utils;
use utils::*;

#[tokio::test]
async fn create_delete() -> Result<(), ots::Error> {
    let (ep, cred) = fetch_endpoint_credential()?;
    let opts = ots::ClientOptions::default();
    let client = ots::Client::new(ep, cred, opts)?;
    let table_name = "create_delete".to_string();
    {
        let meta = ots::TableMeta{
            name: table_name.clone().into(),
            schema: vec![
                ots::PkeyColumnSchema{
                    name: ots::Name::new("haha"),
                    type_: ots::PkeyValueType::Str,
                }
            ]
        };
        let req = ots::CreateTableRequest::new(meta);
        let _resp = client.create_table(req).await?;
    }
    let should_in = {
        let resp = client.list_table().await?;
        let tables = resp.tables;
        tables.iter()
            .any(|x| {
                x == &table_name
            })
    };
    let _resp = client.delete_table(table_name.clone()).await?;
    let should_not_in = {
        let resp = client.list_table().await?;
        let tables = resp.tables;
        tables.iter()
            .any(|x| {
                x == &table_name
            })
    };

    assert_eq!((should_in, should_not_in), (true, false));
    Ok(())
}
