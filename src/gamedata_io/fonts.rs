use std::io::prelude::*;
use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use {GameDataRead, GameDataWrite, Font, Fonts};
use gamedata_io::{Chunk, get_chunk_header, ReadError, read_into_byte_vec, Tell};

pub struct Offset {
    pub name: u32,
    pub font_name: u32,
}

impl<'a> Chunk<'a> for Fonts {
    const TYPE_ID: &'static [u8; 4] = b"FONT";
    type ReadOutput = (Self, Vec<Offset>);
    type WriteInput = &'a [u32];
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        let header = try!(get_chunk_header(reader, Self::TYPE_ID));
        let count = try!(reader.read_u32::<LittleEndian>());
        let start_offset = try!(reader.tell()) as usize;
        trace!("{} fonts", count);
        // Read offset table. Seem to be separated by 1964.
        for _ in 0..count {
            try!(reader.read_u32::<LittleEndian>());
        }
        let mut string_offsets = Vec::with_capacity(count as usize);
        let mut fonts = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let name_offset = try!(reader.read_u32::<LittleEndian>());
            let font_name_offset = try!(reader.read_u32::<LittleEndian>());
            string_offsets.push(Offset {
                name: name_offset,
                font_name: font_name_offset,
            });
            let point_size = try!(reader.read_u32::<LittleEndian>());
            let data = try!(read_into_byte_vec(reader, 1952));
            fonts.push(Font {
                name_index: 0,
                font_name_index: 0,
                point_size: point_size,
                data: data,
            });
            trace!("name: {} font_name: {}", name_offset, font_name_offset);
        }
        let end_offset = try!(reader.tell()) as usize;
        let rel_offset = end_offset - start_offset;
        trace!("Relative offset: {}", rel_offset);
        // -4 for some reason, or else I'll read past the chunk
        let need_to_read = header.size - rel_offset - 4;
        trace!("Need to read {} bytes.", need_to_read);
        let unknown_data = try!(read_into_byte_vec(reader, need_to_read));
        let end_offset = try!(reader.tell()) as usize;
        let rel_offset = end_offset - start_offset;
        trace!("Size: {} Offset: {}", header.size, rel_offset);
        trace!("Absolute offset: {}", end_offset);
        Ok((Fonts {
            fonts: fonts,
            unknown: unknown_data,
        },
            string_offsets))
    }
    fn write_content<W: GameDataWrite>(&self,
                                       writer: &mut W,
                                       string_offsets: &'a [u32])
                                       -> io::Result<()> {
        try!(writer.write_u32::<LittleEndian>(self.fonts.len() as u32));
        let write_offset = try!(writer.tell()) as u32;
        let count = self.fonts.len() as u32;
        for i in 0..count {
            try!(writer.write_u32::<LittleEndian>(write_offset + (count * 4) + (i * 1964)));
        }
        for f in &self.fonts {
            try!(writer.write_u32::<LittleEndian>(string_offsets[f.name_index]));
            try!(writer.write_u32::<LittleEndian>(string_offsets[f.font_name_index]));
            try!(writer.write_u32::<LittleEndian>(f.point_size));
            try!(writer.write_all(&f.data));
        }
        try!(writer.write_all(&self.unknown));
        Ok(())
    }
    chunk_write_impl!();
    fn content_size(&self) -> u32 {
        let count = self.fonts.len() as u32;
        let count_size = 4;
        let offsets_size = count * 4;
        let fonts_size = count * 1964;
        count_size + offsets_size + fonts_size + self.unknown.len() as u32
    }
}
