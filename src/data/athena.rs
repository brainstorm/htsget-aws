use uuid::Uuid;

#[macro_use]
use dotenv_codegen;

// Rusoto
// XXX: More specific imports
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

    let query_token = store.client.start_query_execution(query_input).sync()
        .map_err(|error| Error::ReadsQueryError { cause: format!("{:?}", error) })
        .and_then(|output| {
            output.query_execution_id
                .ok_or(Error::ReadsQueryError { cause: "No reads found 1".to_string() })
        })?;

    // XXX: Check AThena rusoto for wait-like methods
    // XXX: Timeout
    wait_query_execution(store.client, query_token)?;

    let mut refs = Vec::new();
    println!("{:?}", query_token.to_string());
    let mut token: Option<String> = Some(query_token);
    while token.is_some() {
      let next_token = token.unwrap();
      let query_results_input = GetQueryResultsInput {
        query_execution_id: next_token.clone(),
        max_results: None,
        next_token: Some(next_token),
      };
      
      let query_results_x = store.client.get_query_results(query_results_input).sync()
        .map_err(|err| Error::ReadsQueryError { cause: format!("No reads found: {:?}", err) });

      println!("{:?}", query_results_x);
      let query_results = query_results_x?;
      
      println!("{:?}", query_results.result_set);
      // TODO query_results.result_set -> Vec<ReadsRef>
      let reads_batch = Vec::<ReadsRef>::new();
      
      refs.extend(reads_batch.into_iter());
      token = query_results.next_token;
    }
    Ok(refs)
  }
}