#[macro_use]
extern crate clap;
#[macro_use]
extern crate dotenv_codegen;

mod data;

use clap::{App, Arg, SubCommand, ArgMatches};

use rusoto_core::Region;

use crate::data::ReadsIndex;
use crate::data::ReadsRequest;


fn htsget_index(location: &str, _store: &str) {
    println!("Locally indexing file: {}", location)
}

fn htsget_search<I>(reads_index: I, args: &ArgMatches)
  where I: ReadsIndex {

    let id = args.value_of("id").unwrap().to_string();

    let igvjs_htsget_example = ReadsRequest {
        url: "http://htsget.umccr.org/v1".to_string(),
        id: "BroadHiSeqX_b37/NA12878".to_string(),
        chromosome: "11".to_string(),
        start: 5011963,
        end: 5012660,
    };

    println!("Searching {:#?}: ", igvjs_htsget_example);

    let reads_refs = reads_index
        .find_by_id(igvjs_htsget_example);
    
    for reads_ref in reads_refs.into_iter() {
        println!("{:?}", reads_ref);
    }
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
                                        .required(true)))
                        .get_matches();


    match matches.subcommand() {
        ("search", Some(args)) => {
            htsget_search(store, args)
        },
        ("", None)   => println!("{}", matches.usage()),
        _            => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}