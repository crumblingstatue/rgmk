extern crate rgmk;
extern crate env_logger;

use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;

fn main() {
    env_logger::init().unwrap();
    let mut args = std::env::args().skip(1);
    let mode = args.next().expect("Expected dump/rebuild");
    let path = args.next().expect("Expected file name as argument");
    let mut game_data = rgmk::GameData::from_file(&path).unwrap();
    match &mode[..] {
        "dump" => {
            let out = args.next().expect("Expected file name for output as argument");
            let f = File::create(out).unwrap();
            let mut writer = BufWriter::new(f);
            for string in &game_data.strings.strings {
                writeln!(writer, "{}", string).unwrap();
            }
        }
        "rebuild" => {
            let in_ = args.next().expect("Expected file name for input as argument");
            let f = File::open(in_).unwrap();
            let reader = BufReader::new(f);
            let strings = reader.lines().map(|l| l.unwrap()).collect();
            game_data.strings.strings = strings;
            game_data.save_to_file(&(path + ".rgmk")).unwrap();
        }
        _ => {
            println!("Invalid mode \"{}\"", mode);
            return;
        }
    }
}
