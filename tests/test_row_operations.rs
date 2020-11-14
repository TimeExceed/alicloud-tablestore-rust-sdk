use tablestore as ots;
use tokio::stream::StreamExt;
mod utils;
use utils::*;

#[tokio::test]
async fn put_row() -> Result<(), ots::Error> {
    flexi_logger::Logger
        ::with_env()
        .format(flexi_logger::colored_with_thread)
        .start()
        .unwrap();
    let (ep, cred) = fetch_endpoint_credential()?;
    let opts = ots::ClientOptions::default();
    let client = ots::Client::new(ep, cred, opts)?;
    let table_name = "put_row".to_string();
    let table = TableBench::new(client.clone(), table_name.clone()).await?;
    {
        let row = ots::Row{
            row_key: ots::RowKey(vec![ots::RowKeyColumn{
                name: ots::Name::new(table.pkey_names()[0]),
                value: ots::RowKeyValue::Str("haha".to_string()),
            }]),
            attrs: vec![
                ots::Attribute{
                    name: ots::Name::new("who"),
                    value: ots::AttrValue::Int(123),
                    timestamp: None,
                }
            ],
        };
        let req = ots::PutRowRequest::new(table_name.clone(), row)?;
        let _resp = client.put_row(req).await?;
    }
    let res: Vec<_> = {
        let start = ots::ExtendedRowKey::fill_with_infmin(table.pkey_names(), vec![])?;
        let end = ots::ExtendedRowKey::fill_with_infmax(table.pkey_names(), vec![])?;
        let req = ots::GetRangeRequest{
            table_name: table_name.clone().into(),
            inclusive_start: start,
            exclusive_end: end,
        };
        let row_stream = client.scan(req, 1)?;
        row_stream
            .map(|x| {
                format!("{}", x)
            })
            .collect()
            .await
    };
    std::mem::drop(table);
    assert_eq!(&res.join(","), "haha=>{who:123}");
    Ok(())
}

struct TableBench {
    client: ots::Client,
    table_name: String,
}

impl TableBench {
    pub async fn new(
        client: ots::Client,
        table_name: String,
    ) -> Result<Self, ots::Error> {
        let meta = ots::TableMeta{
            name: table_name.clone().into(),
            schema: vec![
                ots::PkeyColumnSchema{
                    name: ots::Name::new("laugh"),
                    type_: ots::PkeyValueType::Str,
                }
            ]
        };
        let req = ots::CreateTableRequest::new(meta);
        let _resp = client.create_table(req).await?;
        Ok(Self{
            client,
            table_name,
        })
    }

    pub fn pkey_names(&self) -> &[&str] {
        &["laugh"]
    }
}

impl Drop for TableBench {
    fn drop(&mut self) {
        println!("drop TableBench");
        let client = self.client.clone();
        let table_name = self.table_name.clone();
        tokio::spawn(async move {
            client.delete_table(table_name).await.unwrap();
        });
    }
}