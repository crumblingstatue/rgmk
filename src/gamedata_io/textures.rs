use std::io::prelude::*;
use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use {GameDataRead, GameDataWrite, Texture, Textures};
use gamedata_io::{Chunk, get_chunk_header, ReadError, read_into_byte_vec, Tell};

impl<'a> Chunk<'a> for Textures {
    const TYPE_ID: &'static [u8; 4] = b"TXTR";
    type ReadOutput = Self;
    type WriteInput = ();
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Textures, ReadError> {
        let header = try!(get_chunk_header(reader, Self::TYPE_ID));
        let start_offset = try!(reader.tell());
        let num_textures = try!(reader.read_u32::<LittleEndian>());
        trace!("{} textures", num_textures);
        // Read texture entry offsets
        for _ in 0..num_textures {
            // For now just discard them
            try!(reader.read_u32::<LittleEndian>());
        }
        let reader_offset = try!(reader.tell()) as u32;
        let data_offset = reader_offset + (num_textures * 8);
        let mut textures = Vec::new();
        for _ in 0..num_textures {
            let unk = try!(reader.read_u32::<LittleEndian>());
            let offset = try!(reader.read_u32::<LittleEndian>());
            trace!("unk: {}, offset: {}", unk, offset - data_offset);
            textures.push(Texture {
                unknown: unk,
                offset: offset - data_offset,
            });
        }
        let rel_offset = try!(reader.tell()) - start_offset;
        let data = try!(read_into_byte_vec(reader, header.size - rel_offset as usize));
        Ok(Textures {
            textures: textures,
            texture_data: data,
        })
    }
    chunk_write_impl!();
    fn write_content<W: GameDataWrite>(&self, writer: &mut W, _input: ()) -> io::Result<()> {
        try!(writer.write_u32::<LittleEndian>(self.textures.len() as u32));
        let writer_offset = try!(writer.tell());
        let num_textures = self.textures.len() as u32;
        // Write offset table
        for i in 0..num_textures {
            let offset_table_len = num_textures * 4;
            try!(writer.write_u32::<LittleEndian>(writer_offset as u32 + offset_table_len +
                                                  (i * 8)));
        }
        let writer_offset = try!(writer.tell());
        let texture_data_offset = writer_offset as u32 + (num_textures * 8);
        for t in &self.textures {
            try!(writer.write_u32::<LittleEndian>(t.unknown));
            try!(writer.write_u32::<LittleEndian>(texture_data_offset + t.offset));
        }
        try!(writer.write_all(&self.texture_data));
        Ok(())
    }
    fn content_size(&self) -> u32 {
        let num_textures = self.textures.len();
        let num_textures_size = 4;
        let texture_offsets_size = num_textures * 4;
        let texture_entries_size = num_textures * 8;
        (num_textures_size + texture_offsets_size + texture_entries_size +
         self.texture_data.len()) as u32
    }
}
