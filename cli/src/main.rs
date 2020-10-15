#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    // CLI definition...
    let matches = App::new(crate_name!())
                        .version(crate_version!())
                        .author(crate_authors!())
                        .about("Retrieve bioinformatics data using REST")
                        .subcommand(SubCommand::with_name("search")
                                    .about("Searches the specified id")
                                    .arg(Arg::with_name("id")
                                        .help("Bioinformatic attribute ID, i.e: chr1")
                                        .required(true))
                                    .arg(Arg::with_name("location")
                                        .help("Object location of the index, i.e: s3://bucket/test.bam.bai")
                                        .required(true))
                                    )
                        .get_matches();


    match matches.subcommand() {
        ("search", Some(_search_args)) => {
            dbg!("Mockito?");
        },
        ("", None)   => println!("{}", matches.usage()),
        _            => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}

use httpmock::Method::{GET};
use httpmock::{Mock, MockServer};

#[test]
fn example_test() {
    // Start a local mock server for exclusive use by this test function.
    let mock_server = MockServer::start();

    // Create a mock on the mock server. The mock will return HTTP status code 200 whenever
    // the mock server receives a GET-request with path "/hello".
    let search_mock = Mock::new()
        .expect_method(GET)
        .expect_path("/hello")
        .return_status(200)
        .create_on(&mock_server);

    // Send an HTTP request to the mock server. This simulates your code.
    // The mock_server variable is being used to generate a mock server URL for path "/hello".
    let response = reqwest::get(&mock_server.url("/hello")).await?;

    // Ensure the mock server did respond as specified above.
    assert_eq!(response.status(), 200);
    // Ensure the specified mock responded exactly one time.
    assert_eq!(search_mock.times_called(), 1);
}