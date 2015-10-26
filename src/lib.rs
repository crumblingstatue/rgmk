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
    metadata: MetaData,
    optn: Optn,
    extn: Extn,
    sounds: Sounds,
    agrp: Option<Agrp>,
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
    vari: Vari,
    functions: Functions,
    strings: Strings,
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
}

/// Contains various metadata.
pub struct MetaData {
    raw: Vec<u8>, // Data not analyzed yet
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

/// Purpose unknown. Not present in all games.
pub struct Agrp {
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

/// Variable data?
pub struct Vari {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A collection of functions.
pub struct Functions {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A collection of strings.
///
/// All strings are assumed to be valid UTF-8.
pub struct Strings {
    strings: Vec<String>,
}

/// A collection of textures.
pub struct Textures {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A collection of audio data.
pub struct Audio {
    raw: Vec<u8>, // Data not analyzed yet
}
