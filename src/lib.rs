#![feature(read_exact)]

#[macro_use]
extern crate quick_error;
extern crate byteorder;

use std::path::Path;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct GmkString {
    buf: Vec<u8>,
}

impl GmkString {
    pub fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.buf.clone())
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum StringReadError {
        Io(err: io::Error) {
            from()
        }
        ByteOrder(err: byteorder::Error) {
            from()
        }
        MissingNullTerminator {}
    }
}

#[derive(Debug)]
pub enum ChunkContent {
    Form(Vec<Chunk>),
    Gen8(Vec<u8>),
    Optn(Vec<u8>),
    Extn(Vec<u8>),
    Sound(Vec<u8>),
    Agrp(Vec<u8>),
    Sprite(Vec<u8>),
    Background(Vec<u8>),
    Path(Vec<u8>),
    Script(Vec<u8>),
    Shader(Vec<u8>),
    Font(Vec<u8>),
    Timeline(Vec<u8>),
    Object(Vec<u8>),
    Room(Vec<u8>),
    Dafl(Vec<u8>),
    Tpag(Vec<u8>),
    Code(Vec<u8>),
    Vari(Vec<u8>),
    Function(Vec<u8>),
    StringTable {
        offsets: Vec<u32>,
        strings: Vec<GmkString>,
    },
    Txtr(Vec<u8>),
    Audio(Vec<u8>),
}

#[derive(Debug)]
pub struct Chunk {
    pub content: ChunkContent,
    pub size: i32,
}

const TYPE_ID_LEN: usize = 4;
const CHUNK_HEADER_LEN: usize = TYPE_ID_LEN + 4;

quick_error! {
    #[derive(Debug)]
    pub enum LoadError {
        Io(err: io::Error) {
            from()
        }
        ByteOrder(err: byteorder::Error) {
            from()
        }
        String(err: StringReadError) {
            from()
        }
    }
}

fn read_into_byte_vec<R: Read>(reader: &mut R, len: usize) -> Result<Vec<u8>, io::Error> {
    let mut vec = Vec::with_capacity(len);
    unsafe {
        vec.set_len(len);
        try!(reader.read_exact(&mut vec));
    }
    Ok(vec)
}

fn read_string<R: Read>(reader: &mut R) -> Result<GmkString, StringReadError> {
    let len = try!(reader.read_u32::<LittleEndian>());
    let mut buf = Vec::with_capacity(len as usize);
    unsafe {
        buf.set_len(len as usize);
        try!(reader.read_exact(&mut buf));
    }
    let terminator = try!(reader.read_u8());
    if terminator == 0 {
        Ok(GmkString { buf: buf })
    } else {
        Err(StringReadError::MissingNullTerminator)
    }
}

fn read_chunk<R: Read>(reader: &mut R) -> Result<Chunk, LoadError> {
    let mut type_id = [0u8; TYPE_ID_LEN];
    try!(reader.read_exact(&mut type_id));
    let size = try!(reader.read_i32::<LittleEndian>());

    let content = match &type_id {
        b"FORM" => {
            let mut chunks = Vec::new();
            let mut bytes_left = size;
            while bytes_left > 0 {
                let chunk = try!(read_chunk(reader));
                bytes_left -= chunk.size + CHUNK_HEADER_LEN as i32;
                chunks.push(chunk);
            }
            ChunkContent::Form(chunks)
        }
        b"GEN8" => {
            ChunkContent::Gen8(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"OPTN" => {
            ChunkContent::Optn(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"EXTN" => {
            ChunkContent::Extn(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"SOND" => {
            ChunkContent::Sound(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"AGRP" => {
            ChunkContent::Agrp(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"SPRT" => {
            ChunkContent::Sprite(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"BGND" => {
            ChunkContent::Background(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"PATH" => {
            ChunkContent::Path(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"SCPT" => {
            ChunkContent::Script(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"SHDR" => {
            ChunkContent::Shader(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"FONT" => {
            ChunkContent::Font(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"TMLN" => {
            ChunkContent::Timeline(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"OBJT" => {
            ChunkContent::Object(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"ROOM" => {
            ChunkContent::Room(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"DAFL" => {
            ChunkContent::Dafl(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"TPAG" => {
            ChunkContent::Tpag(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"CODE" => {
            ChunkContent::Code(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"VARI" => {
            ChunkContent::Vari(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"FUNC" => {
            ChunkContent::Function(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"STRG" => {
            let count = try!(reader.read_u32::<LittleEndian>());
            let mut offsets = Vec::with_capacity(count as usize);
            for _ in 0..count {
                let offset = try!(reader.read_u32::<LittleEndian>());
                offsets.push(offset);
            }
            let mut strings = Vec::with_capacity(count as usize);
            for _ in 0..count {
                let string = try!(read_string(reader));
                strings.push(string);
            }
            // TODO: Why do we need to consume additional 4 bytes?
            let _ = reader.read_u8();
            let _ = reader.read_u8();
            let _ = reader.read_u8();
            let _ = reader.read_u8();
            ChunkContent::StringTable {
                offsets: offsets,
                strings: strings,
            }
        }
        b"TXTR" => {
            ChunkContent::Txtr(try!(read_into_byte_vec(reader, size as usize)))
        }
        b"AUDO" => {
            ChunkContent::Audio(try!(read_into_byte_vec(reader, size as usize)))
        }
        _ => panic!("Unknown type id \"{}\"", String::from_utf8_lossy(&type_id)),
    };

    Ok(Chunk {
        content: content,
        size: size,
    })
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Chunk, LoadError> {
    let file = try!(File::open(path));
    let mut reader = BufReader::new(file);
    read_chunk(&mut reader)
}
