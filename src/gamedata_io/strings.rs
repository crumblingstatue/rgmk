use std::io;
use byteorder::{self, ReadBytesExt, WriteBytesExt, LittleEndian};
use {GameDataRead, GameDataWrite, Strings};
use gamedata_io::{Chunk, get_chunk_header, ReadError, Tell};

impl<'a> Chunk<'a> for Strings {
    const TYPE_ID: &'static [u8; 4] = b"STRG";
    type ReadOutput = (Self, Vec<u32>);
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        get_chunk_header(reader, Self::TYPE_ID)?;
        let count = reader.read_u32::<LittleEndian>()?;
        let mut offsets = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let offset = reader.read_u32::<LittleEndian>()?;
            offsets.push(offset);
        }
        let mut strings = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let string = read_string(reader)?;
            strings.push(string);
        }
        // TODO: Why do we need to consume additional 4 bytes?
        // Looks like 4 zero bytes.
        // Okay, let's assume that this is zero padding for 16 byte alignment.
        let finished_offset = reader.tell()?;
        let mut offset = finished_offset;
        // Seek to nearest 16 byte aligned offset
        while offset % 16 != 0 {
            offset += 1;
        }
        reader.seek(io::SeekFrom::Start(offset))?;
        Ok((Strings { strings: strings }, offsets))
    }
    fn write_content<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32::<LittleEndian>(self.strings.len() as u32)?;
        let start_offset = writer.tell()?;
        let mut string_offset = start_offset as u32 + (self.strings.len() as u32 * 4);
        for string in &self.strings {
            writer.write_u32::<LittleEndian>(string_offset)?;
            string_offset += (string.len() + 1) as u32 + 4;
        }
        for string in &self.strings {
            writer.write_u32::<LittleEndian>(string.len() as u32)?;
            writer.write_all(string.as_bytes())?;
            writer.write_u8(0)?;
        }
        // Write zero padding for 16 byte alignment
        let finished_offset = writer.tell()?;
        let mut offset = finished_offset;
        while offset % 16 != 0 {
            offset += 1;
            writer.write_u8(0)?;
        }
        Ok(())
    }
}

fn read_string<R: GameDataRead>(reader: &mut R) -> Result<String, StringReadError> {
    let len = reader.read_u32::<LittleEndian>()?;
    let buf = super::read_into_byte_vec(reader, len as usize)?;
    let terminator = reader.read_u8()?;
    if terminator == 0 {
        // We assume strings are valid UTF-8, if not, panic.
        Ok(String::from_utf8(buf).unwrap())
    } else {
        Err(StringReadError::MissingNullTerminator)
    }
}

quick_error! {
    #[derive(Debug)]
    /// Error when reading a string from the string table.
    pub enum StringReadError {
        /// An I/O error.
        Io(err: io::Error) {
            from()
        }
        /// A byte order error.
        ByteOrder(err: byteorder::Error) {
            from()
        }
        /// Missing null terminator.
        MissingNullTerminator {}
    }
}
