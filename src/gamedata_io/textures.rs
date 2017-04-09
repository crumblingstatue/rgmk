use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian, BigEndian};
use {GameDataRead, GameDataWrite, Texture, Textures};
use gamedata_io::{Chunk, get_chunk_header, ReadError, read_into_byte_vec, Tell};

const IMAGE_DATA_ALIGNMENT: u32 = 128;

impl<'a> Chunk<'a> for Textures {
    const TYPE_ID: &'static [u8; 4] = b"TXTR";
    type ReadOutput = Self;
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Textures, ReadError> {
        get_chunk_header(reader, Self::TYPE_ID)?;
        let num_textures = reader.read_u32::<LittleEndian>()?;
        trace!("{} textures", num_textures);
        // Read texture entry offsets
        for _ in 0..num_textures {
            // For now just discard them
            reader.read_u32::<LittleEndian>()?;
        }
        let mut textures = Vec::new();
        let mut finished_offset = 0;
        for i in 0..num_textures {
            let unk = reader.read_u32::<LittleEndian>()?;
            let offset = reader.read_u32::<LittleEndian>()?;
            let reader_offset = reader.tell()?;
            reader.seek(io::SeekFrom::Start(offset as u64))?;
            assert_eq!(offset % IMAGE_DATA_ALIGNMENT,
                       0,
                       "Image data is assumed to be aligned on {} byte boundaries",
                       IMAGE_DATA_ALIGNMENT);
            trace!("Reading image data {} @ {}", i, offset);
            let png = read_png(reader)?;
            finished_offset = reader.tell()?;
            reader.seek(io::SeekFrom::Start(reader_offset))?;
            textures.push(Texture {
                              unknown: unk,
                              png_data: png,
                          });
        }
        // Looks like chunks don't use the same alignment as image data.
        // Or maybe they don't use alignment at all?
        // Why the zero padding then?
        while finished_offset % 16 != 0 {
            finished_offset += 1;
        }
        reader.seek(io::SeekFrom::Start(finished_offset))?;
        Ok(Textures { textures: textures })
    }
    fn write_content<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32::<LittleEndian>(self.textures.len() as u32)?;
        let start_offset = writer.tell()?;
        let num_textures = self.textures.len() as u32;
        let offset_table_len = num_textures * 4;
        let fileinfo_table_len = num_textures * 8;
        let mut texture_data_offsets: Vec<u32> = Vec::with_capacity(num_textures as usize);
        let offset_table_offset = writer.tell()?;
        // Skip offset table and fileinfos and write image data first
        writer.seek(io::SeekFrom::Current((offset_table_len + fileinfo_table_len) as i64))?;
        for (i, t) in self.textures.iter().enumerate() {
            let mut offset = writer.tell()?;
            while offset % IMAGE_DATA_ALIGNMENT as u64 != 0 {
                offset += 1;
            }
            writer.seek(io::SeekFrom::Start(offset))?;
            trace!("Writing image data {} @ {}", i, offset);
            writer.write_all(&t.png_data)?;
            texture_data_offsets.push(offset as u32);
        }
        let finished_offset = writer.tell()?;
        // Go back and write offset table
        writer.seek(io::SeekFrom::Start(offset_table_offset))?;
        for i in 0..num_textures {
            writer.write_u32::<LittleEndian>(start_offset as u32 + offset_table_len + (i * 8))?;
        }
        // Write fileinfos
        for (t, &off) in self.textures.iter().zip(texture_data_offsets.iter()) {
            writer.write_u32::<LittleEndian>(t.unknown)?;
            writer.write_u32::<LittleEndian>(off)?;
        }
        writer.seek(io::SeekFrom::Start(finished_offset))?;
        trace!("Finished at {}", finished_offset);
        let mut offset = finished_offset;
        // Write zero padding
        while offset % 16 != 0 {
            offset += 1;
            writer.write_u8(0)?;
        }
        Ok(())
    }
}

fn png_length<R: GameDataRead>(reader: &mut R) -> Result<u32, ReadError> {
    const MAGIC: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let reader_start = reader.seek(io::SeekFrom::Current(0))?;
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    assert_eq!(buf, MAGIC);
    loop {
        let length = reader.read_u32::<BigEndian>()?;
        let mut chunk_type = [0u8; 4];
        reader.read_exact(&mut chunk_type)?;
        let crc_len = 4;
        reader.seek(io::SeekFrom::Current(length as i64 + crc_len))?;
        if chunk_type == *b"IEND" {
            break;
        }
    }
    let reader_end = reader.seek(io::SeekFrom::Current(0))?;
    let length = reader_end - reader_start;
    reader.seek(io::SeekFrom::Start(reader_start))?;
    trace!("Length of PNG is {}", length);
    Ok(length as u32)
}

fn read_png<R: GameDataRead>(reader: &mut R) -> Result<Vec<u8>, ReadError> {
    let length = png_length(reader)?;
    let buf = read_into_byte_vec(reader, length as usize)?;
    Ok(buf)
}
