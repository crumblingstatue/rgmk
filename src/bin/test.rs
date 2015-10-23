extern crate rgmk;

fn main() {
    let path = std::env::args_os().skip(1).next().expect("Expected path as argument");
    let root = rgmk::load_from_file(path);
}
