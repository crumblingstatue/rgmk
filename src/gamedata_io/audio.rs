use std::io;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use {GameDataRead, GameDataWrite};
use unexposed::{Audio, AudioData};
use gamedata_io::{Chunk, ReadError, Tell, get_chunk_header, read_into_byte_vec};

impl<'a> Chunk<'a> for Audio {
    const TYPE_ID: &'static [u8; 4] = b"AUDO";
    type ReadOutput = Self;
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        try!(get_chunk_header(reader, Self::TYPE_ID));
        let num_audio = try!(reader.read_u32::<LittleEndian>());
        trace!("num audio: {}", num_audio);
        // Get offsets
        let mut offsets = Vec::new();
        let audio_data_offset = try!(reader.tell()) as u32 + (num_audio * 4);
        trace!("audio_data_offset is {} ", audio_data_offset);
        for _ in 0..num_audio {
            offsets.push(try!(reader.read_u32::<LittleEndian>()) - audio_data_offset);
        }
        // Get audio
        let mut audio = Vec::new();

        for (i, &offset) in offsets.iter().enumerate() {
            let offset =
                try!(reader.seek(io::SeekFrom::Start((audio_data_offset + offset) as u64)));
            let size = try!(reader.read_u32::<LittleEndian>());
            trace!("({}) @offset {} Reading audio file of size {}",
                   i,
                   offset,
                   size);
            audio.push(AudioData { data: try!(read_into_byte_vec(reader, size as usize)) });
        }
        Ok(Audio {
            audio: audio,
            offsets: offsets,
        })
    }
    fn write_content<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
        try!(writer.write_u32::<LittleEndian>(self.audio.len() as u32));
        let audio_data_offset = try!(writer.tell()) as u32 + (self.offsets.len() as u32 * 4);
        for &offset in &self.offsets {
            trace!("Writing offset {} ", audio_data_offset + offset);
            try!(writer.write_u32::<LittleEndian>(audio_data_offset + offset));
        }
        for (&offset, data) in self.offsets.iter().zip(self.audio.iter()) {
            try!(writer.seek(io::SeekFrom::Start((audio_data_offset + offset) as u64)));
            try!(writer.write_u32::<LittleEndian>(data.data.len() as u32));
            try!(writer.write_all(&data.data));
        }
        Ok(())
    }
}
