#[macro_use]
extern crate clap;
#[macro_use]
extern crate dotenv_codegen;

mod data;

use clap::{App, Arg, SubCommand, ArgMatches};

use rusoto_core::Region;

use crate::data::athena::AthenaStore;
use crate::data::ReadsIndex;
use crate::data::IgvParametersRequest;


fn htsget_index(location: String) {
    println!("Indexing file: {}", location)
}

fn htsget_search<I>(reads_index: I, args: &ArgMatches)
  where I: ReadsIndex {

    let id = args.value_of("id").unwrap().to_string();

//    let igvjs_htsget_example = igvParametersRequest {
//        url: "http://htsnexus.rnd.dnanex.us/v1",
//        id: "BroadHiSeqX_b37/NA12878",
//        chromosome: "chr1",
//        start: 10000,
//        end: 10100
//    };

    let igvjs_htsget_example = IgvParametersRequest {
        url: "http://htsget.umccr.org/v1".to_string(),
        id: "BroadHiSeqX_b37/NA12878".to_string(),
        chromosome: "chr1".to_string(),
        start: 10000,
        end: 10100
    };


    //dbg!(igvjs_htsget_example)
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
                        .subcommand(SubCommand::with_name("index")
                                    .about("Indexes an object sitting on object storage location")
                                    .arg(Arg::with_name("location")
                                    .help("Store object location, i.e: s3://bucket/key.bam")
                                    .required(true)))
                        .subcommand(SubCommand::with_name("search")
                                    .about("Searches the specified id")
                                    .arg(Arg::with_name("id")
                                    .help("Bioinformatic attribute ID, i.e: chr1")
                                    .required(true)))
                        .get_matches();

    //let region = dotenv!("AWS_REGION").to_string();
    let region = Region::default();
    let database = dotenv!("AWS_ATHENA_DB").to_string();
    let results_bucket = dotenv!("AWS_ATHENA_RESULTS_OUTPUT_BUCKET").to_string();
    // Connect to Athena on AWS
    let store = AthenaStore::new(region, database, results_bucket);

    // ... and some argument action!
    match matches.subcommand() {
        ("index", Some(index_matches)) => {
            htsget_index(index_matches.value_of("location").unwrap().to_string());
        },
        ("search", Some(args)) => htsget_search(store, args),
        ("", None)   => println!("{}", matches.usage()),
        _            => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}