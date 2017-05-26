//! Library for manipulating Game Maker Studio's "data.win" (GEN8) data files.

#![warn(missing_docs, trivial_casts, trivial_numeric_casts)]

extern crate byteorder;

mod pseudo_iff;

use std::io::{self, Read, Write};
use std::path;
use std::error::Error;
use pseudo_iff::PseudoIff;

/// The data of a Game Maker Studio game.
///
/// This is the collective information acquired from "data.win".
pub struct GameData {
    iff: PseudoIff,
}

/// A reader that satisfies the requirements for reading a `GameData`.
pub trait GameDataRead: Read + io::Seek {}
impl<T: Read + io::Seek> GameDataRead for T {}
/// A writer that satisfies the requirements for writing a `GameData`.
pub trait GameDataWrite: Write + io::Seek {}
impl<T: Write + io::Seek> GameDataWrite for T {}

impl GameData {
    /// Reads a GameData from a reader.
    pub fn from_reader<R: GameDataRead>(reader: &mut R) -> Result<GameData, Box<Error>> {
        let iff = PseudoIff::load(reader)?;
        Ok(Self { iff })
    }
    /// Reads a GameData from a file.
    pub fn from_file<P: AsRef<path::Path>>(path: P) -> Result<GameData, Box<Error>> {
        use std::fs::File;
        use std::io::BufReader;
        let file = File::open(path)?;
        GameData::from_reader(&mut BufReader::new(file))
    }
    /// Writes self to a writer.
    pub fn write_to_writer<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
        self.iff.save(writer)
    }
    /// Writes self to a file.
    pub fn save_to_file<P: AsRef<path::Path>>(&self, path: P) -> io::Result<()> {
        use std::fs::File;
        use std::io::BufWriter;
        let file = File::create(path)?;
        self.write_to_writer(&mut BufWriter::new(file))
    }
}
