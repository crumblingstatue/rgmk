use std::io::prelude::*;
use std::io;
use byteorder::{self, ReadBytesExt, WriteBytesExt, LittleEndian};
use {GameDataRead, GameDataWrite, Strings};
use gamedata_io::{Chunk, get_chunk_header, ReadError, CHUNK_HEADER_LEN};

impl<'a> Chunk<'a> for Strings {
    const TYPE_ID: &'static [u8; 4] = b"STRG";
    type ReadOutput = (Self, Vec<u32>);
    type WriteInput = u32;
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        try!(get_chunk_header(reader, Self::TYPE_ID));
        let count = try!(reader.read_u32::<LittleEndian>());
        let mut offsets = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let offset = try!(reader.read_u32::<LittleEndian>());
            offsets.push(offset);
        }
        let mut strings = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let string = try!(read_string(reader));
            strings.push(string);
        }
        // TODO: Why do we need to consume additional 4 bytes?
        // Looks like 4 zero bytes.
        let mut buf = [0u8; 4];
        try!(reader.read_exact(&mut buf));
        Ok((Strings { strings: strings }, offsets))
    }
    fn write<W: GameDataWrite>(&self, writer: &mut W, offset: u32) -> io::Result<()> {
        try!(writer.write_all(Self::TYPE_ID));
        try!(writer.write_u32::<LittleEndian>(self.content_size()));
        try!(writer.write_u32::<LittleEndian>(self.strings.len() as u32));
        let mut string_offset = offset + CHUNK_HEADER_LEN + 4 + (self.strings.len() as u32 * 4);
        for string in &self.strings {
            try!(writer.write_u32::<LittleEndian>(string_offset));
            string_offset += (string.len() + 1) as u32 + 4;
        }
        for string in &self.strings {
            try!(writer.write_u32::<LittleEndian>(string.len() as u32));
            try!(writer.write_all(string.as_bytes()));
            try!(writer.write_u8(0));
        }
        // Required padding
        try!(writer.write_all(&[0u8; 4]));
        Ok(())
    }
    fn content_size(&self) -> u32 {
        let mut lengths = 0;
        for s in &self.strings {
            // The length denominator before the string
            lengths += 4;
            // + 1 for null terminator
            lengths += s.len() + 1;
        }
        // +4 at end for zero padding
        (4 + (self.strings.len() * 4) + lengths + 4) as u32
    }
}

fn read_string<R: GameDataRead>(reader: &mut R) -> Result<String, StringReadError> {
    let len = try!(reader.read_u32::<LittleEndian>());
    let mut buf = Vec::with_capacity(len as usize);
    unsafe {
        buf.set_len(len as usize);
        try!(reader.read_exact(&mut buf));
    }
    let terminator = try!(reader.read_u8());
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
