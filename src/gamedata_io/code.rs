use std::io::prelude::*;
use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use {GameDataRead, GameDataWrite, Code, CodeChunk};
use gamedata_io::{Chunk, get_chunk_header, ReadError, read_into_byte_vec, Tell};

pub fn write_offsets<W: GameDataWrite>(code: &Code,
                                       writer: &mut W,
                                       string_offsets: &[u32])
                                       -> io::Result<()> {
    // Skip num_codes + offset table
    try!(writer.seek(io::SeekFrom::Current(code.code_chunks.len() as i64 * 4 + 4)));
    for chunk in &code.code_chunks {
        try!(writer.write_u32::<LittleEndian>(string_offsets[chunk.name_index]));
        // Skip code size + data
        try!(writer.seek(io::SeekFrom::Current(4 + chunk.raw_code.len() as i64)));
    }
    Ok(())
}

impl<'a> Chunk<'a> for Code {
    const TYPE_ID: &'static [u8; 4] = b"CODE";
    type ReadOutput = (Self, Vec<u32>);
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        try!(get_chunk_header(reader, Self::TYPE_ID));
        let num_codes = try!(reader.read_u32::<LittleEndian>());
        trace!("{} codes", num_codes);
        // Skip offset table
        try!(reader.seek(io::SeekFrom::Current(num_codes as i64 * 4)));
        // Read chunks
        let mut code_chunks = Vec::with_capacity(num_codes as usize);
        let mut name_offsets = Vec::with_capacity(num_codes as usize);
        for _ in 0..num_codes {
            let name_offset = try!(reader.read_u32::<LittleEndian>());
            name_offsets.push(name_offset);
            let size = try!(reader.read_u32::<LittleEndian>());
            let code_data = try!(read_into_byte_vec(reader, size as usize));
            code_chunks.push(CodeChunk {
                name_index: 0,
                raw_code: code_data,
            })
        }
        Ok((Code { code_chunks: code_chunks }, name_offsets))
    }
    fn write_content<W: GameDataWrite>(&self, writer: &mut W) -> Result<(), io::Error> {
        let num_chunks = self.code_chunks.len() as u32;
        try!(writer.write_u32::<LittleEndian>(num_chunks));
        // Skip offset table, we'll write it later
        let offset_table_size = num_chunks * 4;
        let offset_table_pos = try!(writer.tell());
        try!(writer.seek(io::SeekFrom::Current(offset_table_size as i64)));
        // Write the chunk offsets and the chunks simultaneously
        for (i, chunk) in self.code_chunks.iter().enumerate() {
            // Write the offset
            let offset = try!(writer.tell());
            try!(writer.seek(io::SeekFrom::Start(offset_table_pos + (i as u64 * 4))));
            try!(writer.write_u32::<LittleEndian>(offset as u32));
            try!(writer.seek(io::SeekFrom::Start(offset)));
            // Skip the name offset, we'll write it later
            try!(writer.seek(io::SeekFrom::Current(4)));
            // Write the size of the code data
            try!(writer.write_u32::<LittleEndian>(chunk.raw_code.len() as u32));
            // Write the raw code data
            try!(writer.write_all(&chunk.raw_code));
        }
        trace!("Finished writing code data, offset is {}",
               try!(writer.tell()));
        Ok(())
    }
    chunk_write_impl!();
}