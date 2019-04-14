// use lambda_http::{IntoResponse, Request};
// use lambda_runtime::{Context};
// use serde_json::json;

// #[test]
// fn handler_handles() {
//     let request = Request::default();
//     let expected = json!({
//     "message": "Go Serverless v1.0! Your function executed successfully!"
//     })
//     .into_response();
//     let response = handler(request, Context::default())
//         .expect("expected Ok(_) value")
//         .into_response();
//     assert_eq!(response.body(), expected.body())
// }