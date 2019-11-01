use std::path::Path;
//use std::collections::HashMap;
use std::collections::BTreeMap;

use rust_htslib::bam::{Reader, Record, Read};

pub fn seek_voffset() {
    let mut bam = Reader::from_path(&Path::new("tests/data/mt.bam")).ok().expect("Error opening file.");

    //let mut names_by_voffset = HashMap::new();
    let mut names_by_voffset = BTreeMap::new();

    let mut offset = bam.tell();
    let mut rec = Record::new();
    loop {
        if !bam.read(&mut rec).expect("error reading bam") {
            break;
        }

        let pos = rec.pos();
        println!("{} {}", pos, offset);
        names_by_voffset.insert(pos, offset);
        offset = bam.tell();
    }

    for (pos, offset) in names_by_voffset.iter() {
        println!("{} {}", pos, offset);
        bam.seek(*offset).unwrap();
        bam.read(&mut rec).unwrap();
        let rec_pos = rec.pos();
    }
}

