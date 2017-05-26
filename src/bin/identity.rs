extern crate rgmk;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Cursor};

fn main() {
    let path = std::env::args().nth(1).expect("Expected path as argument");
    let original = {
        let mut file = File::open(&path).unwrap();
        let mut vec = Vec::new();
        file.read_to_end(&mut vec).unwrap();
        vec
    };
    let game_data = rgmk::GameData::from_reader(&mut BufReader::new(Cursor::new(&original[..])))
        .unwrap_or_else(|e| panic!("Failed to read: {}", e));
    let mut new: Vec<u8> = vec![0; original.len()];
    game_data
        .write_to_writer(&mut BufWriter::new(Cursor::new(&mut new[..])))
        .unwrap();
    for (i, (o, n)) in original[..].iter().zip(new[..].iter()).enumerate() {
        if o != n {
            panic!("Difference at offset {}. orig {} vs new {}", i, o, n);
        }
    }
    eprintln!("Identity test successful for {}", path);
}
