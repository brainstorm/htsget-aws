// AWS Lambda Rust runtime
use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;
use uuid::Uuid;

// DotEnv
#[macro_use]
extern crate dotenv_codegen;

// Rusoto
use rusoto_athena::*;
use rusoto_core::{Region};

fn main() {
    // Init env logger for debugging: https://www.rusoto.org/debugging.html
    let _ = env_logger::try_init();

    lambda!(handler);
}

fn athena_query(query: String) {
//    let client = AthenaClient::new(Default::default());
// XXX: Region out of here
    let client = AthenaClient::new(Region::ApSoutheast2);
    let request_token = Uuid::new_v4();

    let query_input = StartQueryExecutionInput {
        client_request_token: Some(request_token.to_string()),
        query_string: query,
        query_execution_context: Some(QueryExecutionContext {
            database: Some(dotenv!("AWS_ATHENA_DB").to_string())
        }),
        result_configuration: Some(ResultConfiguration {
            encryption_configuration: None,
            output_location: Some(dotenv!("AWS_ATHENA_RESULTS_OUTPUT_BUCKET").to_string())
        }),
        work_group: Default::default()
    };

    match client.start_query_execution(query_input).sync() {
        Ok(output) => {
            match output.query_execution_id {
                Some(query_id) => process_athena_results(client, request_token, query_id),
                None => println!("query running. no id found"),
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        },
    }
}

fn process_athena_results(client: AthenaClient, query_token: uuid::Uuid, query_id: String){
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

fn http_request_to_athena_query(uri_id: String) -> String {
    // A string identifying which records to return.
    
    // The format of this identifier is left to the discretion of the 
    // API provider, including allowing embedded “/” characters. 
    // The following would be valid identifiers:
    
    // ReadGroupSetIds or VariantSetIds as defined by the GA4GH API
    // Studies: PRJEB4019 or /byStudy/PRJEB4019
    // Samples: NA12878 or /data/platinum/NA12878
    // Runs: ERR148333 or /byRun/ERR148333

    let split = uri_id.split("/");
    let mut path_parts: Vec<&str> = split.collect();

    // XXX: only return last element for now, be more clever afterwards?
    let id = path_parts.pop().unwrap();

    // XXX: Parametrize for start/end and many others according to spec and backend schema
    //let sql_query = "SELECT referencename FROM htsget.adam WHERE referencename LIKE 'chr1';".to_string(); //XXX: id should be interpolated here
    let sql_query = "SELECT referencename FROM htsget.adam WHERE referencename LIKE 'chr1';".to_string(); //XXX: id should be interpolated here

    println!("{}: {}", id, sql_query);

    return sql_query;
}

fn handler(
    req: Request,
    _ctx: Context,
) -> Result<impl IntoResponse, HandlerError> {

    let query = http_request_to_athena_query(req.uri().path().to_string());
    athena_query(query);

    Ok(json!({
        "message": "Reads: Your function executed successfully!"
    }))
}
