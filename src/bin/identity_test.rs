extern crate rgmk;

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().expect("Expected path as argument");
    let root = rgmk::load_from_file(&path).unwrap();
    root.save_to_file(String::from(path) + ".dup").unwrap();
}
