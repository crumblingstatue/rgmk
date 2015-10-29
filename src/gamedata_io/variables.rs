use std::io::prelude::*;
use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use {GameDataRead, GameDataWrite, Variable, Variables};
use gamedata_io::{Chunk, get_chunk_header, ReadError};

impl<'a> Chunk<'a> for Variables {
    const TYPE_ID: &'static [u8; 4] = b"VARI";
    type ReadOutput = (Self, Vec<u32>);
    type WriteInput = &'a [u32];
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        let header = try!(get_chunk_header(reader, Self::TYPE_ID));
        let mut offsets = Vec::new();
        let mut vars = Vec::new();
        let mut remaining = header.size;
        while remaining > 0 {
            let offset = try!(reader.read_u32::<LittleEndian>());
            let unk = try!(reader.read_u32::<LittleEndian>());
            let code_offset = try!(reader.read_u32::<LittleEndian>());
            trace!("unk {} code_offset {}", unk, code_offset);
            vars.push(Variable {
                name_index: 0,
                unknown: unk,
                code_offset: code_offset,
            });
            offsets.push(offset);
            remaining -= 3 * 4;
        }
        Ok((Variables { variables: vars }, offsets))
    }
    chunk_write_impl!();
    fn write_content<W: GameDataWrite>(&self,
                                       writer: &mut W,
                                       input: Self::WriteInput)
                                       -> io::Result<()> {
        for var in &self.variables {
            try!(writer.write_u32::<LittleEndian>(input[var.name_index]));
            try!(writer.write_u32::<LittleEndian>(var.unknown));
            try!(writer.write_u32::<LittleEndian>(var.code_offset));
        }
        Ok(())
    }
    fn content_size(&self) -> u32 {
        (self.variables.len() * (3 * 4)) as u32
    }
}
