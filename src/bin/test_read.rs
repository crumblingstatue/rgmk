extern crate rgmk;

fn main() {
    let path = std::env::args().skip(1).next().expect("Expected file name as argument");
    let _f = rgmk::GameData::from_file(&path).unwrap();
}
