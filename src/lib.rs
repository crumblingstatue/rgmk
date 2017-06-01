//! Library for manipulating Game Maker Studio's "data.win" (GEN8) data files.

#![warn( /*missing_docs,*/
 trivial_casts, trivial_numeric_casts)]

extern crate byteorder;

mod serde;
mod io_util;

use std::io::{self, Read, Write, BufReader};
use std::path;
use std::error::Error;

/// The data of a Game Maker Studio game.
///
/// This is the collective information acquired from "data.win".
pub struct GameData {
    pub gen8: Box<[u8]>,
    pub optn: Box<[u8]>,
    pub extn: Box<[u8]>,
    pub sond: Box<[u8]>,
    pub agrp: Option<Box<[u8]>>,
    pub sprt: Box<[u8]>,
    pub bgnd: Box<[u8]>,
    pub path: Box<[u8]>,
    pub scpt: Box<[u8]>,
    pub shdr: Box<[u8]>,
    pub font: Box<[u8]>,
    pub tmln: Box<[u8]>,
    pub objt: Box<[u8]>,
    pub room: Box<[u8]>,
    pub dafl: Box<[u8]>,
    pub tpag: Box<[u8]>,
    pub code: Box<[u8]>,
    pub vari: Box<[u8]>,
    pub func: Box<[u8]>,
    pub strg: Box<[u8]>,
    pub txtr: Txtr,
    pub audo: Box<[u8]>,
    pub lang: Option<Box<[u8]>>,
    pub glob: Option<Box<[u8]>>,
}

pub struct Texture {
    unknown: u32,
    source: TextureSource,
}

pub enum TextureSource {
    Original { offset: u64 },
}

pub struct Txtr {
    textures: Vec<Texture>,
    end_offset: u64,
}

/// A write that satisfies the requirements for reading a `GameData`.
pub trait GameDataRead: Read + io::Seek {}
impl<T: Read + io::Seek> GameDataRead for T {}

/// A writer that satisfies the requirements for writing a `GameData`.
pub trait GameDataWrite: Write + io::Seek {}
impl<T: Write + io::Seek> GameDataWrite for T {}

impl GameData {
    /// Reads a GameData from a reader.
    pub fn from_reader<R: GameDataRead>(reader: &mut R) -> Result<GameData, Box<Error>> {
        serde::read_from(reader)
    }
    /// Reads a GameData from a file.
    pub fn from_file<P: AsRef<path::Path>>(path: P) -> Result<GameData, Box<Error>> {
        use std::fs::File;
        use std::io::BufReader;
        let file = File::open(path)?;
        GameData::from_reader(&mut BufReader::new(file))
    }
    /// Writes self to a writer.
    pub fn write_to_writer<W: GameDataWrite, R: GameDataRead>(
        &self,
        writer: &mut W,
        reader_orig: &mut R,
    ) -> io::Result<()> {
        serde::write_to(self, writer, reader_orig)
    }
    /// Writes self to a file.
    pub fn save_to_file<P: AsRef<path::Path>, P2: AsRef<path::Path>>(
        &self,
        path: P,
        path_of_orig: P2,
    ) -> io::Result<()> {
        use std::fs::File;
        use std::io::BufWriter;
        let file = File::create(path)?;
        let file_orig = File::open(path_of_orig)?;
        self.write_to_writer(&mut BufWriter::new(file), &mut BufReader::new(file_orig))
    }
}
