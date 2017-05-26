extern crate rgmk;

fn main() {
    let path = std::env::args().nth(1).expect(
        "Expected file name as argument",
    );
    let _gamedata = rgmk::GameData::from_file(&path).unwrap();
    /*for chunk in &gamedata.code.code_chunks {
        let name = &gamedata.strings.strings[chunk.name_index][..];
        println!("Code chunk == \"{}\" ==", name);
        for b in &chunk.raw_code {
            print!("{:02X} ", b);
        }
        print!("\n");
    }*/
}
