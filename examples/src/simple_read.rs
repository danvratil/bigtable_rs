use bigtable_rs::bigtable;
use bigtable_rs::google::bigtable::v2::row_filter::{Chain, Filter};
use bigtable_rs::google::bigtable::v2::row_range::{EndKey, StartKey};
use bigtable_rs::google::bigtable::v2::{ReadRowsRequest, RowFilter, RowRange, RowSet};
use env_logger;
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let project_id = "project-id";
    let instance_name = "test1";
    let table_name = "my-table";
    let channel_size = 4;
    let timeout = Duration::from_secs(10);

    let key_start: String = "key1".to_owned();
    let key_end: String = "key3".to_owned();

    // make a bigtable client
    let connection = bigtable::BigTableConnection::new(
        project_id,
        instance_name,
        true,
        channel_size,
        Some(timeout),
    )
    .await?;
    let mut bigtable = connection.client();

    // prepare a ReadRowsRequest
    let request = ReadRowsRequest {
        table_name: format!("{}{}", bigtable.table_prefix, table_name),
        rows_limit: 10,
        rows: Some(RowSet {
            row_keys: vec![], // use this field to put keys for reading specific rows
            row_ranges: vec![RowRange {
                start_key: Some(StartKey::StartKeyClosed(key_start.into_bytes())),
                end_key: Some(EndKey::EndKeyOpen(key_end.into_bytes())),
            }],
        }),
        filter: Some(RowFilter {
            filter: Some(Filter::Chain(Chain {
                filters: vec![
                    RowFilter {
                        filter: Some(Filter::FamilyNameRegexFilter("cf1".to_owned())),
                    },
                    RowFilter {
                        filter: Some(Filter::ColumnQualifierRegexFilter("c1".as_bytes().to_vec())),
                    },
                    RowFilter {
                        filter: Some(Filter::CellsPerColumnLimitFilter(1)),
                    },
                ],
            })),
        }),
        ..ReadRowsRequest::default()
    };

    // calling bigtable API to get results
    let response = bigtable.read_rows(request).await?;

    // simply print results for example usage
    response.into_iter().for_each(|(key, data)| {
        data.into_iter().for_each(|(cell_name, cell_value)| {
            println!(
                "{} -> {}:{}",
                String::from_utf8(key.clone()).unwrap(),
                String::from_utf8(cell_name).unwrap(),
                String::from_utf8(cell_value).unwrap()
            )
        })
    });

    Ok(())
}
