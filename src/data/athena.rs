use std::error::Error;
use uuid::Uuid;

#[macro_use]
use dotenv_codegen;

// Rusoto
use rusoto_athena::*;
use rusoto_core::Region;

use crate::data::{ReadsRef, ReadsIndex};

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
  fn find_by_id(&self, id: String) -> Result<Vec<ReadsRef>, Box<dyn Error>> {
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
        query_string: id
    };

    match store.client.start_query_execution(query_input).sync() {
        Ok(output) => {
            match output.query_execution_id {
                Some(query_id) => (),
                None => println!("No dice")
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        },
    }
    Ok(vec!())
    }
}