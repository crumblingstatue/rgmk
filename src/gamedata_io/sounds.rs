use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use {GameDataRead, GameDataWrite, Sounds, Sound};
use gamedata_io::{Chunk, get_chunk_header, ReadError, Tell};

pub struct Offsets {
    pub name_offset: u32,
    pub ext_offset: u32,
    pub filename_offset: u32,
}

pub(super) fn write_offsets<W: GameDataWrite>(sounds: &Sounds,
                                              writer: &mut W,
                                              string_offsets: &[u32])
                                              -> io::Result<()> {
    writer.seek(io::SeekFrom::Current(4 + (sounds.sounds.len() * 4) as i64))?;
    for s in &sounds.sounds {
        writer.write_u32::<LittleEndian>(string_offsets[s.name_index])?;
        writer.seek(io::SeekFrom::Current(4))?;
        writer.write_u32::<LittleEndian>(string_offsets[s.ext_index])?;
        writer.write_u32::<LittleEndian>(string_offsets[s.filename_index])?;
        writer.seek(io::SeekFrom::Current(5 * 4))?;
    }
    Ok(())
}

impl<'a> Chunk<'a> for Sounds {
    const TYPE_ID: &'static [u8; 4] = b"SOND";
    type ReadOutput = (Self, Vec<Offsets>);
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        get_chunk_header(reader, Self::TYPE_ID)?;
        let num_sounds = reader.read_u32::<LittleEndian>()?;
        trace!("{} sounds", num_sounds);
        // Read sound entry offsets
        for _ in 0..num_sounds {
            // For now just discard them
            reader.read_u32::<LittleEndian>()?;
        }
        // Read sound entries
        let mut sounds = Vec::new();
        let mut offsets = Vec::new();
        for _ in 0..num_sounds {
            let name_offset = reader.read_u32::<LittleEndian>()?;
            let unk1 = reader.read_u32::<LittleEndian>()?;
            let ext_offset = reader.read_u32::<LittleEndian>()?;
            let filename_offset = reader.read_u32::<LittleEndian>()?;
            let unk2 = reader.read_u32::<LittleEndian>()?;
            let unk3 = reader.read_u32::<LittleEndian>()?;
            let unk4 = reader.read_u32::<LittleEndian>()?;
            let unk5 = reader.read_u32::<LittleEndian>()?;
            let unk6 = reader.read_u32::<LittleEndian>()?;
            sounds.push(Sound {
                name_index: 0,
                unk1: unk1,
                ext_index: 0,
                filename_index: 0,
                unk2: unk2,
                unk3: unk3,
                unk4: unk4,
                unk5: unk5,
                unk6: unk6,
            });
            offsets.push(Offsets {
                name_offset: name_offset,
                ext_offset: ext_offset,
                filename_offset: filename_offset,
            });
        }
        Ok((Sounds { sounds: sounds }, offsets))
    }
    fn write_content<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
        let num_sounds = self.sounds.len() as u32;
        writer.write_u32::<LittleEndian>(num_sounds)?;
        let writer_offset = writer.tell()? as u32;
        let sound_data_offset = writer_offset + (num_sounds * 4);
        for i in 0..num_sounds {
            writer.write_u32::<LittleEndian>(sound_data_offset + (i * (9 * 4)))?;
        }
        for s in &self.sounds {
            // We'll write this later
            writer.seek(io::SeekFrom::Current(4))?;
            writer.write_u32::<LittleEndian>(s.unk1)?;
            // We'll write these later
            writer.seek(io::SeekFrom::Current(8))?;
            writer.write_u32::<LittleEndian>(s.unk2)?;
            writer.write_u32::<LittleEndian>(s.unk3)?;
            writer.write_u32::<LittleEndian>(s.unk4)?;
            writer.write_u32::<LittleEndian>(s.unk5)?;
            writer.write_u32::<LittleEndian>(s.unk6)?;
        }
        Ok(())
    }
}
