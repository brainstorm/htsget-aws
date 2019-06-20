#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let _matches = App::new(crate_name!())
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
}