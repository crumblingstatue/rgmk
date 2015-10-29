use std::io::prelude::*;
use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use {GameDataRead, GameDataWrite, Sprite, Sprites};
use gamedata_io::{Chunk, get_chunk_header, ReadError, read_into_byte_vec};

impl<'a> Chunk<'a> for Sprites {
    const TYPE_ID: &'static [u8; 4] = b"SPRT";
    type ReadOutput = (Self, Vec<u32>);
    type WriteInput = &'a [u32];
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        let header = try!(get_chunk_header(reader, Self::TYPE_ID));
        let count = try!(reader.read_u32::<LittleEndian>());
        let start_offset = try!(reader.seek(io::SeekFrom::Current(0))) as usize;
        trace!("{} sprites", count);
        // Read offset table
        let mut sprite_offsets = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let offset = try!(reader.read_u32::<LittleEndian>());
            sprite_offsets.push(offset as usize);
        }
        let mut name_offsets = Vec::with_capacity(count as usize);
        let mut sprites = Vec::with_capacity(count as usize);
        let mut sprite_offsets = sprite_offsets.into_iter().peekable();
        while let Some(reader_offset) = sprite_offsets.next() {
            let reader_offset =
                try!(reader.seek(io::SeekFrom::Start(reader_offset as u64))) as usize;
            trace!("Reading sprite from offset {}", reader_offset);
            let name_offset = try!(reader.read_u32::<LittleEndian>());
            name_offsets.push(name_offset);
            let width = try!(reader.read_u32::<LittleEndian>());
            let height = try!(reader.read_u32::<LittleEndian>());
            trace!("name: {} w: {} h: {}", name_offset, width, height);
            let reader_offset = try!(reader.seek(io::SeekFrom::Current(0))) as usize;
            let next_offset = *sprite_offsets.peek().unwrap_or(&((start_offset + header.size) - 4));
            let remaining = next_offset - reader_offset;
            trace!("At {}, Next offset is {}, reading remaining {} bytes",
                   reader_offset,
                   next_offset,
                   remaining);
            let data = try!(read_into_byte_vec(reader, remaining));
            sprites.push(Sprite {
                name_index: 0,
                width: width,
                height: height,
                unknown: data,
            });
        }
        Ok((Sprites { sprites: sprites }, name_offsets))
    }
    fn write<W: GameDataWrite>(&self, writer: &mut W, string_offsets: &'a [u32]) -> io::Result<()> {
        try!(writer.write_all(Self::TYPE_ID));
        try!(writer.write_u32::<LittleEndian>(self.content_size()));
        let count = self.sprites.len() as u32;
        try!(writer.write_u32::<LittleEndian>(count));
        let mut offset = try!(writer.seek(io::SeekFrom::Current(0))) as u32 + (count * 4);
        // Write offset table
        for s in &self.sprites {
            try!(writer.write_u32::<LittleEndian>(offset));
            offset += (3 * 4) + s.unknown.len() as u32;
        }
        // Write sprites
        for s in &self.sprites {
            try!(writer.write_u32::<LittleEndian>(string_offsets[s.name_index]));
            try!(writer.write_u32::<LittleEndian>(s.width));
            try!(writer.write_u32::<LittleEndian>(s.height));
            try!(writer.write_all(&s.unknown));
        }
        Ok(())
    }
    fn content_size(&self) -> u32 {
        let count = self.sprites.len() as u32;
        let count_size = 4;
        let offsets_size = count * 4;
        let sprites_size = self.sprites.iter().fold(0, |a, s| a + (3 * 4) + s.unknown.len() as u32);
        count_size + offsets_size + sprites_size
    }
}
