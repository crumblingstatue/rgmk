use std::io::prelude::*;
use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use {GameDataRead, GameDataWrite, Function, Functions};
use gamedata_io::{Chunk, get_chunk_header, ReadError};

impl<'a> Chunk<'a> for Functions {
    const TYPE_ID: &'static [u8; 4] = b"FUNC";
    type ReadOutput = (Self, Vec<u32>);
    type WriteInput = &'a [u32];
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        let header = try!(get_chunk_header(reader, Self::TYPE_ID));
        let mut offsets = Vec::new();
        let mut funs = Vec::new();
        let mut remaining = header.size;
        while remaining > 0 {
            let offset = try!(reader.read_u32::<LittleEndian>());
            let unk = try!(reader.read_u32::<LittleEndian>());
            let code_offset = try!(reader.read_u32::<LittleEndian>());
            trace!("unk {}, code offset {}", unk, code_offset);
            funs.push(Function {
                name_index: 0,
                unknown: unk,
                code_offset: code_offset,
            });
            offsets.push(offset);
            remaining -= 3 * 4;
        }
        Ok((Functions { functions: funs }, offsets))
    }
    fn write<W: GameDataWrite>(&self, writer: &mut W, input: Self::WriteInput) -> io::Result<()> {
        try!(writer.write_all(Self::TYPE_ID));
        let len = self.content_size();
        try!(writer.write_u32::<LittleEndian>(len));
        for fun in &self.functions {
            try!(writer.write_u32::<LittleEndian>(input[fun.name_index]));
            try!(writer.write_u32::<LittleEndian>(fun.unknown));
            try!(writer.write_u32::<LittleEndian>(fun.code_offset));
        }
        Ok(())
    }
    fn content_size(&self) -> u32 {
        (self.functions.len() * (3 * 4)) as u32
    }
}