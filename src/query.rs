use crate::aws::athena::query_athena;

#[allow(dead_code)]
fn query_storage() -> String {
    unimplemented!()
}

fn query_db(id: String) -> String {
    return query_athena(id);
}

// Database interface fronted by APIGW/REST endpoint
// fn query_rest(id: String) -> String {
//     let query = apigw_request(req.uri().path().to_string());
// }