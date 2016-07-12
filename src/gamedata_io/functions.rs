use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use {GameDataRead, GameDataWrite, Function, Functions};
use gamedata_io::{Chunk, get_chunk_header, ReadError};

pub(crate) fn write_offsets<W: GameDataWrite>(funs: &Functions,
                                              writer: &mut W,
                                              string_offsets: &[u32])
                                              -> io::Result<()> {
    for fun in &funs.functions {
        try!(writer.write_u32::<LittleEndian>(string_offsets[fun.name_index]));
        try!(writer.seek(io::SeekFrom::Current(2 * 4)));
    }
    Ok(())
}

impl<'a> Chunk<'a> for Functions {
    const TYPE_ID: &'static [u8; 4] = b"FUNC";
    type ReadOutput = (Self, Vec<u32>);
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
    fn write_content<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
        for fun in &self.functions {
            // We'll write this later
            try!(writer.seek(io::SeekFrom::Current(4)));
            try!(writer.write_u32::<LittleEndian>(fun.unknown));
            try!(writer.write_u32::<LittleEndian>(fun.code_offset));
        }
        Ok(())
    }
}
