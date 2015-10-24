extern crate rgmk;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

fn main() {
    let original = {
        let mut args = std::env::args().skip(1);
        let path = args.next().expect("Expected path as argument");
        let mut file = File::open(&path).unwrap();
        let mut vec = Vec::new();
        file.read_to_end(&mut vec).unwrap();
        vec
    };
    let root = rgmk::read_chunk(&mut BufReader::new(&original[..])).unwrap();
    let mut new: Vec<u8> = Vec::with_capacity(original.len());
    unsafe {
        new.set_len(original.len());
    }
    rgmk::write_chunk(&mut BufWriter::new(&mut new[..]), &root).unwrap();
    assert_eq!(original, new);
}
