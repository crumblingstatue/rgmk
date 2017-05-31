use super::*;
use super::LittleEndian as LE;
use byteorder::BigEndian as BE;

const TYPE_ID: &'static [u8; 4] = b"TXTR";

pub fn read<R: GameDataRead>(reader: &mut R) -> Result<Txtr, Box<Error>> {
    let size = expect_chunk(reader, TYPE_ID)?;
    println!("Read size: {}", size);
    let begin = reader.seek(SeekFrom::Current(0))?;
    let num_textures = reader.read_u32::<LE>()?;
    // TODO: Here we assume these offsets are useless, because texture entries
    // are laid out sequentially right after these offsets.
    // This might be a wrong assumption.
    for _ in 0..num_textures {
        let _offset = reader.read_u32::<LE>()?;
    }
    let mut textures = Vec::with_capacity(num_textures as usize);
    for _ in 0..num_textures {
        println!(
            "Read texture entry @ {}",
            reader.seek(SeekFrom::Current(0))?
        );
        let unknown = reader.read_u32::<LE>()?;
        let offset = reader.read_u32::<LE>()?;
        println!("Read texture offset: {}", offset);
        textures.push(Texture {
            unknown,
            source: TextureSource::Original { offset: offset.into() },
        });
    }
    let end = reader.seek(SeekFrom::Current(0))?;
    let total_read = end - begin;
    let true_end = reader.seek(
        SeekFrom::Current((size as u64 - total_read) as i64),
    )?;
    Ok(Txtr {
        textures,
        end_offset: true_end,
    })
}

pub fn write<W: GameDataWrite, R: GameDataRead>(
    txtr: &Txtr,
    writer: &mut W,
    reader_orig: &mut R,
) -> io::Result<()> {
    writer.write_all(TYPE_ID)?;
    // Write size of chunk later
    let after_size_pos = writer.seek(SeekFrom::Current(4))?;
    let len = txtr.textures.len();
    // Write number of textures
    writer.write_u32::<LE>(len as u32)?;
    // Skip writing the offsets
    let after_offset_pos = writer.seek(SeekFrom::Current((len * 4) as i64))?;
    let mut offsets = Vec::with_capacity(len);
    for tex in &txtr.textures {
        println!(
            "Write texture entry @ {}",
            writer.seek(SeekFrom::Current(0))?
        );
        offsets.push(writer.seek(SeekFrom::Current(0))?);
        writer.write_u32::<LE>(tex.unknown)?;
        let data_offset_pos = writer.seek(SeekFrom::Current(0))?;
        // Skip over data offset, write it later.
        writer.seek(SeekFrom::Current(4))?;
        let before_write_src = writer.seek(SeekFrom::Current(0))?;
        let data_offset = write_texture_source(&tex.source, writer, reader_orig)?;
        writer.seek(SeekFrom::Start(before_write_src))?;
        writer.seek(SeekFrom::Start(data_offset_pos))?;
        writer.write_u32::<LE>(data_offset as u32)?;
        writer.seek(SeekFrom::Start(before_write_src))?;
    }
    let end_pos = writer.seek(SeekFrom::Start(txtr.end_offset))?;
    let size = end_pos - after_size_pos;
    println!("Write size: {}", size);
    writer.seek(SeekFrom::Start(after_size_pos - 4))?;
    writer.write_u32::<LE>(size as u32)?;
    writer.seek(
        SeekFrom::Start(after_offset_pos - (len * 4) as u64),
    )?;
    for &offset in &offsets {
        writer.write_u32::<LE>(offset as u32)?;
    }
    writer.seek(SeekFrom::Start(txtr.end_offset))?;
    Ok(())
}

pub fn write_texture_source<W: GameDataWrite, R: GameDataRead>(
    source: &TextureSource,
    writer: &mut W,
    reader_orig: &mut R,
) -> io::Result<u64> {
    match *source {
        TextureSource::Original { offset } => {
            let offset = reader_orig.seek(SeekFrom::Start(offset))?;
            println!("Writing texture data at offset {}", offset);
            let len = png_length(reader_orig)?;
            writer.seek(SeekFrom::Start(offset))?;
            io::copy(&mut reader_orig.take(len.into()), writer)?;
            Ok(offset)
        }
    }
}

fn png_length<R: GameDataRead>(reader: &mut R) -> Result<u32, io::Error> {
    const MAGIC: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let reader_start = reader.seek(io::SeekFrom::Current(0))?;
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    assert_eq!(buf, MAGIC);
    loop {
        let length = reader.read_u32::<BE>()?;
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
    Ok(length as u32)
}
