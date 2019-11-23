use uuid::Uuid;
use std::{thread, time};

//#[macro_use]
use dotenv_codegen;

// Rusoto
use rusoto_athena::*;
use rusoto_core::{Region};

use crate::data::{ReadsRef, ReadsRefHeaders, ReadsIndex};
use crate::data::errors::{Error, Result};
use crate::data::ReadsRequest;

const TIMEOUT:u64 = 10;

enum QueryState {
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
    Unknown,
}

impl QueryState {
    pub fn from_string(name: String) -> Self {
        match name.as_str() {
            "QUEUED" => QueryState::Queued,
            "RUNNING" => QueryState::Running,
            "SUCCEEDED" => QueryState::Succeeded,
            "FAILED" => QueryState::Failed,
            "CANCELLED" => QueryState::Cancelled,
            _ => QueryState::Unknown,
        }
    }
}

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
  fn find_by_id(&self, id: ReadsRequest) -> Result<Vec<ReadsRef>, Error> {
    let query_input = self.build_query_execution_input(id);

    let query = self.client.start_query_execution(query_input);
    let query_exec_id = query.sync()
        .map_err(|error| Error::ReadsQueryError { cause: format!("{:?}", error) })
        .and_then(|output| output.query_execution_id
            .ok_or(Error::ReadsQueryError { cause: "No reads found".to_string() }))?;

      //XXX: Should handle errors, ? eats ReadsRef results for breakfast :/
    Self::wait_for_results(&self.client, &query_exec_id);

    let refs = Vec::new();
    let query_results_input = GetQueryResultsInput {
      query_execution_id: query_exec_id,
      .. GetQueryResultsInput::default()
    };
    
    let query_results = self.client.get_query_results(query_results_input).sync()
      .map_err(|err| Error::ReadsQueryError { cause: format!("No reads found: {:?}", err) });
    
    let reads = query_results.map(|res| {
      res.result_set.as_ref()
        .and_then(Self::extract_reads)
        .ok_or(Error::ReadsQueryError { cause: "No metadata found".to_string() })
    });

    dbg!(reads);
    Ok(refs)
  }
}

impl AthenaStore {
  fn build_query_execution_input(&self, id: ReadsRequest) -> StartQueryExecutionInput {
    StartQueryExecutionInput {
      client_request_token: Some(self.request_token.to_string()),
      query_execution_context: Some(QueryExecutionContext {
        database: Some(self.database.clone())
      }),
      result_configuration: Some(ResultConfiguration {
        encryption_configuration: Default::default(),
        output_location: Some(self.results_bucket.clone())
      }),
      work_group: Default::default(),
      query_string: Self::igv_to_sql(id),
    }
  }

  fn query_done(maybe_state: Option<QueryState>, start_time: time::Instant) -> bool {
    let one_second = time::Duration::from_secs(1);

    if time::Instant::now().duration_since(start_time).as_secs() < TIMEOUT {
      false
    } else {
      maybe_state.map(|state| match state {
        QueryState::Succeeded => true,
        QueryState::Cancelled | QueryState::Failed | QueryState::Unknown => false,
        QueryState::Queued    | QueryState::Running => { thread::sleep(one_second); false },
      }).unwrap_or(true)
    }
  }

  fn wait_for_results(client: &AthenaClient, token: &String) -> Result <(), Error> {
    let success = false;
    let start_time = time::Instant::now();

    let mut maybe_state = None;

    while !Self::query_done(maybe_state, start_time) {
      let query_in = GetQueryExecutionInput { query_execution_id: token.clone() };
      maybe_state = client.get_query_execution(query_in).sync()
          .map_err(|error| Error::ReadsQueryError { cause: format!("{:?}", error) })
          .map(|output| {
            output.query_execution
                .and_then(|query_exec| query_exec.status)
                .and_then(|status| status.state.map(QueryState::from_string))
          })?;
    }

    if success {
      Ok(())
    } else {
      Err(Error::ReadsQueryError { cause: "Timeout waiting for the query result".to_string() })
    }
  }

  fn extract_reads(result_set: &ResultSet) -> Option<Vec<ReadsRef>> {
    let read_refs: Vec<ReadsRef> = result_set.rows.iter()
        .flat_map(|rows| rows.into_iter())
        .flat_map(|row| Self::extract_row(row).into_iter())
        .collect();

    Some(read_refs)
  }

  //XXX: baseurl parameter
  fn extract_row(row: &Row) -> Option<ReadsRef> {
    row.data.as_ref()
        .and_then(|cols| {
          cols[0].var_char_value.as_ref()
              .and_then(|coffset_start| {
                cols[1].var_char_value.as_ref()
                    .map(|coffset_end| (coffset_start, coffset_end))
              })
        })
        .map(|(coffset_start, coffset_end)| {
          let url = "XXX".to_string();
          let headers = ReadsRefHeaders {
            authorization: "Bearer all_good_for_now".to_string(),
            range: format!("bytes={}..{}", coffset_start, coffset_end)
          };

          ReadsRef::new(url, "body".to_string(), headers)
        })
  }

  fn igv_to_sql(query_json: ReadsRequest) -> String {
    let start = query_json.start;
    let end = query_json.end;
    let chromosome = query_json.chromosome;

      //XXX: Reasonable types for target_name et al
    dbg!(format!("SELECT coffset_start, coffset_end \
                    FROM htsget.csv \
                    WHERE seq_start <= {} AND seq_end >= {} AND target_name = {};"
                   , end, start, chromosome))
  }
}
