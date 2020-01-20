#[macro_use]
extern crate clap;

mod data;

use clap::{App, Arg, SubCommand};
use htsget::data::{ ReadsRequest, ReadsResponse };

pub fn htsget_search(_req: ReadsRequest) -> ReadsResponse {
    // XXX: Should just invoke lambda on bin
    unimplemented!();
}

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
            let igvjs_example = ReadsRequest {
                url: "http://htsget.umccr.org/v1".to_string(),
                id: "foo".to_string(),
                chromosome: "11".to_string(),
                start: 4999976,
                end: 5002147,
            };

            htsget_search(igvjs_example);
        },
        ("", None)   => println!("{}", matches.usage()),
        _            => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}