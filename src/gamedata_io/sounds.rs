use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use {GameDataRead, GameDataWrite, Sounds, Sound};
use gamedata_io::{Chunk, get_chunk_header, ReadError, Tell};

pub struct Offsets {
    pub name_offset: u32,
    pub ext_offset: u32,
    pub filename_offset: u32,
}

pub(crate) fn write_offsets<W: GameDataWrite>(sounds: &Sounds,
                                              writer: &mut W,
                                              string_offsets: &[u32])
                                              -> io::Result<()> {
    try!(writer.seek(io::SeekFrom::Current(4 + (sounds.sounds.len() * 4) as i64)));
    for s in &sounds.sounds {
        try!(writer.write_u32::<LittleEndian>(string_offsets[s.name_index]));
        try!(writer.seek(io::SeekFrom::Current(4)));
        try!(writer.write_u32::<LittleEndian>(string_offsets[s.ext_index]));
        try!(writer.write_u32::<LittleEndian>(string_offsets[s.filename_index]));
        try!(writer.seek(io::SeekFrom::Current(5 * 4)));
    }
    Ok(())
}

impl<'a> Chunk<'a> for Sounds {
    const TYPE_ID: &'static [u8; 4] = b"SOND";
    type ReadOutput = (Self, Vec<Offsets>);
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        try!(get_chunk_header(reader, Self::TYPE_ID));
        let num_sounds = try!(reader.read_u32::<LittleEndian>());
        trace!("{} sounds", num_sounds);
        // Read sound entry offsets
        for _ in 0..num_sounds {
            // For now just discard them
            try!(reader.read_u32::<LittleEndian>());
        }
        // Read sound entries
        let mut sounds = Vec::new();
        let mut offsets = Vec::new();
        for _ in 0..num_sounds {
            let name_offset = try!(reader.read_u32::<LittleEndian>());
            let unk1 = try!(reader.read_u32::<LittleEndian>());
            let ext_offset = try!(reader.read_u32::<LittleEndian>());
            let filename_offset = try!(reader.read_u32::<LittleEndian>());
            let unk2 = try!(reader.read_u32::<LittleEndian>());
            let unk3 = try!(reader.read_u32::<LittleEndian>());
            let unk4 = try!(reader.read_u32::<LittleEndian>());
            let unk5 = try!(reader.read_u32::<LittleEndian>());
            let unk6 = try!(reader.read_u32::<LittleEndian>());
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
        try!(writer.write_u32::<LittleEndian>(num_sounds));
        let writer_offset = try!(writer.tell()) as u32;
        let sound_data_offset = writer_offset + (num_sounds * 4);
        for i in 0..num_sounds {
            try!(writer.write_u32::<LittleEndian>(sound_data_offset + (i * (9 * 4))));
        }
        for s in &self.sounds {
            // We'll write this later
            try!(writer.seek(io::SeekFrom::Current(4)));
            try!(writer.write_u32::<LittleEndian>(s.unk1));
            // We'll write these later
            try!(writer.seek(io::SeekFrom::Current(8)));
            try!(writer.write_u32::<LittleEndian>(s.unk2));
            try!(writer.write_u32::<LittleEndian>(s.unk3));
            try!(writer.write_u32::<LittleEndian>(s.unk4));
            try!(writer.write_u32::<LittleEndian>(s.unk5));
            try!(writer.write_u32::<LittleEndian>(s.unk6));
        }
        Ok(())
    }
}
