extern crate rgmk;
extern crate env_logger;

fn main() {
    env_logger::init().unwrap();
    let path = std::env::args().skip(1).next().expect("Expected file name as argument");
    let mut gamedata = rgmk::GameData::from_file(&path).unwrap();
    for i in 0..gamedata.fonts.fonts.len() {
        gamedata.fonts.fonts[i] = gamedata.fonts.fonts[9].clone();
    }
    let papyrus = gamedata.sounds.sounds[252];
    let papyrusboss = gamedata.sounds.sounds[237];
    let papyrusvoice = gamedata.sounds.sounds[87];
    gamedata.sounds.sounds[214] = papyrus;
    gamedata.sounds.sounds[390] = papyrusboss;
    gamedata.sounds.sounds[78] = papyrusvoice;
    gamedata.sounds.sounds[79] = papyrusvoice;
    gamedata.sounds.sounds[80] = papyrusvoice;
    gamedata.sounds.sounds[81] = papyrusvoice;
    gamedata.sounds.sounds[82] = papyrusvoice;
    gamedata.sounds.sounds[83] = papyrusvoice;
    gamedata.sounds.sounds[84] = papyrusvoice;
    gamedata.sounds.sounds[85] = papyrusvoice;
    gamedata.sounds.sounds[88] = papyrusvoice;
    gamedata.sounds.sounds[89] = papyrusvoice;
    gamedata.sounds.sounds[90] = papyrusvoice;
    gamedata.sounds.sounds[99] = papyrusvoice;
    gamedata.sounds.sounds[100] = papyrusvoice;
    gamedata.sounds.sounds[101] = papyrusvoice;
    for (i, s) in gamedata.sounds.sounds.iter_mut().enumerate() {
        println!("{} => {} {} {}", i, gamedata.strings.strings[s.name_index],
                             gamedata.strings.strings[s.ext_index],
                             gamedata.strings.strings[s.filename_index]);
    }
    gamedata.save_to_file(&(path + ".rgmk")).unwrap();
}
