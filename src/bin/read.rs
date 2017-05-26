extern crate rgmk;

fn main() {
    let path = std::env::args().nth(1).expect(
        "Expected file name as argument",
    );
    let _f = rgmk::GameData::from_file(&path).unwrap();
    eprintln!("Successfully read {}", path);
}
