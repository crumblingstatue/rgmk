use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use {GameDataRead, GameDataWrite, Script, Scripts};
use gamedata_io::{Chunk, get_chunk_header, ReadError, Tell};

pub(crate) fn write_offsets<W: GameDataWrite>(scripts: &Scripts,
                                              writer: &mut W,
                                              string_offsets: &[u32])
                                              -> io::Result<()> {
    writer.seek(io::SeekFrom::Current(4 + (scripts.scripts.len() * 4) as i64))?;
    for s in &scripts.scripts {
        writer.write_u32::<LittleEndian>(string_offsets[s.name_index])?;
        writer.seek(io::SeekFrom::Current(4))?;
    }
    Ok(())
}

impl<'a> Chunk<'a> for Scripts {
    const TYPE_ID: &'static [u8; 4] = b"SCPT";
    type ReadOutput = (Self, Vec<u32>);
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        get_chunk_header(reader, Self::TYPE_ID)?;
        let num_scripts = reader.read_u32::<LittleEndian>()?;
        trace!("{} scripts", num_scripts);
        // Read script entry offsets
        for _ in 0..num_scripts {
            // For now just discard them
            reader.read_u32::<LittleEndian>()?;
        }
        let mut name_offsets = Vec::new();
        let mut scripts = Vec::new();
        for _ in 0..num_scripts {
            name_offsets.push(reader.read_u32::<LittleEndian>()?);
            scripts.push(Script {
                name_index: 0,
                unknown: reader.read_u32::<LittleEndian>()?,
            });
        }
        Ok((Scripts { scripts: scripts }, name_offsets))
    }
    fn write_content<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32::<LittleEndian>(self.scripts.len() as u32)?;
        let writer_offset = writer.tell()? as u32;
        let first_script_offset = writer_offset + (self.scripts.len() as u32 * 4);
        // Write offset data
        for i in 0..self.scripts.len() as u32 {
            writer.write_u32::<LittleEndian>(first_script_offset + (i * 8))?;
        }
        // Write script data
        for s in &self.scripts {
            // We'll write this later
            writer.seek(io::SeekFrom::Current(4))?;
            writer.write_u32::<LittleEndian>(s.unknown)?;
        }
        Ok(())
    }
}
