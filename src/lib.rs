#![feature(read_exact, associated_consts, associated_type_defaults)]

#[macro_use]
extern crate quick_error;
extern crate byteorder;
#[macro_use]
extern crate log;

mod gamedata_io;

pub use gamedata_io::{ReadError, StringReadError};

use std::io::{self, Read, Write};
use std::path;

/// The data of a Game Maker Studio game.
///
/// This is the collective information acquired from "data.win".
pub struct GameData {
    pub metadata: MetaData,
    options: Options,
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

pub trait GameDataRead: Read + io::Seek {}
impl<T: Read + io::Seek> GameDataRead for T {}
pub trait GameDataWrite: Write + io::Seek {}
impl<T: Write + io::Seek> GameDataWrite for T {}

impl GameData {
    pub fn from_reader<R: GameDataRead>(reader: &mut R) -> Result<GameData, ReadError> {
        gamedata_io::read(reader)
    }
    pub fn from_file<P: AsRef<path::Path>>(path: &P) -> Result<GameData, ReadError> {
        use std::fs::File;
        use std::io::BufReader;
        let file = try!(File::open(path));
        GameData::from_reader(&mut BufReader::new(file))
    }
    pub fn write_to_writer<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
        gamedata_io::write(self, writer)
    }
    pub fn save_to_file<P: AsRef<path::Path>>(&self, path: &P) -> io::Result<()> {
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

/// Game Maker project Options
pub struct Options {
    unk1: u32, // Unknown
    unk2: u32, // Unknown
    icon_offset: u32, // Points to texture data (icon?)
    unk3: u32,
    unk4: u32,
    unk5: u32,
    unk6: u32,
    unk7: u32,
    unk8: u32,
    unk9: u32,
    unk10: u32,
    unk11: u32,
    unk12: u32,
    unk13: u32,
    unk14: u32,
    unk15: u32,
    constant1_name_index: usize, // Offset of the name of this constant in the string table
    constant2_name_index: usize, // Offset of the name of this constant in the string table
    constant3_name_index: usize, // Offset of the name of this constant in the string table
    constant4_name_index: usize, // Offset of the name of this constant in the string table
    constant5_name_index: usize, // Offset of the name of this constant in the string table
    constant6_name_index: usize, // Offset of the name of this constant in the string table
    constant7_name_index: usize, // Offset of the name of this constant in the string table
    constant8_name_index: usize, // Offset of the name of this constant in the string table
    constant9_name_index: usize, // Offset of the name of this constant in the string table
    constant10_name_index: usize, // Offset of the name of this constant in the string table
    constant11_name_index: usize, // Offset of the name of this constant in the string table
    constant12_name_index: usize, // Offset of the name of this constant in the string table
    constant13_name_index: usize, // Offset of the name of this constant in the string table
    constant14_name_index: usize, // Offset of the name of this constant in the string table
}

/// Purpose unknown.
pub struct Extn {
    raw: Vec<u8>, // Data not analyzed yet
}

pub struct Sound {
    name_index: usize,
    unk1: u32,
    ext_index: usize,
    filename_index: usize,
    unk2: u32,
    unk3: u32,
    unk4: u32,
    unk5: u32,
    unk6: u32,
}

/// A collection of sounds.
pub struct Sounds {
    sounds: Vec<Sound>,
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

/// A game maker script.
pub struct Script {
    /// Index of the name of the script in the string table
    pub name_index: usize,
    unknown: u32, // Unknown
}

/// A collection of scripts.
pub struct Scripts {
    pub scripts: Vec<Script>,
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
    unknown: u32, // Purpose unknown. Ranges from 1 to a few thousand.
    code_offset: u32, // Points into the code section
}

/// A collection of variables.
pub struct Variables {
    pub variables: Vec<Variable>,
}

/// A game maker function.
pub struct Function {
    /// Index of the name of the function in the strings section.
    pub name_index: usize,
    unknown: u32, // Purpose unknown. Ranges from 1 to a few thousand.
    code_offset: u32, // Points into the code section.
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

pub struct Texture {
    unknown: u32, // Purpose unknown. Always seems to be 1.
    offset: u32, // Offset of data in the texture data
}

/// A collection of textures.
pub struct Textures {
    pub textures: Vec<Texture>,
    texture_data: Vec<u8>,
}

pub struct AudioData {
    data: Vec<u8>,
}

/// A collection of audio data.
pub struct Audio {
    audio: Vec<AudioData>,
    offsets: Vec<u32>, // Audio data is not contiguous, so we need to store relative offsets
    size: u32, // Fuck it
}
