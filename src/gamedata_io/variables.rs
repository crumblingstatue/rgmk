use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use {GameDataRead, GameDataWrite, Variable, Variables};
use gamedata_io::{Chunk, get_chunk_header, ReadError};

pub(super) fn write_offsets<W: GameDataWrite>(variables: &Variables,
                                              writer: &mut W,
                                              string_offsets: &[u32])
                                              -> io::Result<()> {
    for var in &variables.variables {
        writer.write_u32::<LittleEndian>(string_offsets[var.name_index])?;
        writer.seek(io::SeekFrom::Current(8))?;
    }
    Ok(())
}

impl<'a> Chunk<'a> for Variables {
    const TYPE_ID: &'static [u8; 4] = b"VARI";
    type ReadOutput = (Self, Vec<u32>);
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        let header = get_chunk_header(reader, Self::TYPE_ID)?;
        let mut offsets = Vec::new();
        let mut vars = Vec::new();
        let mut remaining = header.size;
        while remaining > 0 {
            let offset = reader.read_u32::<LittleEndian>()?;
            let unk = reader.read_u32::<LittleEndian>()?;
            let code_offset = reader.read_u32::<LittleEndian>()?;
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
    fn write_content<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
        for var in &self.variables {
            // We'll write this later
            writer.seek(io::SeekFrom::Current(4))?;
            writer.write_u32::<LittleEndian>(var.unknown)?;
            writer.write_u32::<LittleEndian>(var.code_offset)?;
        }
        Ok(())
    }
}
