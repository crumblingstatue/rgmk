use std::io::prelude::*;
use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian, BigEndian};
use {GameDataRead, GameDataWrite, Texture, Textures};
use gamedata_io::{Chunk, get_chunk_header, ReadError, read_into_byte_vec, Tell};

const IMAGE_DATA_ALIGNMENT: u32 = 128;

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
            let reader_offset = try!(reader.seek(io::SeekFrom::Current(0)));
            try!(reader.seek(io::SeekFrom::Start(offset as u64)));
            assert!(offset % IMAGE_DATA_ALIGNMENT == 0,
                    "Image data is assumed to be aligned on {} byte boundaries", IMAGE_DATA_ALIGNMENT);
            let png = try!(read_png(reader));
            try!(reader.seek(io::SeekFrom::Start(reader_offset)));
            trace!("unk: {}, offset: {}", unk, offset - data_offset);
            textures.push(Texture {
                unknown: unk,
                png_data: png,
            });
        }
        let rel_offset = try!(reader.tell()) - start_offset;
        let data = try!(read_into_byte_vec(reader, header.size - rel_offset as usize));
        Ok(Textures {
            textures: textures,
        })
    }
    chunk_write_impl!();
    fn write_content<W: GameDataWrite>(&self, writer: &mut W, _input: ()) -> io::Result<()> {
        try!(writer.write_u32::<LittleEndian>(self.textures.len() as u32));
        let start_offset = try!(writer.seek(io::SeekFrom::Current(0)));
        let start_offset = try!(writer.tell());
        let num_textures = self.textures.len() as u32;
        let offset_table_len = num_textures * 4;
        let fileinfo_table_len = num_textures * 8;
        let mut texture_data_offsets: Vec<u32> = Vec::with_capacity(num_textures as usize);
        // Skip offset table and write image data first
        try!(writer.seek(io::SeekFrom::Current((offset_table_len + fileinfo_table_len) as i64)));
        for t in &self.textures {
            try!(writer.write_all(&t.png_data));
            let mut offset = try!(writer.seek(io::SeekFrom::Current(0)));
            while offset % IMAGE_DATA_ALIGNMENT as u64 != 0 {
                offset += 1;
            }
            texture_data_offsets.push(offset as u32);
            try!(writer.seek(io::SeekFrom::Start(offset)));
        }
        // Write offset table
        for i in 0..num_textures {
            try!(writer.write_u32::<LittleEndian>(start_offset as u32 + offset_table_len +
                                                  (i * 8)));
        }
        let writer_offset = try!(writer.tell());
        let texture_data_offset = writer_offset as u32 + (num_textures * 8);
        for (t, &off) in self.textures.iter().zip(texture_data_offsets.iter()) {
            try!(writer.write_u32::<LittleEndian>(t.unknown));
            try!(writer.write_u32::<LittleEndian>(off));
        }
        Ok(())
    }
    fn content_size(&self) -> u32 {
        let num_textures = self.textures.len();
        let num_textures_size = 4;
        let texture_offsets_size = num_textures * 4;
        let texture_entries_size = num_textures * 8;
        //(num_textures_size + texture_offsets_size + texture_entries_size +
        // self.texture_data.len()) as u32
        unimplemented!()
    }
}

fn png_length<R: GameDataRead>(reader: &mut R) -> Result<u32, ReadError> {
    const MAGIC: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let reader_start = try!(reader.seek(io::SeekFrom::Current(0)));
    let mut buf = [0u8; 8];
    try!(reader.read_exact(&mut buf));
    assert_eq!(buf, MAGIC);
    loop {
        let length = try!(reader.read_u32::<BigEndian>());
        let mut chunk_type = [0u8; 4];
        try!(reader.read_exact(&mut chunk_type));
        let crc_len = 4;
        try!(reader.seek(io::SeekFrom::Current(length as i64 + crc_len)));
        if &chunk_type == b"IEND" {
            break;
        }
    }
    let reader_end = try!(reader.seek(io::SeekFrom::Current(0)));
    let length = reader_end - reader_start;
    try!(reader.seek(io::SeekFrom::Start(reader_start)));
    trace!("Length of PNG is {}", length);
    Ok(length as u32)
}

fn read_png<R: GameDataRead>(reader: &mut R) -> Result<Vec<u8>, ReadError> {
    let length = try!(png_length(reader));
    let buf = try!(read_into_byte_vec(reader, length as usize));
    Ok(buf)
}
