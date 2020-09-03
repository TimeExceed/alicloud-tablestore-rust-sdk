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
                    name: "haha".to_string().into(),
                    type_: ots::PkeyValueType::Str,
                }
            ]
        };
        let opts = ots::TableOptions::default_for_create();
        let _resp = client.create_table(meta, opts).await?;
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
