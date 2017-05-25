use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::error::Error;
use std::io::{self, SeekFrom};
use {GameDataRead, GameDataWrite};

// A couple assumptions are made about the format.
//
// - The root chunk is always a FORM chunk
//
// - One chunk type occurs exactly once.
//
// - Chunk order is important. We write back chunks in the same order we read them.
//
// - Chunks are aligned on even byte offsets. Padding bytes are always zero.

/// A pseudo IFF-like format that Game Maker Studio uses.
///
/// It doesn't actually conform to the IFF spec, so it doesn't make sense
/// to use a generic IFF library.
///
/// Instead, this type deals with the generic IFF-like top-level structure, so
/// we only have to deal with encoding/decoding the chunks we actually care about,
/// and errors in the handling of individual chunks don't break the rest of the chunks.
#[derive(Debug)]
pub struct PseudoIff {
    pub chunks: Vec<Chunk>,
}

#[derive(Debug)]
pub struct Chunk {
    pub type_id: TypeId,
    pub data: Vec<u8>,
}

type TypeId = [u8; 4];

impl PseudoIff {
    pub fn load<R: GameDataRead>(reader: &mut R) -> Result<Self, Box<Error>> {
        Ok(Self { chunks: read_form_chunk(reader)? })
    }
    pub fn save<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b"FORM")?;
        // Skip writing the FORM length, because we don't know it yet.
        let after_form_len = writer.seek(SeekFrom::Current(4))?;
        for chunk in &self.chunks {
            writer.write_all(&chunk.type_id[..])?;
            writer.write_i32::<LittleEndian>(chunk.data.len() as i32)?;
            writer.write_all(&chunk.data)?;
            // Chunks are aligned on even byte boundaries
            let pos = writer.seek(SeekFrom::Current(0))?;
            if pos % 2 != 0 {
                writer.write_u8(0)?;
            }
        }
        let end = writer.seek(SeekFrom::Current(0))?;
        let form_len = end - after_form_len;
        // Finally write the form length
        writer.seek(SeekFrom::Start(after_form_len - 4))?;
        writer.write_i32::<LittleEndian>(form_len as i32)?;
        Ok(())
    }
}

fn read_form_chunk<R: GameDataRead>(reader: &mut R) -> Result<Vec<Chunk>, Box<Error>> {
    let mut type_id = [0; 4];
    reader.read_exact(&mut type_id)?;
    let _len = reader.read_i32::<LittleEndian>();

    if &type_id[..] != b"FORM" {
        return Err(
            "Invalid data.win file. Must start with a FORM chunk."
                .into(),
        );
    }

    read_chunks(reader)
}

fn read_chunks<R: GameDataRead>(reader: &mut R) -> Result<Vec<Chunk>, Box<Error>> {
    let mut chunks = Vec::new();

    loop {
        match read_chunk(reader) {
            Ok(Some(chunk)) => {
                // Chunks are aligned on even byte boundaries
                let pos = reader.seek(SeekFrom::Current(0))?;
                if pos % 2 != 0 {
                    reader.seek(SeekFrom::Current(1))?;
                }
                chunks.push(chunk)
            }
            Ok(None) => return Ok(chunks),
            Err(e) => return Err(e),
        }
    }
}

fn read_chunk<R: GameDataRead>(reader: &mut R) -> Result<Option<Chunk>, Box<Error>> {
    use std::io::ErrorKind;

    let mut type_id = [0; 4];
    if let Err(e) = reader.read_exact(&mut type_id) {
        match e.kind() {
            ErrorKind::UnexpectedEof => return Ok(None),
            _ => return Err(Box::new(e)),
        }
    }

    let size = reader.read_i32::<LittleEndian>()?;
    let mut data = vec![0; size as usize];
    reader.read_exact(&mut data[..])?;

    Ok(Some(Chunk { type_id, data }))
}
