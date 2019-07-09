#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};

fn htsget_index(location: String) {
    println!("Indexing file: {}", location)
}

fn htsget_search(id: String) {
    println!("Let's search: {}", id)
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

    // ... and some argument action!
    match matches.subcommand() {
        ("index", Some(index_matches)) => {
            htsget_index(index_matches.value_of("location").unwrap().to_string());
        },
        ("search", Some(search_matches)) => {
            htsget_search(search_matches.value_of("id").unwrap().to_string());
        },
        ("", None)   => println!("{}", matches.usage()),
        _            => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}