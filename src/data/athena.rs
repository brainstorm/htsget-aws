use uuid::Uuid;

#[macro_use]
use dotenv_codegen;

// Rusoto
use rusoto_athena::*;
use rusoto_core::Region;

use crate::data::{ReadsRef, ReadsIndex};
use crate::data::errors::{Error, Result};

pub struct AthenaStore {
  client: AthenaClient,
  database: String,
  results_bucket: String,
  request_token: Uuid
}

impl AthenaStore {
  pub fn new(region: Region, database: String, results_bucket: String) -> AthenaStore {
    let client = AthenaClient::new(region);
    let request_token = Uuid::new_v4();
    return AthenaStore {
              client,
              database,
              results_bucket,
              request_token
          }
  }
}

impl ReadsIndex for AthenaStore {
  fn find_by_id(&self, id: String) -> Result<Vec<ReadsRef>, Error> {
    let store = AthenaStore::new(Region::ApSoutheast2,
                                 dotenv_codegen::dotenv!("AWS_ATHENA_DB").to_string(), 
                                 dotenv_codegen::dotenv!("AWS_ATHENA_RESULTS_OUTPUT_BUCKET").to_string());

    let query_input = StartQueryExecutionInput {
        client_request_token: Some(store.request_token.to_string()),
        query_execution_context: Some(QueryExecutionContext {
            database: Some(dotenv_codegen::dotenv!("AWS_ATHENA_DB").to_string())
        }),
        result_configuration: Some(ResultConfiguration {
            encryption_configuration: None,
            output_location: Some(dotenv_codegen::dotenv!("AWS_ATHENA_RESULTS_OUTPUT_BUCKET").to_string())
        }),
        work_group: Default::default(),
        //XXX: query_string: id
        query_string: "SELECT referencename FROM htsget.adam WHERE referencename LIKE 'chr1';".to_string()
    };

    let query_id = store.client.start_query_execution(query_input).sync()
        //XXX: Figure out why source: is not an implicit snafu parameter
            .map_err(|_| Error::NoResults )
            .and_then(|output| {
                output.query_execution_id
                    .map(|query_id| vec!(ReadsRef{ url: query_id, range: 1..2 }))
                    .ok_or(Error::NoResults)
            });

    return query_id;
  }
}