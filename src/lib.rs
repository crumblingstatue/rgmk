#![feature(read_exact, associated_consts, associated_type_defaults)]

#[macro_use]
extern crate quick_error;
extern crate byteorder;

mod gamedata_io;

pub use gamedata_io::{ReadError, StringReadError};

use std::io::{self, Read, Write};
use std::path;

/// The data of a Game Maker Studio game.
///
/// This is the collective information acquired from "data.win".
pub struct GameData {
    pub metadata: MetaData,
    optn: Optn,
    extn: Extn,
    sounds: Sounds,
    audio_groups: Option<AudioGroups>,
    sprites: Sprites,
    backgrounds: Backgrounds,
    paths: Paths,
    scripts: Scripts,
    shaders: Shaders,
    fonts: Fonts,
    timelines: Timelines,
    objects: Objects,
    rooms: Rooms,
    dafl: Dafl,
    tpag: Tpag,
    code: Code,
    pub variables: Variables,
    pub functions: Functions,
    pub strings: Strings,
    textures: Textures,
    audio: Audio,
}

impl GameData {
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<GameData, ReadError> {
        gamedata_io::read(reader)
    }
    pub fn from_file<P: AsRef<path::Path>>(path: &P) -> Result<GameData, ReadError> {
        use std::fs::File;
        use std::io::BufReader;
        let file = try!(File::open(path));
        GameData::from_reader(&mut BufReader::new(file))
    }
    pub fn write_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        gamedata_io::write(self, writer)
    }
    pub fn save_to_file<P: AsRef<path::Path>>(&self, path: &P) -> Result<(), io::Error> {
        use std::fs::File;
        use std::io::BufWriter;
        let file = try!(File::create(path));
        self.write_to_writer(&mut BufWriter::new(file))
    }
    pub fn string_at(&self, index: usize) -> &str {
        &self.strings.strings[index]
    }
    pub fn replace_string_at<S: Into<String>>(&mut self, index: usize, with: S) {
        self.strings.strings[index] = with.into();
    }
    pub fn window_width(&self) -> u32 {
        self.metadata.window_width
    }
    pub fn window_height(&self) -> u32 {
        self.metadata.window_height
    }
    pub fn set_window_dimensions(&mut self, width: u32, height: u32) {
        self.metadata.window_width = width;
        self.metadata.window_height = height;
    }
    pub fn window_title(&self) -> &str {
        &self.strings.strings[self.metadata.window_title_index]
    }
    pub fn set_window_title<S: Into<String>>(&mut self, new: S) {
        self.strings.strings[self.metadata.window_title_index] = new.into();
    }
}

/// Contains various metadata.
pub struct MetaData {
    unk1: u32, // Purpose unknown
    game_id_1_index: usize, // Some kind of game id
    default_index: usize, // Points to "Default"
    unk2: u32,
    unk3: u32,
    unk4: u32,
    unk5: u32,
    unk6: u32,
    unk7: u32,
    unk8: u32,
    game_id_2_index: usize, // Some kind of game id, identical to game_id_1
    unk9: u32,
    unk10: u32,
    unk11: u32,
    unk12: u32,
    window_width: u32,
    window_height: u32,
    unk13: u32,
    unk14: u32,
    unk15: u32,
    unk16: u32,
    unk17: u32,
    unk18: u32,
    unk19: u32,
    unk20: u32,
    window_title_index: usize,
    unknown: Vec<u32>,
}

/// Purpose unknown.
pub struct Optn {
    raw: Vec<u8>, // Data not analyzed yet
}

/// Purpose unknown.
pub struct Extn {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A collection of sounds.
pub struct Sounds {
    raw: Vec<u8>, // Data not analyzed yet
}

/// Collection of audio groups. Not present in all games.
pub struct AudioGroups {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A collection of sprites.
pub struct Sprites {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A collection of backgrounds.
pub struct Backgrounds {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A collection of paths.
pub struct Paths {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A collection of scripts.
pub struct Scripts {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A collection of shaders.
pub struct Shaders {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A collection of fonts.
pub struct Fonts {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A collection of timelines.
pub struct Timelines {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A collection of objects.
pub struct Objects {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A collection of rooms.
pub struct Rooms {
    raw: Vec<u8>, // Data not analyzed yet
}

/// Purpose unknown.
pub struct Dafl {
    raw: Vec<u8>, // Data not analyzed yet
}

/// Purpose unknown.
pub struct Tpag {
    raw: Vec<u8>, // Data not analyzed yet
}

/// Something to do with code. Maybe code of scripts?
pub struct Code {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A game maker variable.
pub struct Variable {
    /// Index of the name of the variable in the strings section.
    pub name_index: usize,
    unknown1: u32, // Purpose unknown
    unknown2: u32, // Purpose unknown
}

/// A collection of variables.
pub struct Variables {
    pub variables: Vec<Variable>,
}

/// A game maker function.
pub struct Function {
    /// Index of the name of the function in the strings section.
    pub name_index: usize,
    unknown1: u32, // Purpose unknown
    unknown2: u32, // Purpose unknown
}

/// A collection of functions.
pub struct Functions {
    pub functions: Vec<Function>,
}

/// A collection of strings.
///
/// All strings are assumed to be valid UTF-8.
pub struct Strings {
    pub strings: Vec<String>,
}

/// A collection of textures.
pub struct Textures {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A collection of audio data.
pub struct Audio {
    raw: Vec<u8>, // Data not analyzed yet
}
