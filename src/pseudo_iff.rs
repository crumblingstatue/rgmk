use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::error::Error;
use std::io::{self, SeekFrom};
use {GameDataRead, GameDataWrite};

// A couple assumptions are made about the format.
//
// - The root chunk is always a FORM chunk
//
// - The chunks are the following in the following order
//   (optional chunks are marked with parentheses):
//
//    1. GEN8
//    2. OPTN
//    3. EXTN
//    4. SOND
//    5. (AGRP)
//    6. SPRT
//    7. BGND
//    8. PATH
//    9. SCPT
//   10. SHDR
//   11. FONT
//   12. TMLN
//   13. OBJT
//   14. ROOM
//   15. DAFL
//   16. TPAG
//   17. CODE
//   18. VARI
//   19. FUNC
//   20. STRG
//   21. TXTR
//   22. AUDO
//   23. (LANG)
//   24. (GLOB)
//
// - Chunks are aligned on even byte offsets. Padding bytes are always zero.
//
// - You can rely on the fact that chunks always have sizes (indicated by their
//   IFF chunk size property) that land you on even alignment when reading,
//   so you don't have to manually adjust alignment.

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
    pub gen8: Box<[u8]>,
    pub optn: Box<[u8]>,
    pub extn: Box<[u8]>,
    pub sond: Box<[u8]>,
    pub agrp: Option<Box<[u8]>>,
    pub sprt: Box<[u8]>,
    pub bgnd: Box<[u8]>,
    pub path: Box<[u8]>,
    pub scpt: Box<[u8]>,
    pub shdr: Box<[u8]>,
    pub font: Box<[u8]>,
    pub tmln: Box<[u8]>,
    pub objt: Box<[u8]>,
    pub room: Box<[u8]>,
    pub dafl: Box<[u8]>,
    pub tpag: Box<[u8]>,
    pub code: Box<[u8]>,
    pub vari: Box<[u8]>,
    pub func: Box<[u8]>,
    pub strg: Box<[u8]>,
    pub txtr: Box<[u8]>,
    pub audo: Box<[u8]>,
    pub lang: Option<Box<[u8]>>,
    pub glob: Option<Box<[u8]>>,
}

impl PseudoIff {
    pub fn load<R: GameDataRead>(reader: &mut R) -> Result<Self, Box<Error>> {
        read_form_chunk(reader)?;
        Ok(Self {
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
    pub fn save<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
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
        write!(self.gen8, b"GEN8");
        write!(self.optn, b"OPTN");
        write!(self.extn, b"EXTN");
        write!(self.sond, b"SOND");
        if let Some(ref data) = self.agrp {
            write!(data, b"AGRP");
        }
        write!(self.sprt, b"SPRT");
        write!(self.bgnd, b"BGND");
        write!(self.path, b"PATH");
        write!(self.scpt, b"SCPT");
        write!(self.shdr, b"SHDR");
        write!(self.font, b"FONT");
        write!(self.tmln, b"TMLN");
        write!(self.objt, b"OBJT");
        write!(self.room, b"ROOM");
        write!(self.dafl, b"DAFL");
        write!(self.tpag, b"TPAG");
        write!(self.code, b"CODE");
        write!(self.vari, b"VARI");
        write!(self.func, b"FUNC");
        write!(self.strg, b"STRG");
        write!(self.txtr, b"TXTR");
        write!(self.audo, b"AUDO");
        if let Some(ref data) = self.lang {
            write!(data, b"LANG");
        }
        if let Some(ref data) = self.glob {
            write!(data, b"GLOB");
        }
        let end = writer.seek(SeekFrom::Current(0))?;
        let form_len = end - after_form_len;
        // Finally write the form length
        writer.seek(SeekFrom::Start(after_form_len - 4))?;
        writer.write_i32::<LittleEndian>(form_len as i32)?;
        Ok(())
    }
}

fn read_form_chunk<R: GameDataRead>(reader: &mut R) -> Result<(), Box<Error>> {
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

fn read_opt_chunk<R: GameDataRead>(
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
