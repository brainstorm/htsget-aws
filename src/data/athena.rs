use uuid::Uuid;
use std::{thread, time};

#[macro_use]
use dotenv_codegen;

// Rusoto
// XXX: More specific imports
use rusoto_athena::*;
use rusoto_core::{Region};

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

fn wait_for_results(client: &AthenaClient, token: &String) -> Result <(), Error> {
  let mut succeed = false;
  let start_time = time::Instant::now();

  while !succeed && time::Instant::now().duration_since(start_time).as_secs() < 10 {
    let query_in = GetQueryExecutionInput { query_execution_id: token.clone() };
    let state = client.get_query_execution(query_in).sync()
      .map_err(|error| Error::ReadsQueryError { cause: format!("{:?}", error) })
      .map(|output| {
        output.query_execution
          .and_then(|query_exec| query_exec.status)
          .and_then(|status| status.state)
      })?;

    succeed = state.map(|status| status == "SUCCEED").unwrap_or(false);
      // XXX: If failure states do not wait
    if !succeed {
      // Wish Rusoto's async was in better shape :_/
      let one_second = time::Duration::from_secs(1);
      thread::sleep(one_second);
    }
  }

  if succeed {
    Ok(())
  }
  else {
    Err(Error::ReadsQueryError { cause: "Timeout waiting for the query result".to_string() })
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
        // XXX: query_string: id
        // XXX: see how igv.js implements htsget
        query_string: "SELECT referencename, cigar FROM htsget.umccr_htsget_dev WHERE referencename = 'chr1';".to_string()
    };

    let query = store.client.start_query_execution(query_input); 
    let query_exec_id = query.sync()
        .map_err(|error| Error::ReadsQueryError { cause: format!("{:?}", error) })
        .and_then(|output| {
            output.query_execution_id
                .ok_or(Error::ReadsQueryError { cause: "No reads found 1".to_string() })
        })?;

    // XXX: Handle timeouts better
    wait_for_results(&store.client, &query_exec_id)?;

    let refs = Vec::new();
    let query_results_input = GetQueryResultsInput {
      query_execution_id: query_exec_id,
      max_results: None,
      next_token: Default::default()
    };
    
    let query_results = store.client.get_query_results(query_results_input).sync()
      .map_err(|err| Error::ReadsQueryError { cause: format!("No reads found: {:?}", err) });
    
    let meta = query_results.map(|res| { 
      res.result_set
        //.and_then(|col_info| col_info.column_info)
        .and_then(|col| col.result_set_metadata )
        .and_then(|col| col.column_info )
        .ok_or(Error::ReadsQueryError { cause: "No metadata found".to_string() })
    })?;
    
    //let res = meta.unwrap();
    dbg!(meta)?;

    //dbg!(meta);
    //println!("{:#?}", rows);
    // TODO query_results.result_set -> Vec<ReadsRef>
    // let reads_batch = Vec::<ReadsRef>::new();
    
    // refs.extend(reads_batch.into_iter());
    // token = query_results.next_token;
    // }
    Ok(refs)
  }
}