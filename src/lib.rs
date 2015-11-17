//! Library for manipulating Game Maker Studio's "data.win" (GEN8) data files.
//!
//! Typical usage consists of reading [GameData](struct.GameData.html) from a
//! file, doing stuff with it,
//! then writing it back to a file.
//!
//! Example:
//!
//! ```no_run
//! let mut game_data = rgmk::GameData::from_file("data.win")
//!                                    .expect("Failed to open data.win");
//! game_data.strings.strings[0] = "NYEH HEH HEH!".into();
//! game_data.save_to_file("data.win").expect("Failed to save data.win");
//! ```
//!

#![feature(read_exact, associated_consts)]
#![warn(missing_docs, trivial_casts, trivial_numeric_casts)]

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
    metadata: MetaData,
    options: Options,
    extn: Extn,
    /// The sounds of the game.
    pub sounds: Sounds,
    audio_groups: Option<AudioGroups>,
    /// The sprites of the game.
    pub sprites: Sprites,
    backgrounds: Backgrounds,
    paths: Paths,
    scripts: Scripts,
    shaders: Shaders,
    /// The fonts of the game.
    pub fonts: Fonts,
    timelines: Timelines,
    objects: Objects,
    rooms: Rooms,
    dafl: Dafl,
    tpag: Tpag,
    code: Code,
    variables: Variables,
    functions: Functions,
    /// The strings of the game.
    pub strings: Strings,
    /// The texture data of the game.
    pub textures: Textures,
    audio: Audio,
}

/// A reader that satisfies the requirements for reading a GameData.
pub trait GameDataRead: Read + io::Seek {}
impl<T: Read + io::Seek> GameDataRead for T {}
/// A writer that satisfies the requirements for writing a GameData.
pub trait GameDataWrite: Write + io::Seek {}
impl<T: Write + io::Seek> GameDataWrite for T {}

impl GameData {
    /// Reads a GameData from a reader.
    pub fn from_reader<R: GameDataRead>(reader: &mut R) -> Result<GameData, ReadError> {
        gamedata_io::read(reader)
    }
    /// Reads a GameData from a file.
    pub fn from_file<P: AsRef<path::Path>>(path: P) -> Result<GameData, ReadError> {
        use std::fs::File;
        use std::io::BufReader;
        let file = try!(File::open(path));
        GameData::from_reader(&mut BufReader::new(file))
    }
    /// Writes self to a writer.
    pub fn write_to_writer<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
        gamedata_io::write(self, writer)
    }
    /// Writes self to a file.
    pub fn save_to_file<P: AsRef<path::Path>>(&self, path: P) -> io::Result<()> {
        use std::fs::File;
        use std::io::BufWriter;
        let file = try!(File::create(path));
        self.write_to_writer(&mut BufWriter::new(file))
    }
    /// Returns the window width of the game.
    pub fn window_width(&self) -> u32 {
        self.metadata.window_width
    }
    /// Returns the window height of the game.
    pub fn window_height(&self) -> u32 {
        self.metadata.window_height
    }
    /// Sets the window dimensions of the game.
    pub fn set_window_dimensions(&mut self, width: u32, height: u32) {
        self.metadata.window_width = width;
        self.metadata.window_height = height;
    }
    /// Returns the window title of the game.
    pub fn window_title(&self) -> &str {
        &self.strings.strings[self.metadata.window_title_index]
    }
    /// Sets the window title of the game.
    pub fn set_window_title<S: Into<String>>(&mut self, new: S) {
        self.strings.strings[self.metadata.window_title_index] = new.into();
    }
}

/// Contains various metadata, for example, the window width/height/title.
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

/// A game maker sound.
#[derive(Clone, Copy)]
pub struct Sound {
    /// Index of the name in the string table.
    pub name_index: usize,
    unk1: u32,
    /// Index of the extension in the string table.
    pub ext_index: usize,
    /// Index of the filename in the string table.
    pub filename_index: usize,
    unk2: u32,
    unk3: u32,
    unk4: u32,
    unk5: u32,
    unk6: u32,
}

/// A collection of sounds.
pub struct Sounds {
    /// The sounds.
    pub sounds: Vec<Sound>,
}

/// Collection of audio groups. Not present in all games.
pub struct AudioGroups {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A game maker sprite.
#[derive(Clone)]
pub struct Sprite {
    /// The index of the resource name in the string table.
    pub name_index: usize,
    /// The width of the sprite
    pub width: u32,
    /// The height of the sprite
    pub height: u32,
    unknown: Vec<u8>,
}

/// A collection of sprites.
pub struct Sprites {
    /// The sprites
    pub sprites: Vec<Sprite>,
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
    /// The scripts.
    pub scripts: Vec<Script>,
}

/// A collection of shaders.
pub struct Shaders {
    raw: Vec<u8>, // Data not analyzed yet
}

/// A font.
#[derive(Clone)]
pub struct Font {
    /// Index of the font's resource name in the string table.
    pub name_index: usize,
    /// Index of the font's font name in the string table.
    pub font_name_index: usize,
    /// Point size of the font.
    pub point_size: u32,
    data: Vec<u8>,
}

/// A collection of fonts.
pub struct Fonts {
    /// The fonts.
    pub fonts: Vec<Font>,
    unknown: Vec<u8>, // Unknown trailing data
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

/// A chunk of code
pub struct CodeChunk {
    /// Index of the name of the code chunk in the string table.
    pub name_index: usize,
    /// The raw code.
    pub raw_code: Vec<u8>,
}

/// A collection of code chunks.
pub struct Code {
    code_chunks: Vec<CodeChunk>,
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
    /// The variables.
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
    /// The functions.
    pub functions: Vec<Function>,
}

/// A collection of strings.
///
/// All strings are assumed to be valid UTF-8.
pub struct Strings {
    /// The vector holding the strings.
    pub strings: Vec<String>,
}

/// A texture.
///
/// Currently only PNG textures are supported.
pub struct Texture {
    unknown: u32, // Purpose unknown. Always seems to be 1.
    /// The PNG data associated with this texture.
    pub png_data: Vec<u8>,
}

/// A collection of textures.
pub struct Textures {
    /// The textures.
    pub textures: Vec<Texture>,
}

/// Raw audio data.
pub struct AudioData {
    data: Vec<u8>,
}

/// A collection of audio data.
pub struct Audio {
    audio: Vec<AudioData>,
    offsets: Vec<u32>, // Audio data is not contiguous, so we need to store relative offsets
}
