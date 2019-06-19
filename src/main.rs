#[macro_use]
extern crate clap;

use clap::{App, SubCommand};

fn main() {
    let _matches = App::new(crate_name!())
                        .version(crate_version!())
                        .author(crate_authors!())
                        .get_matches();
}