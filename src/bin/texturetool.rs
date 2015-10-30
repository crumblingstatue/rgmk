extern crate rgmk;
extern crate env_logger;

use std::io::prelude::*;
use std::fs::File;

fn main() {
    env_logger::init().unwrap();
    let mut args = std::env::args().skip(1);
    let mode = args.next().expect("Expected dump/rebuild");
    let path = args.next().expect("Expected file name as argument");
    let mut game_data = rgmk::GameData::from_file(&path).unwrap();
    match &mode[..] {
        "dump" => {
            for (i, texture) in game_data.textures.textures.iter().enumerate() {
                let mut f = File::create(format!("{}.png", i)).unwrap();
                f.write_all(&texture.png_data).unwrap();
            }
        }
        "rebuild" => {
            for (i, texture) in game_data.textures.textures.iter_mut().enumerate() {
                let mut f = File::open(format!("{}.png", i)).unwrap();
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).unwrap();
                (*texture).png_data = buffer;
            }
            game_data.save_to_file(&(path + ".rgmk")).unwrap();
        }
        _ => {
            println!("Invalid mode \"{}\"", mode);
            return;
        }
    }
}
