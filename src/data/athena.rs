use uuid::Uuid;
use std::{thread, time};

//#[macro_use]
use dotenv_codegen;

// Rusoto
use rusoto_athena::*;
use rusoto_core::{Region};

use crate::data::{ReadsRef, ReadsRefHeaders, ReadsIndex};
use crate::data::errors::{Error, Result};
use crate::data::IgvParametersRequest;


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

fn extract_reads(result_set: &ResultSet) -> Option<Vec<ReadsRef>> {
  let read_refs: Vec<ReadsRef> = result_set.rows.iter()
      .flat_map(|rows| rows.into_iter())
      .flat_map(|row| extract_row(row).into_iter())
      .collect();
  
  Some(read_refs)
}

//XXX: baseurl parameter
fn extract_row(row: &Row) -> Option<ReadsRef> {
  row.data.as_ref()
    .and_then(|cols| {
      cols[0].var_char_value.as_ref()
        .and_then(|ref_name| {
          cols[1].var_char_value.as_ref()
            .map(|cigar| (ref_name, cigar))
        })
    })
    .map(|(_ref_name, _cigar)| {
      let url = "XXX".to_string();
      let headers = ReadsRefHeaders {
        authorization: "Bearer all_good_for_now".to_string(),
        range: "bytes=XXX".to_string() //XXX: translation between input coords and bytes
      };

      ReadsRef::new(url, "body".to_string(), headers)
    })
}

fn igv_to_sql(query_json: IgvParametersRequest) -> String {
    let start = query_json.start;
    let end = query_json.end;
    let chromosome = query_json.chromosome;

    return format!("SELECT referencename, start, \"end\" \
                    FROM htsget.umccr_htsget_dev \
                    WHERE start >= {} AND \"end\" <= {} AND referencename = '{}' LIMIT 10;"
                    , start, end, chromosome);
}

impl ReadsIndex for AthenaStore {
  fn find_by_id(&self, id: IgvParametersRequest) -> Result<Vec<ReadsRef>, Error> {
    let store = AthenaStore::new(Region::ApSoutheast2,
                                 dotenv_codegen::dotenv!("AWS_ATHENA_DB").to_string(), 
                                 dotenv_codegen::dotenv!("AWS_ATHENA_RESULTS_OUTPUT_BUCKET").to_string());

    let query_input = StartQueryExecutionInput {
        client_request_token: Some(store.request_token.to_string()),
        query_execution_context: Some(QueryExecutionContext {
            database: Some(dotenv_codegen::dotenv!("AWS_ATHENA_DB").to_string())
        }),
        result_configuration: Some(ResultConfiguration {
            encryption_configuration: Default::default(),
            output_location: Some(dotenv_codegen::dotenv!("AWS_ATHENA_RESULTS_OUTPUT_BUCKET").to_string())
        }),
        work_group: Default::default(),
        query_string: igv_to_sql(id),
    };

    let query = store.client.start_query_execution(query_input); 
    let query_exec_id = query.sync()
        .map_err(|error| Error::ReadsQueryError { cause: format!("{:?}", error) })
        .and_then(|output| {
            output.query_execution_id
                .ok_or(Error::ReadsQueryError { cause: "No reads found 1".to_string() })
        })?;

    // XXX: Handle timeouts better
    wait_for_results(&store.client, &query_exec_id);

    let refs = Vec::new();
    let query_results_input = GetQueryResultsInput {
      query_execution_id: query_exec_id,
      max_results: Default::default(),
      next_token: Default::default()
    };
    
    let query_results = store.client.get_query_results(query_results_input).sync()
      .map_err(|err| Error::ReadsQueryError { cause: format!("No reads found: {:?}", err) });
    
    let reads = query_results.map(|res| {
      res.result_set.as_ref()
        .and_then(extract_reads)
        .ok_or(Error::ReadsQueryError { cause: "No metadata found".to_string() })
    });

    dbg!(reads);
    Ok(refs)
  }
}