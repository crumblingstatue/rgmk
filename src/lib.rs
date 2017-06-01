//! Library for manipulating Game Maker Studio's "data.win" (GEN8) data files.

#![warn( /*missing_docs,*/
 trivial_casts, trivial_numeric_casts)]

extern crate byteorder;

mod serde;
mod io_util;

use std::io::{self, BufReader, BufWriter};
use std::path;
use std::error::Error;
use std::fs::File;

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
    reader: FileBufRead,
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

type FileBufRead = BufReader<File>;
type FileBufWrite = BufWriter<File>;

impl GameData {
    /// Reads a GameData from a file.
    pub fn open<P: AsRef<path::Path>>(path: P) -> Result<GameData, Box<Error>> {
        let file = File::open(path)?;
        serde::open_and_read(BufReader::new(file))
    }
    /// Writes self to a file.
    pub fn save_to_file<P: AsRef<path::Path>>(&mut self, path: P) -> io::Result<()> {
        let file = File::create(path)?;
        serde::write_to(self, &mut BufWriter::new(file))
    }
}
