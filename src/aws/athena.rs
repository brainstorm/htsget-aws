// A string identifying which records to return.

// The format of this identifier is left to the discretion of the 
// API provider, including allowing embedded “/” characters. 
// The following would be valid identifiers:

// ReadGroupSetIds or VariantSetIds as defined by the GA4GH API
// Studies: PRJEB4019 or /byStudy/PRJEB4019
// Samples: NA12878 or /data/platinum/NA12878
// Runs: ERR148333 or /byRun/ERR148333

// Futures/Async
#![allow(unused_imports)]
use futures::compat::Future01CompatExt as _;

extern crate futures;
extern crate tokio_core;

//use futures::future::Future;
use tokio_core::reactor::Core;

// AWS
use rusoto_athena::*;
use rusoto_core::{Region, RusotoError, RusotoFuture};
use uuid::Uuid;

// Request token must be generated and passed, unlike
// AWS Java SDK which does this implicitly.
struct AthenaQueryFuture {
    inner: RusotoFuture<StartQueryExecutionOutput, StartQueryExecutionError>,
    token: String,
    query_id: String,
}

fn athena_start_query(client: &AthenaClient, query: String) -> AthenaQueryFuture {
        let request_token = Uuid::new_v4().to_string();
        let athena_query_attrs = StartQueryExecutionInput {
            client_request_token: Some(request_token),
            query_string: query,
            result_configuration: Some(ResultConfiguration {
                   encryption_configuration: None,
                   output_location: Some(dotenv!("AWS_ATHENA_RESULTS_OUTPUT_BUCKET").to_string())
            }),
            query_execution_context: Some(QueryExecutionContext {
               database: Some(dotenv!("AWS_ATHENA_DB").to_string()),
            }),
            work_group: Default::default()
        };

        AthenaQueryFuture {
            inner: client.start_query_execution(athena_query_attrs),
            token: request_token,
            query_id: query
        }
    }

pub fn query_athena(query: String) -> String {
// XXX: Region out of here
// I wish "let client = AthenaClient::new(Default::default());" could derive current
// region from AWS credentials?
// https://github.com/rusoto/rusoto/blob/master/rusoto/services/athena/src/generated.rs#L1718
    let client = AthenaClient::new(Region::ApSoutheast2);
    let mut core = Core::new().unwrap();

    let AthenaQueryFuture { inner: query_future, token, query_id } = athena_start_query(&client, query);
    let get_results = athena_get_query_results(&client, token, query_id);
    let parse_results = athena_parse_query_results();

    let chained_futures = 
         query_future
        .and_then(|_| get_results)
        .and_then(|_| parse_results);

    let results_from_athena = match core.run(chained_futures) {
        Ok(resultset) => resultset,
        Err(e) => panic!("Error completing Athena query: {}", e),
    };

    println!("Athena resultset: {:?}", results_from_athena);
    return results_from_athena.unwrap()
}

fn athena_get_query_results(client: &AthenaClient, query_token: uuid::Uuid, query_id: String){
    println!("query running. id: {} with query token: {}", query_id, query_token.to_string());

    match client.get_query_results(GetQueryResultsInput {
        max_results: Default::default(),
        query_execution_id: query_id,
        next_token: Some(query_token.to_string())
    }).sync() {
        Ok(output) => {
            match output.result_set {
                Some(resultset) => println!("woot: {:#?}", resultset),
                None => println!("nooo"),
            }
        },
        Err(error) => {
            println!("no resultset for you: {:#?}", error)
        },
    }
}

fn athena_parse_query_results() {
    unimplemented!();
}

async fn athena_query_async() -> Result<(), Error> {
    let client = AthenaClient::new(Region::ApSoutheast2);
    client.start_query_execution(input: StartQueryExecutionInput)
}