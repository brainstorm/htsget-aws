use std::path::Path;
use std::collections::HashMap;

use rust_htslib::bam::{Reader, Record, Read};

struct  Voffsets {
    coffset: i32,
    uoffset: i32,
}

pub fn seek_voffset() {
    let mut bam = Reader::from_path(&Path::new("tests/data/mt.bam")).ok().expect("Error opening file.");

    let mut voffset = Voffsets{ coffset: 0, uoffset: 0 };
    let mut pos_voffset = HashMap::new();

    let mut offset;

    let mut rec = Record::new();
    loop {
        if !bam.read(&mut rec).expect("error reading bam") { break; }

        // Retrieve virtual offset
        offset = bam.tell();
        // Get compressed and uncompressed indexes from virtual offset
        let mut coffset = offset.checked_shr(16);
        let mut uoffset = (offset ^ coffset.checked_shl(16)) as i32;
        voffset = Voffsets { coffset, uoffset };

        pos_voffset.insert(rec.pos(), &voffset);
    }

    for (pos, voffsets) in pos_voffset {
        bam.seek(offset).unwrap();
        bam.read(&mut rec).unwrap();
    }
}

