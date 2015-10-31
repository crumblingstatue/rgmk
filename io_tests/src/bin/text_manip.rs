extern crate rgmk;
extern crate env_logger;

fn main() {
    env_logger::init().unwrap();
    let path = std::env::args().skip(1).next().expect("Expected file name as argument");
    let mut gamedata = rgmk::GameData::from_file(&path).unwrap();
    // Popping a char messes something up, the audio doesn't play.
    gamedata.strings.strings[0].pop();
    gamedata.save_to_file(&(path + ".rgmk")).unwrap();
}
