extern crate rgmk;

use std::path::Path;

fn main() {
    let path = std::env::args().nth(1).expect(
        "Expected file name as argument",
    );
    let f = rgmk::GameData::from_file(&path).unwrap();
    f.save_to_file(&Path::new(&path).with_extension("new"), &path)
        .unwrap();
    eprintln!("Successfully read/written {}", path);
}
