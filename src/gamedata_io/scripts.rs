use std::io;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use {GameDataRead, GameDataWrite};
use gamedata_io::{Chunk, ReadError, Tell, get_chunk_header};
use unexposed::{Script, Scripts};

pub fn write_offsets<W: GameDataWrite>(scripts: &Scripts,
                                       writer: &mut W,
                                       string_offsets: &[u32])
                                       -> io::Result<()> {
    try!(writer.seek(io::SeekFrom::Current(4 + (scripts.scripts.len() * 4) as i64)));
    for s in &scripts.scripts {
        try!(writer.write_u32::<LittleEndian>(string_offsets[s.name_index]));
        try!(writer.seek(io::SeekFrom::Current(4)));
    }
    Ok(())
}

impl<'a> Chunk<'a> for Scripts {
    const TYPE_ID: &'static [u8; 4] = b"SCPT";
    type ReadOutput = (Self, Vec<u32>);
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        try!(get_chunk_header(reader, Self::TYPE_ID));
        let num_scripts = try!(reader.read_u32::<LittleEndian>());
        trace!("{} scripts", num_scripts);
        // Read script entry offsets
        for _ in 0..num_scripts {
            // For now just discard them
            try!(reader.read_u32::<LittleEndian>());
        }
        let mut name_offsets = Vec::new();
        let mut scripts = Vec::new();
        for _ in 0..num_scripts {
            name_offsets.push(try!(reader.read_u32::<LittleEndian>()));
            scripts.push(Script {
                name_index: 0,
                unknown: try!(reader.read_u32::<LittleEndian>()),
            });
        }
        Ok((Scripts { scripts: scripts }, name_offsets))
    }
    fn write_content<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
        try!(writer.write_u32::<LittleEndian>(self.scripts.len() as u32));
        let writer_offset = try!(writer.tell()) as u32;
        let first_script_offset = writer_offset + (self.scripts.len() as u32 * 4);
        // Write offset data
        for i in 0..self.scripts.len() as u32 {
            try!(writer.write_u32::<LittleEndian>(first_script_offset + (i * 8)));
        }
        // Write script data
        for s in &self.scripts {
            // We'll write this later
            try!(writer.seek(io::SeekFrom::Current(4)));
            try!(writer.write_u32::<LittleEndian>(s.unknown));
        }
        Ok(())
    }
}
