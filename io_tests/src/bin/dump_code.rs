extern crate rgmk;
extern crate env_logger;

fn main() {
    env_logger::init().unwrap();
    let path = std::env::args().skip(1).next().expect("Expected file name as argument");
    let gamedata = rgmk::GameData::from_file(&path).unwrap();
    for chunk in &gamedata.code.code_chunks {
        let name = &gamedata.strings.strings[chunk.name_index][..];
        println!("Code chunk == \"{}\" ==", name);
        for b in &chunk.raw_code {
            print!("{:02X} ", b);
        }
        print!("\n");
    }
}
