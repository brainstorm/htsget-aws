#[cfg(test)]
mod tests {
    use httpmock::Method::{GET};
    use httpmock::{MockServer};
    use isahc;

    #[test]
    fn htsget_spec_request() {
        // Start a local mock server for exclusive use by this test function.
        let server = MockServer::start();

        let search_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/reads/NA12878");
            then.status(200)
                .header("Content-Type", "application/vnd.ga4gh.htsget.v1.0.0+json; charset=utf-8");
                
        });

        // GET the response...
        let response = isahc::get(&server.url("/reads/NA12878")).unwrap();
        
        // Ensure the specified mock was called exactly one time.
        search_mock.assert();
        // Ensure the mock server did respond as specified above.
        assert_eq!(response.status(), 200);
        // assert_eq!(response.text().unwrap(), 
        //            response.body_from_file("../tests/rest/htsget_spec_response.json"));
    }
}