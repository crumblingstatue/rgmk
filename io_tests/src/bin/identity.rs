extern crate rgmk;
extern crate byteorder;
extern crate env_logger;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Cursor};

fn main() {
    env_logger::init().unwrap();
    let mut args = std::env::args().skip(1);
    let path = args.next().expect("Expected path as argument");
    let original = {
        let mut file = File::open(&path).unwrap();
        let mut vec = Vec::new();
        file.read_to_end(&mut vec).unwrap();
        vec
    };
    let game_data = rgmk::GameData::from_reader(&mut BufReader::new(Cursor::new(&original[..])))
                        .unwrap_or_else(|e| panic!("Failed to read: {}", e));
    let mut new: Vec<u8> = vec![0; original.len()];
    game_data.write_to_writer(&mut BufWriter::new(Cursor::new(&mut new[..]))).unwrap();
    for (i, (o, n)) in original[..].iter().zip(new[..].iter()).enumerate() {
        if o != n {
            use byteorder::{ReadBytesExt, LittleEndian};
            let orig = (&original[i..i + 4]).read_u32::<LittleEndian>().unwrap();
            let new = (&new[i..i + 4]).read_u32::<LittleEndian>().unwrap();
            panic!("Difference at offset {}. orig {} vs new {}", i, orig, new);
        }
    }
    eprintln!("Identity test successful for {}", path);
}
