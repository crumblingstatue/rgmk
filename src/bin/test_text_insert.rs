extern crate rgmk;
extern crate env_logger;

fn main() {
    env_logger::init().unwrap();
    let path = std::env::args().skip(1).next().expect("Expected file name as argument");
    let mut gamedata = rgmk::GameData::from_file(&path).unwrap();
    gamedata.save_to_file(&(path + ".rgmk")).unwrap();
}
