//! Reading GameData from data.win format

use std::io::prelude::*;
use std::io;
use byteorder::{self, ReadBytesExt, WriteBytesExt, LittleEndian};
use super::*;

quick_error! {
    #[derive(Debug)]
    pub enum StringReadError {
        Io(err: io::Error) {
            from()
        }
        ByteOrder(err: byteorder::Error) {
            from()
        }
        MissingNullTerminator {}
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum ReadError {
        Io(err: io::Error) {
            from()
        }
        ByteOrder(err: byteorder::Error) {
            from()
        }
        String(err: StringReadError) {
            from()
        }
        InvalidChunkTypeId(what: [u8; 4]) { }
    }
}

const TYPE_ID_LEN: usize = 4;
const CHUNK_HEADER_LEN: i32 = TYPE_ID_LEN as i32 + 4;

pub fn read<R: Read>(reader: &mut R) -> Result<GameData, ReadError> {
    try!(get_chunk_header(reader, b"FORM"));
    let (mut meta, meta_string_offsets) = try!(MetaData::read(reader));
    let optn = try!(Optn::read(reader));
    let extn = try!(Extn::read(reader));
    let sounds = try!(Sounds::read(reader));
    let audio_groups = try!(AudioGroups::read(reader));
    let sprites = try!(Sprites::read(reader));
    let backgrounds = try!(Backgrounds::read(reader));
    let paths = try!(Paths::read(reader));
    let scripts = try!(Scripts::read(reader));
    let shaders = try!(Shaders::read(reader));
    let fonts = try!(Fonts::read(reader));
    let timelines = try!(Timelines::read(reader));
    let objects = try!(Objects::read(reader));
    let rooms = try!(Rooms::read(reader));
    let dafl = try!(Dafl::read(reader));
    let tpag = try!(Tpag::read(reader));
    let code = try!(Code::read(reader));
    let (mut variables, var_name_offsets) = try!(Variables::read(reader));
    let (mut functions, fun_name_offsets) = try!(Functions::read(reader));
    let (strings, offsets) = try!(Strings::read(reader));
    for (i, &soff) in offsets.iter().enumerate() {
        if meta_string_offsets.game_id_1 - 4 == soff {
            meta.game_id_1_index = i;
        }
        if meta_string_offsets.default - 4 == soff {
            meta.default_index = i;
        }
        if meta_string_offsets.game_id_2 - 4 == soff {
            meta.game_id_2_index = i;
        }
        if meta_string_offsets.window_title - 4 == soff {
            meta.window_title_index = i;
        }
    }
    for (i, off) in var_name_offsets.into_iter().enumerate() {
        for (j, &soff) in offsets.iter().enumerate() {
            if off - 4 == soff {
                variables.variables[i].name_index = j;
                break;
            }
        }
    }
    for (i, off) in fun_name_offsets.into_iter().enumerate() {
        for (j, &soff) in offsets.iter().enumerate() {
            if off - 4 == soff {
                functions.functions[i].name_index = j;
                break;
            }
        }
    }
    let textures = try!(Textures::read(reader));
    let audio = try!(Audio::read(reader));
    Ok(GameData {
        metadata: meta,
        optn: optn,
        extn: extn,
        sounds: sounds,
        audio_groups: Some(audio_groups),
        sprites: sprites,
        backgrounds: backgrounds,
        paths: paths,
        scripts: scripts,
        shaders: shaders,
        fonts: fonts,
        timelines: timelines,
        objects: objects,
        rooms: rooms,
        dafl: dafl,
        tpag: tpag,
        code: code,
        variables: variables,
        functions: functions,
        strings: strings,
        textures: textures,
        audio: audio,
    })
}

fn form_content_len(data: &GameData) -> i32 {
    data.metadata.len() + CHUNK_HEADER_LEN + data.optn.len() + CHUNK_HEADER_LEN +
    data.extn.len() + CHUNK_HEADER_LEN + data.sounds.len() + CHUNK_HEADER_LEN +
    data.audio_groups.as_ref().map_or(0, |a| a.len()) + CHUNK_HEADER_LEN +
    data.sprites.len() + CHUNK_HEADER_LEN + data.backgrounds.len() +
    CHUNK_HEADER_LEN +
    data.paths.len() + CHUNK_HEADER_LEN + data.scripts.len() +
    CHUNK_HEADER_LEN + data.shaders.len() +
    CHUNK_HEADER_LEN + data.fonts.len() + CHUNK_HEADER_LEN +
    data.timelines.len() + CHUNK_HEADER_LEN + data.objects.len() + CHUNK_HEADER_LEN +
    data.rooms.len() + CHUNK_HEADER_LEN + data.dafl.len() +
    CHUNK_HEADER_LEN + data.tpag.len() + CHUNK_HEADER_LEN + data.code.len() + CHUNK_HEADER_LEN +
    data.variables.len() + CHUNK_HEADER_LEN + data.functions.len() +
    CHUNK_HEADER_LEN + data.strings.len() + CHUNK_HEADER_LEN + data.textures.len() +
    CHUNK_HEADER_LEN + data.audio.len() + CHUNK_HEADER_LEN
}

pub fn write<W: Write>(data: &GameData, writer: &mut W) -> Result<(), io::Error> {
    let mut offset = 0;
    try!(writer.write_all(b"FORM"));
    let len = form_content_len(data);
    try!(writer.write_i32::<LittleEndian>(len));
    offset += CHUNK_HEADER_LEN;
    try!(data.metadata.write(writer,
                             string_offsets(&data.strings,
                                            offset + data.metadata.len() + data.optn.len() +
                                            data.extn.len() +
                                            data.audio_groups.as_ref().unwrap().len() +
                                            data.sounds.len() +
                                            data.sprites.len() +
                                            data.backgrounds.len() +
                                            data.paths.len() +
                                            data.scripts.len() +
                                            data.shaders.len() +
                                            data.fonts.len() +
                                            data.timelines.len() +
                                            data.objects.len() +
                                            data.rooms.len() +
                                            data.dafl.len() +
                                            data.tpag.len() +
                                            data.code.len() +
                                            data.variables.len() +
                                            data.functions.len() +
                                            (CHUNK_HEADER_LEN * 19))));
    offset += data.metadata.len() + CHUNK_HEADER_LEN;
    try!(data.optn.write(writer, ()));
    offset += data.optn.len() + CHUNK_HEADER_LEN;
    try!(data.extn.write(writer, ()));
    offset += data.extn.len() + CHUNK_HEADER_LEN;
    try!(data.sounds.write(writer, ()));
    offset += data.sounds.len() + CHUNK_HEADER_LEN;
    if let Some(ref agrp) = data.audio_groups {
        try!(agrp.write(writer, ()));
        offset += agrp.len() + CHUNK_HEADER_LEN;
    }
    try!(data.sprites.write(writer, ()));
    offset += data.sprites.len() + CHUNK_HEADER_LEN;
    try!(data.backgrounds.write(writer, ()));
    offset += data.backgrounds.len() + CHUNK_HEADER_LEN;
    try!(data.paths.write(writer, ()));
    offset += data.paths.len() + CHUNK_HEADER_LEN;
    try!(data.scripts.write(writer, ()));
    offset += data.scripts.len() + CHUNK_HEADER_LEN;
    try!(data.shaders.write(writer, ()));
    offset += data.shaders.len() + CHUNK_HEADER_LEN;
    try!(data.fonts.write(writer, ()));
    offset += data.fonts.len() + CHUNK_HEADER_LEN;
    try!(data.timelines.write(writer, ()));
    offset += data.timelines.len() + CHUNK_HEADER_LEN;
    try!(data.objects.write(writer, ()));
    offset += data.objects.len() + CHUNK_HEADER_LEN;
    try!(data.rooms.write(writer, ()));
    offset += data.rooms.len() + CHUNK_HEADER_LEN;
    try!(data.dafl.write(writer, ()));
    offset += data.dafl.len() + CHUNK_HEADER_LEN;
    try!(data.tpag.write(writer, ()));
    offset += data.tpag.len() + CHUNK_HEADER_LEN;
    try!(data.code.write(writer, ()));
    offset += data.code.len() + CHUNK_HEADER_LEN;
    try!(data.variables.write(writer,
                              string_offsets(&data.strings,
                                             offset + data.variables.len() + CHUNK_HEADER_LEN +
                                             data.functions.len() +
                                             CHUNK_HEADER_LEN)));
    offset += data.variables.len() + CHUNK_HEADER_LEN;
    try!(data.functions.write(writer,
                              string_offsets(&data.strings,
                                             offset + data.functions.len() + CHUNK_HEADER_LEN)));
    offset += data.functions.len() + CHUNK_HEADER_LEN;
    try!(data.strings.write(writer, offset));
    // offset += data.strings.len() + CHUNK_HEADER_LEN;
    try!(data.textures.write(writer, ()));
    // offset += data.textures.len() + CHUNK_HEADER_LEN;
    try!(data.audio.write(writer, ()));
    Ok(())
}

fn string_offsets(strings: &Strings, base_offset: i32) -> Vec<i32> {
    let mut offset = base_offset + CHUNK_HEADER_LEN + 4 + (strings.strings.len() as i32 * 4);
    let mut offsets = Vec::new();
    for string in &strings.strings {
        // +4 because functions point right into the string
        offsets.push(offset + 4);
        offset += (string.len() + 1) as i32 + 4;
    }
    offsets
}

trait Chunk {
    const TYPE_ID: &'static [u8; 4];
    type ReadOutput = Self;
    /// Additional inormation needed in order to be able to write correct output.
    type WriteInput = ();
    fn read<R: Read>(reader: &mut R) -> Result<Self::ReadOutput, ReadError>;
    fn write<W: Write>(&self, writer: &mut W, input: Self::WriteInput) -> Result<(), io::Error>;
    fn len(&self) -> i32;
}

macro_rules! unk_chunk {
    ($name:ident, $typeid:expr) => {
        impl Chunk for $name {
            const TYPE_ID: &'static [u8; 4] = $typeid;
            fn read<R: Read>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
                let chunk_header = try!(get_chunk_header(reader, Self::TYPE_ID));
                Ok($name {
                    raw: try!(read_into_byte_vec(reader, chunk_header.size))
                })
            }
            fn write<W: Write>(&self, writer: &mut W, _input: ()) -> Result<(), io::Error> {
                try!(writer.write_all(Self::TYPE_ID));
                try!(writer.write_i32::<LittleEndian>(self.len()));
                try!(writer.write_all(&self.raw));
                Ok(())
            }
            fn len(&self) -> i32 {
                self.raw.len() as i32
            }
        }
    }
}

#[derive(Clone, Copy)]
struct MetaDataOffsets {
    game_id_1: u32,
    default: u32,
    game_id_2: u32,
    window_title: u32,
}

impl Chunk for MetaData {
    const TYPE_ID: &'static [u8; 4] = b"GEN8";
    type ReadOutput = (Self, MetaDataOffsets);
    type WriteInput = Vec<i32>;
    fn read<R: Read>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        let header = try!(get_chunk_header(reader, Self::TYPE_ID));
        let unk1 = try!(reader.read_u32::<LittleEndian>());
        let game_id_1_offset = try!(reader.read_u32::<LittleEndian>());
        let default_offset = try!(reader.read_u32::<LittleEndian>());
        let unk2 = try!(reader.read_u32::<LittleEndian>());
        let unk3 = try!(reader.read_u32::<LittleEndian>());
        let unk4 = try!(reader.read_u32::<LittleEndian>());
        let unk5 = try!(reader.read_u32::<LittleEndian>());
        let unk6 = try!(reader.read_u32::<LittleEndian>());
        let unk7 = try!(reader.read_u32::<LittleEndian>());
        let unk8 = try!(reader.read_u32::<LittleEndian>());
        let game_id_2_offset = try!(reader.read_u32::<LittleEndian>());
        let unk9 = try!(reader.read_u32::<LittleEndian>());
        let unk10 = try!(reader.read_u32::<LittleEndian>());
        let unk11 = try!(reader.read_u32::<LittleEndian>());
        let unk12 = try!(reader.read_u32::<LittleEndian>());
        let window_width = try!(reader.read_u32::<LittleEndian>());
        let window_height = try!(reader.read_u32::<LittleEndian>());
        let unk13 = try!(reader.read_u32::<LittleEndian>());
        let unk14 = try!(reader.read_u32::<LittleEndian>());
        let unk15 = try!(reader.read_u32::<LittleEndian>());
        let unk16 = try!(reader.read_u32::<LittleEndian>());
        let unk17 = try!(reader.read_u32::<LittleEndian>());
        let unk18 = try!(reader.read_u32::<LittleEndian>());
        let unk19 = try!(reader.read_u32::<LittleEndian>());
        let unk20 = try!(reader.read_u32::<LittleEndian>());
        let window_title_offset = try!(reader.read_u32::<LittleEndian>());
        let mut remaining = header.size - (26 * 4);
        let mut values = Vec::new();
        while remaining > 0 {
            let value = try!(reader.read_u32::<LittleEndian>());
            values.push(value);
            remaining -= 4;
        }
        Ok((MetaData {
            unk1: unk1,
            game_id_1_index: 0,
            default_index: 0,
            unk2: unk2,
            unk3: unk3,
            unk4: unk4,
            unk5: unk5,
            unk6: unk6,
            unk7: unk7,
            unk8: unk8,
            game_id_2_index: 0,
            unk9: unk9,
            unk10: unk10,
            unk11: unk11,
            unk12: unk12,
            window_width: window_width,
            window_height: window_height,
            unk13: unk13,
            unk14: unk14,
            unk15: unk15,
            unk16: unk16,
            unk17: unk17,
            unk18: unk18,
            unk19: unk19,
            unk20: unk20,
            window_title_index: 0,
            unknown: values,
        },
            MetaDataOffsets {
            game_id_1: game_id_1_offset,
            default: default_offset,
            game_id_2: game_id_2_offset,
            window_title: window_title_offset,
        }))
    }
    fn write<W: Write>(&self, writer: &mut W, input: Self::WriteInput) -> Result<(), io::Error> {
        try!(writer.write_all(Self::TYPE_ID));
        try!(writer.write_i32::<LittleEndian>(self.len()));
        try!(writer.write_u32::<LittleEndian>(self.unk1));
        try!(writer.write_u32::<LittleEndian>(input[self.game_id_1_index] as u32));
        try!(writer.write_u32::<LittleEndian>(input[self.default_index] as u32));
        try!(writer.write_u32::<LittleEndian>(self.unk2));
        try!(writer.write_u32::<LittleEndian>(self.unk3));
        try!(writer.write_u32::<LittleEndian>(self.unk4));
        try!(writer.write_u32::<LittleEndian>(self.unk5));
        try!(writer.write_u32::<LittleEndian>(self.unk6));
        try!(writer.write_u32::<LittleEndian>(self.unk7));
        try!(writer.write_u32::<LittleEndian>(self.unk8));
        try!(writer.write_u32::<LittleEndian>(input[self.game_id_2_index] as u32));
        try!(writer.write_u32::<LittleEndian>(self.unk9));
        try!(writer.write_u32::<LittleEndian>(self.unk10));
        try!(writer.write_u32::<LittleEndian>(self.unk11));
        try!(writer.write_u32::<LittleEndian>(self.unk12));
        try!(writer.write_u32::<LittleEndian>(self.window_width));
        try!(writer.write_u32::<LittleEndian>(self.window_height));
        try!(writer.write_u32::<LittleEndian>(self.unk13));
        try!(writer.write_u32::<LittleEndian>(self.unk14));
        try!(writer.write_u32::<LittleEndian>(self.unk15));
        try!(writer.write_u32::<LittleEndian>(self.unk16));
        try!(writer.write_u32::<LittleEndian>(self.unk17));
        try!(writer.write_u32::<LittleEndian>(self.unk18));
        try!(writer.write_u32::<LittleEndian>(self.unk19));
        try!(writer.write_u32::<LittleEndian>(self.unk20));
        try!(writer.write_u32::<LittleEndian>(input[self.window_title_index] as u32));
        for &v in &self.unknown {
            try!(writer.write_u32::<LittleEndian>(v));
        }
        Ok(())
    }
    fn len(&self) -> i32 {
        (self.unknown.len() as i32 * 4) + (26 * 4)
    }
}
unk_chunk!(Optn, b"OPTN");
unk_chunk!(Extn, b"EXTN");
unk_chunk!(Sounds, b"SOND");
unk_chunk!(AudioGroups, b"AGRP");
unk_chunk!(Sprites, b"SPRT");
unk_chunk!(Backgrounds, b"BGND");
unk_chunk!(Paths, b"PATH");
unk_chunk!(Scripts, b"SCPT");
unk_chunk!(Shaders, b"SHDR");
unk_chunk!(Fonts, b"FONT");
unk_chunk!(Timelines, b"TMLN");
unk_chunk!(Objects, b"OBJT");
unk_chunk!(Rooms, b"ROOM");
unk_chunk!(Dafl, b"DAFL");
unk_chunk!(Tpag, b"TPAG");
unk_chunk!(Code, b"CODE");
unk_chunk!(Textures, b"TXTR");
unk_chunk!(Audio, b"AUDO");

impl Chunk for Variables {
    const TYPE_ID: &'static [u8; 4] = b"VARI";
    type ReadOutput = (Self, Vec<u32>);
    type WriteInput = Vec<i32>;
    fn read<R: Read>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        let header = try!(get_chunk_header(reader, Self::TYPE_ID));
        let mut offsets = Vec::new();
        let mut vars = Vec::new();
        let mut remaining = header.size;
        while remaining > 0 {
            let offset = try!(reader.read_u32::<LittleEndian>());
            let unk1 = try!(reader.read_u32::<LittleEndian>());
            let unk2 = try!(reader.read_u32::<LittleEndian>());
            vars.push(Variable {
                name_index: 0,
                unknown1: unk1,
                unknown2: unk2,
            });
            offsets.push(offset);
            remaining -= 3 * 4;
        }
        Ok((Variables { variables: vars }, offsets))
    }
    fn write<W: Write>(&self, writer: &mut W, input: Self::WriteInput) -> Result<(), io::Error> {
        try!(writer.write_all(Self::TYPE_ID));
        let len = self.len();
        try!(writer.write_i32::<LittleEndian>(len));
        for var in &self.variables {
            try!(writer.write_u32::<LittleEndian>(input[var.name_index] as u32));
            try!(writer.write_u32::<LittleEndian>(var.unknown1));
            try!(writer.write_u32::<LittleEndian>(var.unknown2));
        }
        Ok(())
    }
    fn len(&self) -> i32 {
        (self.variables.len() * (3 * 4)) as i32
    }
}

impl Chunk for Functions {
    const TYPE_ID: &'static [u8; 4] = b"FUNC";
    type ReadOutput = (Self, Vec<u32>);
    type WriteInput = Vec<i32>;
    fn read<R: Read>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        let header = try!(get_chunk_header(reader, Self::TYPE_ID));
        let mut offsets = Vec::new();
        let mut funs = Vec::new();
        let mut remaining = header.size;
        while remaining > 0 {
            let offset = try!(reader.read_u32::<LittleEndian>());
            let unk1 = try!(reader.read_u32::<LittleEndian>());
            let unk2 = try!(reader.read_u32::<LittleEndian>());
            funs.push(Function {
                name_index: 0,
                unknown1: unk1,
                unknown2: unk2,
            });
            offsets.push(offset);
            remaining -= 3 * 4;
        }
        Ok((Functions { functions: funs }, offsets))
    }
    fn write<W: Write>(&self, writer: &mut W, input: Self::WriteInput) -> Result<(), io::Error> {
        try!(writer.write_all(Self::TYPE_ID));
        let len = self.len();
        try!(writer.write_i32::<LittleEndian>(len));
        for fun in &self.functions {
            try!(writer.write_u32::<LittleEndian>(input[fun.name_index] as u32));
            try!(writer.write_u32::<LittleEndian>(fun.unknown1));
            try!(writer.write_u32::<LittleEndian>(fun.unknown2));
        }
        Ok(())
    }
    fn len(&self) -> i32 {
        (self.functions.len() * (3 * 4)) as i32
    }
}

impl Chunk for Strings {
    const TYPE_ID: &'static [u8; 4] = b"STRG";
    type ReadOutput = (Self, Vec<u32>);
    type WriteInput = i32;
    fn read<R: Read>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
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
    fn write<W: Write>(&self, writer: &mut W, offset: i32) -> Result<(), io::Error> {
        try!(writer.write_all(Self::TYPE_ID));
        try!(writer.write_i32::<LittleEndian>(self.len()));
        try!(writer.write_u32::<LittleEndian>(self.strings.len() as u32));
        let mut string_offset = offset + CHUNK_HEADER_LEN + 4 + (self.strings.len() as i32 * 4);
        for string in &self.strings {
            try!(writer.write_u32::<LittleEndian>(string_offset as u32));
            string_offset += (string.len() + 1) as i32 + 4;
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
    fn len(&self) -> i32 {
        let mut lengths = 0;
        for s in &self.strings {
            // The length denominator before the string
            lengths += 4;
            // + 1 for null terminator
            lengths += s.len() + 1;
        }
        // +4 at end for zero padding
        (4 + (self.strings.len() * 4) + lengths + 4) as i32
    }
}

fn read_string<R: Read>(reader: &mut R) -> Result<String, StringReadError> {
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

struct ChunkHeader {
    type_id: [u8; TYPE_ID_LEN],
    size: usize,
}

fn read_chunk_header<R: Read>(reader: &mut R) -> Result<ChunkHeader, ReadError> {
    let mut type_id = [0u8; TYPE_ID_LEN];
    try!(reader.read_exact(&mut type_id));
    let size = try!(reader.read_i32::<LittleEndian>());
    Ok(ChunkHeader {
        type_id: type_id,
        size: size as usize,
    })
}

fn get_chunk_header<R: Read>(reader: &mut R, should_be: &[u8]) -> Result<ChunkHeader, ReadError> {
    let header = try!(read_chunk_header(reader));
    if &header.type_id == should_be {
        Ok(header)
    } else {
        Err(ReadError::InvalidChunkTypeId(header.type_id))
    }
}

fn read_into_byte_vec<R: Read>(reader: &mut R, len: usize) -> Result<Vec<u8>, io::Error> {
    let mut vec = Vec::with_capacity(len);
    unsafe {
        vec.set_len(len);
        try!(reader.read_exact(&mut vec));
    }
    Ok(vec)
}
