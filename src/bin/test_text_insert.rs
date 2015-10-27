extern crate rgmk;
extern crate env_logger;

fn main() {
    env_logger::init().unwrap();
    let path = std::env::args().skip(1).next().expect("Expected file name as argument");
    let mut gamedata = rgmk::GameData::from_file(&path).unwrap();
    let mut string = gamedata.string_at(0).to_owned();
    string.push('K');
    println!("\"{}\"", string);
    gamedata.replace_string_at(0, string);
    gamedata.save_to_file(&(path + ".rgmk")).unwrap();
}
