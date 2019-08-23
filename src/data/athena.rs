use std::error::Error;
use futures::Future;

// Rusoto
use rusoto_athena::*;
use rusoto_core::{Region, RusotoFuture, RusotoError};

use crate::data;
use crate::data::{ReadsRef, ReadsIndex};

pub struct AthenaStore {
  client: AthenaClient,
  database: String,
  results_bucket: String,
}

impl AthenaStore {
  pub fn new(region: Region, database: String, results_bucket: String) -> AthenaStore {
    let client = AthenaClient::new(region);
    AthenaStore {
      client,
      database,
      results_bucket,
    }
  }
}

impl ReadsIndex for AthenaStore {
  fn find_by_id(&self, id: String) -> Result<Vec<ReadsRef>, Box<dyn Error>> {

    // let request_token = Uuid::new_v4();
    // let query_input = StartQueryExecutionInput {
    //   client_request_token: Some(request_token.to_string()),
    //   query_string: query,
    //   query_execution_context: Some(QueryExecutionContext {
    //     database: Some(self.database)
    //   }),
    //   result_configuration: Some(ResultConfiguration {
    //     encryption_configuration: None,
    //     output_location: Some(self.results_bucket),
    //   }),
    //   work_group: Default::default()
    // };
    // client.start_query_execution(query_input)
    //   .then(|start_query_output| {
    //     start_query_output.query_execution_id
    //       .map(|query_execution_id| {
    //         // TODO start next query --> Future
    //       })
    //       .unwrap_or(|| {
    //         future::err::<>("Error")
    //       })
    //     // ... Future()
    //   });

    unimplemented!();
  }
}
