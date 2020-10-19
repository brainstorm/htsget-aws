use clap::{App, Arg, SubCommand};
use clap::{crate_name, crate_version, crate_authors};

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
