//! Serialization/Deserialization for the Game Maker Studio GEN8 format.
//!
//! Game Maker Studio uses a pseudo IFF-like format.
//!
//! It doesn't actually conform to the IFF spec, so it doesn't make sense
//! to use a generic IFF library.
//!
//! A couple assumptions are made about the format.
//!
//! 1. The root chunk is always a FORM chunk
//!
//! 2. The chunks are the following in the following order
//!    (optional chunks are marked with parentheses):
//!
//!     1. GEN8
//!     2. OPTN
//!     3. EXTN
//!     4. SOND
//!     5. (AGRP)
//!     6. SPRT
//!     7. BGND
//!     8. PATH
//!     9. SCPT
//!    10. SHDR
//!    11. FONT
//!    12. TMLN
//!    13. OBJT
//!    14. ROOM
//!    15. DAFL
//!    16. TPAG
//!    17. CODE
//!    18. VARI
//!    19. FUNC
//!    20. STRG
//!    21. TXTR
//!    22. AUDO
//!    23. (LANG)
//!    24. (GLOB)
//!
//! 3. Chunks are aligned on even byte offsets. Practically, we don't have to care about this,
//!    due to (4.), and due to (5.) saving any alignment padding as trailing data.
//!
//! 4. You can rely on the fact that chunks always have sizes (indicated by their
//!    IFF chunk size property) that land you on even alignment when reading,
//!    so you don't have to manually adjust alignment.
//!
//! 5. You can't rely on any of the chunks (other than the root FORM) having a constant size,
//!    that is, having the same size in every game. Therefore, unknown trailing data should be
//!    read, and collected into a buffer that can be written back when serializing.

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::error::Error;
use std::io::{self, SeekFrom, Read};
use {GameData, GameDataWrite};

pub fn read_from<R: Read>(reader: &mut R) -> Result<GameData, Box<Error>> {
    read_form_chunk(reader)?;
    Ok(GameData {
        gen8: read_opt_chunk(reader, b"GEN8")?.ok_or("missing GEN8 chunk")?,
        optn: read_opt_chunk(reader, b"OPTN")?.ok_or("missing OPTN chunk")?,
        extn: read_opt_chunk(reader, b"EXTN")?.ok_or("missing EXTN chunk")?,
        sond: read_opt_chunk(reader, b"SOND")?.ok_or("missing SOND chunk")?,
        agrp: read_opt_chunk(reader, b"AGRP")?,
        sprt: read_opt_chunk(reader, b"SPRT")?.ok_or("missing SPRT chunk")?,
        bgnd: read_opt_chunk(reader, b"BGND")?.ok_or("missing BGND chunk")?,
        path: read_opt_chunk(reader, b"PATH")?.ok_or("missing PATH chunk")?,
        scpt: read_opt_chunk(reader, b"SCPT")?.ok_or("missing SCPT chunk")?,
        shdr: read_opt_chunk(reader, b"SHDR")?.ok_or("missing SHDR chunk")?,
        font: read_opt_chunk(reader, b"FONT")?.ok_or("missing FONT chunk")?,
        tmln: read_opt_chunk(reader, b"TMLN")?.ok_or("missing TMLN chunk")?,
        objt: read_opt_chunk(reader, b"OBJT")?.ok_or("missing OBJT chunk")?,
        room: read_opt_chunk(reader, b"ROOM")?.ok_or("missing ROOM chunk")?,
        dafl: read_opt_chunk(reader, b"DAFL")?.ok_or("missing DAFL chunk")?,
        tpag: read_opt_chunk(reader, b"TPAG")?.ok_or("missing TPAG chunk")?,
        code: read_opt_chunk(reader, b"CODE")?.ok_or("missing CODE chunk")?,
        vari: read_opt_chunk(reader, b"VARI")?.ok_or("missing VARI chunk")?,
        func: read_opt_chunk(reader, b"FUNC")?.ok_or("missing FUNC chunk")?,
        strg: read_opt_chunk(reader, b"STRG")?.ok_or("missing STRG chunk")?,
        txtr: read_opt_chunk(reader, b"TXTR")?.ok_or("missing TXTR chunk")?,
        audo: read_opt_chunk(reader, b"AUDO")?.ok_or("missing AUDO chunk")?,
        lang: read_opt_chunk(reader, b"LANG")?,
        glob: read_opt_chunk(reader, b"GLOB")?,
    })
}

pub fn write_to<W: GameDataWrite>(gdat: &GameData, writer: &mut W) -> io::Result<()> {
    writer.write_all(b"FORM")?;
    // Skip writing the FORM length, because we don't know it yet.
    let after_form_len = writer.seek(SeekFrom::Current(4))?;
    macro_rules! write {
        ($field:expr, $id:expr) => {
            writer.write_all($id)?;
            writer.write_i32::<LittleEndian>($field.len() as i32)?;
            writer.write_all(&$field)?;
        }
    }
    write!(gdat.gen8, b"GEN8");
    write!(gdat.optn, b"OPTN");
    write!(gdat.extn, b"EXTN");
    write!(gdat.sond, b"SOND");
    if let Some(ref data) = gdat.agrp {
        write!(data, b"AGRP");
    }
    write!(gdat.sprt, b"SPRT");
    write!(gdat.bgnd, b"BGND");
    write!(gdat.path, b"PATH");
    write!(gdat.scpt, b"SCPT");
    write!(gdat.shdr, b"SHDR");
    write!(gdat.font, b"FONT");
    write!(gdat.tmln, b"TMLN");
    write!(gdat.objt, b"OBJT");
    write!(gdat.room, b"ROOM");
    write!(gdat.dafl, b"DAFL");
    write!(gdat.tpag, b"TPAG");
    write!(gdat.code, b"CODE");
    write!(gdat.vari, b"VARI");
    write!(gdat.func, b"FUNC");
    write!(gdat.strg, b"STRG");
    write!(gdat.txtr, b"TXTR");
    write!(gdat.audo, b"AUDO");
    if let Some(ref data) = gdat.lang {
        write!(data, b"LANG");
    }
    if let Some(ref data) = gdat.glob {
        write!(data, b"GLOB");
    }
    let end = writer.seek(SeekFrom::Current(0))?;
    let form_len = end - after_form_len;
    // Finally write the form length
    writer.seek(SeekFrom::Start(after_form_len - 4))?;
    writer.write_i32::<LittleEndian>(form_len as i32)?;
    Ok(())
}

fn read_form_chunk<R: Read>(reader: &mut R) -> Result<(), Box<Error>> {
    let mut type_id = [0; 4];
    reader.read_exact(&mut type_id)?;
    let _len = reader.read_i32::<LittleEndian>();

    if &type_id[..] != b"FORM" {
        return Err(
            "Invalid data.win file. Must start with a FORM chunk.".into(),
        );
    }
    Ok(())
}

fn read_opt_chunk<R: Read>(
    reader: &mut R,
    expected_type: &'static [u8; 4],
) -> Result<Option<Box<[u8]>>, Box<Error>> {
    use std::io::ErrorKind;

    let mut type_id = [0; 4];
    if let Err(e) = reader.read_exact(&mut type_id) {
        match e.kind() {
            ErrorKind::UnexpectedEof => return Ok(None),
            _ => return Err(Box::new(e)),
        }
    }

    if type_id != *expected_type {
        // TODO: You can't just return None here, you already read the type id,
        // Which consumed the type id of the next item you're about to read.
        panic!("Optional chunks are not properly implemented.");
    }

    let size = reader.read_i32::<LittleEndian>()?;
    let mut data = vec![0; size as usize];
    reader.read_exact(&mut data[..])?;

    Ok(Some(data.into_boxed_slice()))
}
