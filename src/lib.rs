#![feature(read_exact)]

#[macro_use]
extern crate quick_error;
extern crate byteorder;

use std::path::Path;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::io::prelude::*;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

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

#[derive(Debug, Clone)]
pub struct StringTable {
    pub offsets: Vec<u32>,
    pub strings: Vec<String>,
}

#[derive(Debug)]
pub struct FuncData {
    nameoffset: u32,
    /// Could be size?
    unk1: u32,
    /// Could be data pointer
    unk2: u32,
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
    Function(Vec<FuncData>),
    StringTable(StringTable),
    Txtr(Vec<u8>),
    Audio(Vec<u8>),
}

#[derive(Debug)]
pub struct Chunk {
    pub content: ChunkContent,
    pub size: i32,
}

pub const TYPE_ID_LEN: usize = 4;
pub const CHUNK_HEADER_LEN: usize = TYPE_ID_LEN + 4;

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

fn read_string<R: Read>(reader: &mut R) -> Result<String, StringReadError> {
    let len = try!(reader.read_u32::<LittleEndian>());
    let mut buf = Vec::with_capacity(len as usize);
    unsafe {
        buf.set_len(len as usize);
        try!(reader.read_exact(&mut buf));
    }
    let terminator = try!(reader.read_u8());
    if terminator == 0 {
        // We assume strings are valid UTF-8, if not, panic.
        Ok(String::from_utf8(buf).unwrap())
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
            let mut remaining = size;
            let mut funcs = Vec::new();
            while remaining > 0 {
                let nameptr = try!(reader.read_u32::<LittleEndian>());
                let unk1 = try!(reader.read_u32::<LittleEndian>());
                let unk2 = try!(reader.read_u32::<LittleEndian>());
                remaining -= 3 * 4;
                funcs.push(FuncData {
                    nameoffset: nameptr,
                    unk1: unk1,
                    unk2: unk2,
                });
            }
            ChunkContent::Function(funcs)
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
            // Looks like 4 zero bytes.
            let mut buf = [0u8; 4];
            try!(reader.read_exact(&mut buf));
            ChunkContent::StringTable(StringTable {
                offsets: offsets,
                strings: strings,
            })
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

fn write_chunk<W: Write>(writer: &mut W, chunk: &Chunk) -> Result<(), io::Error> {
    match chunk.content {
        ChunkContent::Form(ref chunks) => {
            try!(writer.write_all(b"FORM"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            for chunk in chunks.iter() {
                try!(write_chunk(writer, chunk));
            }
            Ok(())
        }
        ChunkContent::Gen8(ref vec) => {
            try!(writer.write_all(b"GEN8"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Optn(ref vec) => {
            try!(writer.write_all(b"OPTN"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Extn(ref vec) => {
            try!(writer.write_all(b"EXTN"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Sound(ref vec) => {
            try!(writer.write_all(b"SOND"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Agrp(ref vec) => {
            try!(writer.write_all(b"AGRP"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Sprite(ref vec) => {
            try!(writer.write_all(b"SPRT"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Background(ref vec) => {
            try!(writer.write_all(b"BGND"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Path(ref vec) => {
            try!(writer.write_all(b"PATH"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Script(ref vec) => {
            try!(writer.write_all(b"SCPT"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Shader(ref vec) => {
            try!(writer.write_all(b"SHDR"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Font(ref vec) => {
            try!(writer.write_all(b"FONT"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Timeline(ref vec) => {
            try!(writer.write_all(b"TMLN"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Object(ref vec) => {
            try!(writer.write_all(b"OBJT"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Room(ref vec) => {
            try!(writer.write_all(b"ROOM"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Dafl(ref vec) => {
            try!(writer.write_all(b"DAFL"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Tpag(ref vec) => {
            try!(writer.write_all(b"TPAG"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Code(ref vec) => {
            try!(writer.write_all(b"CODE"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Vari(ref vec) => {
            try!(writer.write_all(b"VARI"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Function(ref vec) => {
            try!(writer.write_all(b"FUNC"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            for fun in vec {
                try!(writer.write_u32::<LittleEndian>(fun.nameoffset));
                try!(writer.write_u32::<LittleEndian>(fun.unk1));
                try!(writer.write_u32::<LittleEndian>(fun.unk2));
            }
            Ok(())
        }
        ChunkContent::StringTable(ref table) => {
            try!(writer.write_all(b"STRG"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_u32::<LittleEndian>(table.strings.len() as u32));
            for offs in &table.offsets {
                try!(writer.write_u32::<LittleEndian>(*offs));
            }
            for string in &table.strings {
                try!(writer.write_u32::<LittleEndian>(string.len() as u32));
                try!(writer.write_all(string.as_bytes()));
                try!(writer.write_u8(0));
            }
            // Required padding
            try!(writer.write_all(&[0u8; 4]));
            Ok(())
        }
        ChunkContent::Txtr(ref vec) => {
            try!(writer.write_all(b"TXTR"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
        ChunkContent::Audio(ref vec) => {
            try!(writer.write_all(b"AUDO"));
            try!(writer.write_i32::<LittleEndian>(chunk.content_len()));
            try!(writer.write_all(vec));
            Ok(())
        }
    }
}

pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Chunk, LoadError> {
    let file = try!(File::open(path));
    let mut reader = BufReader::new(file);
    read_chunk(&mut reader)
}

impl Chunk {
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        let file = try!(File::create(path));
        let mut writer = BufWriter::new(file);
        write_chunk(&mut writer, self)
    }
    pub fn content_len(&self) -> i32 {
        match self.content {
            ChunkContent::Form(ref chunks) => {
                chunks.iter().fold(0,
                                   |acc, chunk| acc + chunk.content_len() + CHUNK_HEADER_LEN as i32)
            }
            ChunkContent::Gen8(ref vec) => vec.len() as i32,
            ChunkContent::Optn(ref vec) => vec.len() as i32,
            ChunkContent::Extn(ref vec) => vec.len() as i32,
            ChunkContent::Sound(ref vec) => vec.len() as i32,
            ChunkContent::Agrp(ref vec) => vec.len() as i32,
            ChunkContent::Sprite(ref vec) => vec.len() as i32,
            ChunkContent::Background(ref vec) => vec.len() as i32,
            ChunkContent::Path(ref vec) => vec.len() as i32,
            ChunkContent::Script(ref vec) => vec.len() as i32,
            ChunkContent::Shader(ref vec) => vec.len() as i32,
            ChunkContent::Font(ref vec) => vec.len() as i32,
            ChunkContent::Timeline(ref vec) => vec.len() as i32,
            ChunkContent::Object(ref vec) => vec.len() as i32,
            ChunkContent::Room(ref vec) => vec.len() as i32,
            ChunkContent::Dafl(ref vec) => vec.len() as i32,
            ChunkContent::Tpag(ref vec) => vec.len() as i32,
            ChunkContent::Code(ref vec) => vec.len() as i32,
            ChunkContent::Vari(ref vec) => vec.len() as i32,
            ChunkContent::Function(ref vec) => vec.len() as i32 * (3 * 4),
            ChunkContent::StringTable(ref table) => {
                let mut lengths = 0;
                for s in &table.strings {
                    // The length denominator before the string
                    lengths += 4;
                    // + 1 for null terminator
                    lengths += s.len() + 1;
                }
                // +4 at end for zero padding
                (4 + (table.offsets.len() * 4) + lengths + 4) as i32
            }
            ChunkContent::Txtr(ref vec) => vec.len() as i32,
            ChunkContent::Audio(ref vec) => vec.len() as i32,
        }
    }
}
