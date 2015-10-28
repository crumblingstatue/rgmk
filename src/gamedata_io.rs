//! Reading GameData from data.win format

use std::io::prelude::*;
use std::io;
use byteorder::{self, ReadBytesExt, WriteBytesExt, LittleEndian};
use super::{GameData, MetaData, Options, Extn, Sound, Sounds, AudioGroups, Sprites, Backgrounds,
            Paths, Script, Scripts, Shaders, Fonts, Timelines, Objects, Rooms, Dafl, Tpag, Code,
            Variable, Variables, Function, Functions, Strings, Texture, Textures, AudioData,
            Audio, GameDataRead, GameDataWrite};

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

quick_error! {
    #[derive(Debug)]
    /// Error when reading GameData from a reader.
    pub enum ReadError {
        /// An I/O error.
        Io(err: io::Error) {
            from()
        }
        /// A byte order error.
        ByteOrder(err: byteorder::Error) {
            from()
        }
        /// A string read error.
        String(err: StringReadError) {
            from()
        }
        /// Invalid chunk type id.
        InvalidChunkTypeId(what: [u8; 4]) { }
    }
}

const TYPE_ID_LEN: usize = 4;
const CHUNK_HEADER_LEN: i32 = TYPE_ID_LEN as i32 + 4;

pub fn read<R: GameDataRead>(reader: &mut R) -> Result<GameData, ReadError> {
    try!(get_chunk_header(reader, b"FORM"));
    let (mut meta, meta_string_offsets) = try!(MetaData::read(reader));
    let (mut opts, opt_offsets) = try!(Options::read(reader));
    let extn = try!(Extn::read(reader));
    let (mut sounds, sound_string_offsets) = try!(Sounds::read(reader));
    let audio_groups = try!(AudioGroups::read(reader));
    let sprites = try!(Sprites::read(reader));
    let backgrounds = try!(Backgrounds::read(reader));
    let paths = try!(Paths::read(reader));
    let (mut scripts, script_name_offsets) = try!(Scripts::read(reader));
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
    for (i, &soff) in offsets.iter().enumerate() {
        if opt_offsets.const1_offset - 4 == soff {
            opts.constant1_name_index = i;
        }
        if opt_offsets.const2_offset - 4 == soff {
            opts.constant2_name_index = i;
        }
        if opt_offsets.const3_offset - 4 == soff {
            opts.constant3_name_index = i;
        }
        if opt_offsets.const4_offset - 4 == soff {
            opts.constant4_name_index = i;
        }
        if opt_offsets.const5_offset - 4 == soff {
            opts.constant5_name_index = i;
        }
        if opt_offsets.const6_offset - 4 == soff {
            opts.constant6_name_index = i;
        }
        if opt_offsets.const7_offset - 4 == soff {
            opts.constant7_name_index = i;
        }
        if opt_offsets.const8_offset - 4 == soff {
            opts.constant8_name_index = i;
        }
        if opt_offsets.const9_offset - 4 == soff {
            opts.constant9_name_index = i;
        }
        if opt_offsets.const10_offset - 4 == soff {
            opts.constant10_name_index = i;
        }
        if opt_offsets.const11_offset - 4 == soff {
            opts.constant11_name_index = i;
        }
        if opt_offsets.const12_offset - 4 == soff {
            opts.constant12_name_index = i;
        }
        if opt_offsets.const13_offset - 4 == soff {
            opts.constant13_name_index = i;
        }
        if opt_offsets.const14_offset - 4 == soff {
            opts.constant14_name_index = i;
        }
    }
    for (i, s) in sound_string_offsets.into_iter().enumerate() {
        for (j, &soff) in offsets.iter().enumerate() {
            if s.name_offset - 4 == soff {
                sounds.sounds[i].name_index = j;
            }
            if s.ext_offset - 4 == soff {
                sounds.sounds[i].ext_index = j;
            }
            if s.filename_offset - 4 == soff {
                sounds.sounds[i].filename_index = j;
            }
        }
    }
    for (i, off) in script_name_offsets.into_iter().enumerate() {
        for (j, &soff) in offsets.iter().enumerate() {
            if off - 4 == soff {
                scripts.scripts[i].name_index = j;
                break;
            }
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
    let textures_offset = try!(reader.seek(io::SeekFrom::Current(0))) as u32;
    let textures = try!(Textures::read(reader));
    let texture_data_offset = texture_data_offset(&textures, textures_offset);
    opts.icon_offset = opt_offsets.icon_offset - texture_data_offset;
    let audio = try!(Audio::read(reader));
    Ok(GameData {
        metadata: meta,
        options: opts,
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
    data.metadata.content_size() + CHUNK_HEADER_LEN + data.options.content_size() +
    CHUNK_HEADER_LEN + data.extn.content_size() + CHUNK_HEADER_LEN +
    data.sounds.content_size() + CHUNK_HEADER_LEN +
    data.audio_groups.as_ref().map_or(0, |a| a.content_size() + CHUNK_HEADER_LEN) +
    data.sprites.content_size() + CHUNK_HEADER_LEN + data.backgrounds.content_size() +
    CHUNK_HEADER_LEN + data.paths.content_size() + CHUNK_HEADER_LEN +
    data.scripts.content_size() + CHUNK_HEADER_LEN +
    data.shaders.content_size() + CHUNK_HEADER_LEN +
    data.fonts.content_size() + CHUNK_HEADER_LEN + data.timelines.content_size() +
    CHUNK_HEADER_LEN + data.objects.content_size() +
    CHUNK_HEADER_LEN +
    data.rooms.content_size() + CHUNK_HEADER_LEN + data.dafl.content_size() + CHUNK_HEADER_LEN +
    data.tpag.content_size() + CHUNK_HEADER_LEN + data.code.content_size() +
    CHUNK_HEADER_LEN + data.variables.content_size() +
    CHUNK_HEADER_LEN + data.functions.content_size() +
    CHUNK_HEADER_LEN + data.strings.content_size() +
    CHUNK_HEADER_LEN + data.textures.content_size() + CHUNK_HEADER_LEN +
    data.audio.content_size() + CHUNK_HEADER_LEN
}

pub fn write<W: GameDataWrite>(data: &GameData, writer: &mut W) -> io::Result<()> {
    try!(writer.write_all(b"FORM"));
    try!(writer.write_i32::<LittleEndian>(form_content_len(data)));
    let stringtable_offset = data.metadata.content_size() + data.options.content_size() +
                             data.extn.content_size() +
                             data.audio_groups
                                 .as_ref()
                                 .map_or(0, |a| a.content_size() + CHUNK_HEADER_LEN) +
                             data.sounds.content_size() +
                             data.sprites.content_size() +
                             data.backgrounds.content_size() +
                             data.paths.content_size() +
                             data.scripts.content_size() +
                             data.shaders.content_size() +
                             data.fonts.content_size() +
                             data.timelines.content_size() +
                             data.objects.content_size() +
                             data.rooms.content_size() +
                             data.dafl.content_size() +
                             data.tpag.content_size() +
                             data.code.content_size() +
                             data.variables.content_size() +
                             data.functions.content_size() +
                             (CHUNK_HEADER_LEN * 19);
    let string_offsets = string_offsets(&data.strings, stringtable_offset);
    try!(data.metadata.write(writer, &string_offsets));
    try!(data.options.write(writer,
                            (&string_offsets,
                             texture_data_offset(&data.textures,
                                                 stringtable_offset as u32 +
                                                 data.strings.content_size() as u32 +
                                                 CHUNK_HEADER_LEN as u32))));
    try!(data.extn.write(writer, ()));
    try!(data.sounds.write(writer, &string_offsets));
    if let Some(ref agrp) = data.audio_groups {
        try!(agrp.write(writer, ()));
    }
    try!(data.sprites.write(writer, ()));
    try!(data.backgrounds.write(writer, ()));
    try!(data.paths.write(writer, ()));
    try!(data.scripts.write(writer, &string_offsets));
    try!(data.shaders.write(writer, ()));
    try!(data.fonts.write(writer, ()));
    try!(data.timelines.write(writer, ()));
    try!(data.objects.write(writer, ()));
    try!(data.rooms.write(writer, ()));
    try!(data.dafl.write(writer, ()));
    try!(data.tpag.write(writer, ()));
    try!(data.code.write(writer, ()));
    try!(data.variables.write(writer, &string_offsets));
    try!(data.functions.write(writer, &string_offsets));
    try!(data.strings.write(writer, stringtable_offset));
    try!(data.textures.write(writer, ()));
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

trait Chunk<'a> {
    const TYPE_ID: &'static [u8; 4];
    type ReadOutput = Self;
    /// Additional inormation needed in order to be able to write correct output.
    type WriteInput = ();
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError>;
    fn write<W: GameDataWrite>(&self, writer: &mut W, input: Self::WriteInput) -> io::Result<()>;
    fn content_size(&self) -> i32;
}

macro_rules! unk_chunk {
    ($name:ident, $typeid:expr) => {
        impl<'a> Chunk<'a> for $name {
            const TYPE_ID: &'static [u8; 4] = $typeid;
            fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
                let chunk_header = try!(get_chunk_header(reader, Self::TYPE_ID));
                Ok($name {
                    raw: try!(read_into_byte_vec(reader, chunk_header.size))
                })
            }
            fn write<W: GameDataWrite>(&self, writer: &mut W, _input: ()) -> io::Result<()> {
                try!(writer.write_all(Self::TYPE_ID));
                try!(writer.write_i32::<LittleEndian>(self.content_size()));
                try!(writer.write_all(&self.raw));
                Ok(())
            }
            fn content_size(&self) -> i32 {
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

impl<'a> Chunk<'a> for MetaData {
    const TYPE_ID: &'static [u8; 4] = b"GEN8";
    type ReadOutput = (Self, MetaDataOffsets);
    type WriteInput = &'a [i32];
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
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
    fn write<W: GameDataWrite>(&self, writer: &mut W, input: Self::WriteInput) -> io::Result<()> {
        try!(writer.write_all(Self::TYPE_ID));
        try!(writer.write_i32::<LittleEndian>(self.content_size()));
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
    fn content_size(&self) -> i32 {
        (self.unknown.len() as i32 * 4) + (26 * 4)
    }
}

#[derive(Clone, Copy)]
struct OptionOffsets {
    icon_offset: u32,
    const1_offset: u32,
    const2_offset: u32,
    const3_offset: u32,
    const4_offset: u32,
    const5_offset: u32,
    const6_offset: u32,
    const7_offset: u32,
    const8_offset: u32,
    const9_offset: u32,
    const10_offset: u32,
    const11_offset: u32,
    const12_offset: u32,
    const13_offset: u32,
    const14_offset: u32,
}

impl<'a> Chunk<'a> for Options {
    const TYPE_ID: &'static [u8; 4] = b"OPTN";
    type ReadOutput = (Self, OptionOffsets);
    type WriteInput = (&'a [i32], u32);
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        try!(get_chunk_header(reader, Self::TYPE_ID));
        let unk1 = try!(reader.read_u32::<LittleEndian>());
        let unk2 = try!(reader.read_u32::<LittleEndian>());
        let icon_offset = try!(reader.read_u32::<LittleEndian>());
        let unk3 = try!(reader.read_u32::<LittleEndian>());
        let unk4 = try!(reader.read_u32::<LittleEndian>());
        let unk5 = try!(reader.read_u32::<LittleEndian>());
        let unk6 = try!(reader.read_u32::<LittleEndian>());
        let unk7 = try!(reader.read_u32::<LittleEndian>());
        let unk8 = try!(reader.read_u32::<LittleEndian>());
        let unk9 = try!(reader.read_u32::<LittleEndian>());
        let unk10 = try!(reader.read_u32::<LittleEndian>());
        let unk11 = try!(reader.read_u32::<LittleEndian>());
        let unk12 = try!(reader.read_u32::<LittleEndian>());
        let unk13 = try!(reader.read_u32::<LittleEndian>());
        let unk14 = try!(reader.read_u32::<LittleEndian>());
        let unk15 = try!(reader.read_u32::<LittleEndian>());
        let const1_offset = try!(reader.read_u32::<LittleEndian>());
        let const2_offset = try!(reader.read_u32::<LittleEndian>());
        let const3_offset = try!(reader.read_u32::<LittleEndian>());
        let const4_offset = try!(reader.read_u32::<LittleEndian>());
        let const5_offset = try!(reader.read_u32::<LittleEndian>());
        let const6_offset = try!(reader.read_u32::<LittleEndian>());
        let const7_offset = try!(reader.read_u32::<LittleEndian>());
        let const8_offset = try!(reader.read_u32::<LittleEndian>());
        let const9_offset = try!(reader.read_u32::<LittleEndian>());
        let const10_offset = try!(reader.read_u32::<LittleEndian>());
        let const11_offset = try!(reader.read_u32::<LittleEndian>());
        let const12_offset = try!(reader.read_u32::<LittleEndian>());
        let const13_offset = try!(reader.read_u32::<LittleEndian>());
        let const14_offset = try!(reader.read_u32::<LittleEndian>());
        Ok((Options {
            unk1: unk1,
            unk2: unk2,
            icon_offset: 0,
            unk3: unk3,
            unk4: unk4,
            unk5: unk5,
            unk6: unk6,
            unk7: unk7,
            unk8: unk8,
            unk9: unk9,
            unk10: unk10,
            unk11: unk11,
            unk12: unk12,
            unk13: unk13,
            unk14: unk14,
            unk15: unk15,
            constant1_name_index: 0,
            constant2_name_index: 0,
            constant3_name_index: 0,
            constant4_name_index: 0,
            constant5_name_index: 0,
            constant6_name_index: 0,
            constant7_name_index: 0,
            constant8_name_index: 0,
            constant9_name_index: 0,
            constant10_name_index: 0,
            constant11_name_index: 0,
            constant12_name_index: 0,
            constant13_name_index: 0,
            constant14_name_index: 0,
        },
            OptionOffsets {
            icon_offset: icon_offset,
            const1_offset: const1_offset,
            const2_offset: const2_offset,
            const3_offset: const3_offset,
            const4_offset: const4_offset,
            const5_offset: const5_offset,
            const6_offset: const6_offset,
            const7_offset: const7_offset,
            const8_offset: const8_offset,
            const9_offset: const9_offset,
            const10_offset: const10_offset,
            const11_offset: const11_offset,
            const12_offset: const12_offset,
            const13_offset: const13_offset,
            const14_offset: const14_offset,
        }))
    }
    fn write<W: GameDataWrite>(&self,
                               writer: &mut W,
                               (input, texture_data_offset): Self::WriteInput)
                               -> io::Result<()> {
        try!(writer.write_all(Self::TYPE_ID));
        try!(writer.write_i32::<LittleEndian>(self.content_size()));
        try!(writer.write_u32::<LittleEndian>(self.unk1));
        try!(writer.write_u32::<LittleEndian>(self.unk2));
        try!(writer.write_u32::<LittleEndian>(texture_data_offset + self.icon_offset));
        try!(writer.write_u32::<LittleEndian>(self.unk3));
        try!(writer.write_u32::<LittleEndian>(self.unk4));
        try!(writer.write_u32::<LittleEndian>(self.unk5));
        try!(writer.write_u32::<LittleEndian>(self.unk6));
        try!(writer.write_u32::<LittleEndian>(self.unk7));
        try!(writer.write_u32::<LittleEndian>(self.unk8));
        try!(writer.write_u32::<LittleEndian>(self.unk9));
        try!(writer.write_u32::<LittleEndian>(self.unk10));
        try!(writer.write_u32::<LittleEndian>(self.unk11));
        try!(writer.write_u32::<LittleEndian>(self.unk12));
        try!(writer.write_u32::<LittleEndian>(self.unk13));
        try!(writer.write_u32::<LittleEndian>(self.unk14));
        try!(writer.write_u32::<LittleEndian>(self.unk15));
        try!(writer.write_u32::<LittleEndian>(input[self.constant1_name_index] as u32));
        try!(writer.write_u32::<LittleEndian>(input[self.constant2_name_index] as u32));
        try!(writer.write_u32::<LittleEndian>(input[self.constant3_name_index] as u32));
        try!(writer.write_u32::<LittleEndian>(input[self.constant4_name_index] as u32));
        try!(writer.write_u32::<LittleEndian>(input[self.constant5_name_index] as u32));
        try!(writer.write_u32::<LittleEndian>(input[self.constant6_name_index] as u32));
        try!(writer.write_u32::<LittleEndian>(input[self.constant7_name_index] as u32));
        try!(writer.write_u32::<LittleEndian>(input[self.constant8_name_index] as u32));
        try!(writer.write_u32::<LittleEndian>(input[self.constant9_name_index] as u32));
        try!(writer.write_u32::<LittleEndian>(input[self.constant10_name_index] as u32));
        try!(writer.write_u32::<LittleEndian>(input[self.constant11_name_index] as u32));
        try!(writer.write_u32::<LittleEndian>(input[self.constant12_name_index] as u32));
        try!(writer.write_u32::<LittleEndian>(input[self.constant13_name_index] as u32));
        try!(writer.write_u32::<LittleEndian>(input[self.constant14_name_index] as u32));
        Ok(())
    }
    fn content_size(&self) -> i32 {
        30 * 4
    }
}

unk_chunk!(Extn, b"EXTN");
unk_chunk!(AudioGroups, b"AGRP");
unk_chunk!(Sprites, b"SPRT");
unk_chunk!(Backgrounds, b"BGND");
unk_chunk!(Paths, b"PATH");
unk_chunk!(Shaders, b"SHDR");
unk_chunk!(Fonts, b"FONT");
unk_chunk!(Timelines, b"TMLN");
unk_chunk!(Objects, b"OBJT");
unk_chunk!(Rooms, b"ROOM");
unk_chunk!(Dafl, b"DAFL");
unk_chunk!(Tpag, b"TPAG");
unk_chunk!(Code, b"CODE");

struct SoundStringOffsets {
    name_offset: u32,
    ext_offset: u32,
    filename_offset: u32,
}

impl<'a> Chunk<'a> for Sounds {
    const TYPE_ID: &'static [u8; 4] = b"SOND";
    type ReadOutput = (Self, Vec<SoundStringOffsets>);
    type WriteInput = &'a [i32];
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
            offsets.push(SoundStringOffsets {
                name_offset: name_offset,
                ext_offset: ext_offset,
                filename_offset: filename_offset,
            });
        }
        Ok((Sounds { sounds: sounds }, offsets))
    }
    fn write<W: GameDataWrite>(&self, writer: &mut W, input: Self::WriteInput) -> io::Result<()> {
        try!(writer.write_all(Self::TYPE_ID));
        try!(writer.write_i32::<LittleEndian>(self.content_size()));
        let num_sounds = self.sounds.len() as u32;
        try!(writer.write_u32::<LittleEndian>(num_sounds));
        let writer_offset = try!(writer.seek(io::SeekFrom::Current(0))) as u32;
        let sound_data_offset = writer_offset + (num_sounds * 4);
        for i in 0..num_sounds {
            try!(writer.write_u32::<LittleEndian>(sound_data_offset + (i * (9 * 4))));
        }
        for s in &self.sounds {
            try!(writer.write_u32::<LittleEndian>(input[s.name_index] as u32));
            try!(writer.write_u32::<LittleEndian>(s.unk1));
            try!(writer.write_u32::<LittleEndian>(input[s.ext_index] as u32));
            try!(writer.write_u32::<LittleEndian>(input[s.filename_index] as u32));
            try!(writer.write_u32::<LittleEndian>(s.unk2));
            try!(writer.write_u32::<LittleEndian>(s.unk3));
            try!(writer.write_u32::<LittleEndian>(s.unk4));
            try!(writer.write_u32::<LittleEndian>(s.unk5));
            try!(writer.write_u32::<LittleEndian>(s.unk6));
        }
        Ok(())
    }
    fn content_size(&self) -> i32 {
        let num_sounds_size = 4;
        let num_sounds = self.sounds.len() as i32;
        let offsets_size = num_sounds * 4;
        let sounds_size = num_sounds * (9 * 4);
        num_sounds_size + offsets_size + sounds_size
    }
}

impl<'a> Chunk<'a> for Scripts {
    const TYPE_ID: &'static [u8; 4] = b"SCPT";
    type ReadOutput = (Self, Vec<u32>);
    type WriteInput = &'a [i32];
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        try!(get_chunk_header(reader, Self::TYPE_ID));
        let num_scripts = try!(reader.read_u32::<LittleEndian>());
        trace!("{} scripts", num_scripts);
        // Read script entry offsets
        for _ in 0..num_scripts {
            // For now just discard them
            try!(reader.read_u32::<LittleEndian>());
        }
        let mut name_offsets = Vec::new();
        let mut scripts = Vec::new();
        for _ in 0..num_scripts {
            name_offsets.push(try!(reader.read_u32::<LittleEndian>()));
            scripts.push(Script {
                name_index: 0,
                unknown: try!(reader.read_u32::<LittleEndian>()),
            });
        }
        Ok((Scripts { scripts: scripts }, name_offsets))
    }
    fn write<W: GameDataWrite>(&self, writer: &mut W, input: Self::WriteInput) -> io::Result<()> {
        try!(writer.write_all(Self::TYPE_ID));
        try!(writer.write_i32::<LittleEndian>(self.content_size()));
        try!(writer.write_u32::<LittleEndian>(self.scripts.len() as u32));
        let writer_offset = try!(writer.seek(io::SeekFrom::Current(0))) as u32;
        let first_script_offset = writer_offset + (self.scripts.len() as u32 * 4);
        // Write offset data
        for i in 0..self.scripts.len() as u32 {
            try!(writer.write_u32::<LittleEndian>(first_script_offset + (i * 8)));
        }
        // Write script data
        for s in &self.scripts {
            try!(writer.write_u32::<LittleEndian>(input[s.name_index] as u32));
            try!(writer.write_u32::<LittleEndian>(s.unknown));
        }
        Ok(())
    }
    fn content_size(&self) -> i32 {
        4 + (self.scripts.len() as i32 * (4 + (2 * 4)))
    }
}

impl<'a> Chunk<'a> for Variables {
    const TYPE_ID: &'static [u8; 4] = b"VARI";
    type ReadOutput = (Self, Vec<u32>);
    type WriteInput = &'a [i32];
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        let header = try!(get_chunk_header(reader, Self::TYPE_ID));
        let mut offsets = Vec::new();
        let mut vars = Vec::new();
        let mut remaining = header.size;
        while remaining > 0 {
            let offset = try!(reader.read_u32::<LittleEndian>());
            let unk = try!(reader.read_u32::<LittleEndian>());
            let code_offset = try!(reader.read_u32::<LittleEndian>());
            trace!("unk {} code_offset {}", unk, code_offset);
            vars.push(Variable {
                name_index: 0,
                unknown: unk,
                code_offset: code_offset,
            });
            offsets.push(offset);
            remaining -= 3 * 4;
        }
        Ok((Variables { variables: vars }, offsets))
    }
    fn write<W: GameDataWrite>(&self, writer: &mut W, input: Self::WriteInput) -> io::Result<()> {
        try!(writer.write_all(Self::TYPE_ID));
        let len = self.content_size();
        try!(writer.write_i32::<LittleEndian>(len));
        for var in &self.variables {
            try!(writer.write_u32::<LittleEndian>(input[var.name_index] as u32));
            try!(writer.write_u32::<LittleEndian>(var.unknown));
            try!(writer.write_u32::<LittleEndian>(var.code_offset));
        }
        Ok(())
    }
    fn content_size(&self) -> i32 {
        (self.variables.len() * (3 * 4)) as i32
    }
}

impl<'a> Chunk<'a> for Functions {
    const TYPE_ID: &'static [u8; 4] = b"FUNC";
    type ReadOutput = (Self, Vec<u32>);
    type WriteInput = &'a [i32];
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        let header = try!(get_chunk_header(reader, Self::TYPE_ID));
        let mut offsets = Vec::new();
        let mut funs = Vec::new();
        let mut remaining = header.size;
        while remaining > 0 {
            let offset = try!(reader.read_u32::<LittleEndian>());
            let unk = try!(reader.read_u32::<LittleEndian>());
            let code_offset = try!(reader.read_u32::<LittleEndian>());
            trace!("unk {}, code offset {}", unk, code_offset);
            funs.push(Function {
                name_index: 0,
                unknown: unk,
                code_offset: code_offset,
            });
            offsets.push(offset);
            remaining -= 3 * 4;
        }
        Ok((Functions { functions: funs }, offsets))
    }
    fn write<W: GameDataWrite>(&self, writer: &mut W, input: Self::WriteInput) -> io::Result<()> {
        try!(writer.write_all(Self::TYPE_ID));
        let len = self.content_size();
        try!(writer.write_i32::<LittleEndian>(len));
        for fun in &self.functions {
            try!(writer.write_u32::<LittleEndian>(input[fun.name_index] as u32));
            try!(writer.write_u32::<LittleEndian>(fun.unknown));
            try!(writer.write_u32::<LittleEndian>(fun.code_offset));
        }
        Ok(())
    }
    fn content_size(&self) -> i32 {
        (self.functions.len() * (3 * 4)) as i32
    }
}

impl<'a> Chunk<'a> for Strings {
    const TYPE_ID: &'static [u8; 4] = b"STRG";
    type ReadOutput = (Self, Vec<u32>);
    type WriteInput = i32;
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
    fn write<W: GameDataWrite>(&self, writer: &mut W, offset: i32) -> io::Result<()> {
        try!(writer.write_all(Self::TYPE_ID));
        try!(writer.write_i32::<LittleEndian>(self.content_size()));
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
    fn content_size(&self) -> i32 {
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

impl<'a> Chunk<'a> for Textures {
    const TYPE_ID: &'static [u8; 4] = b"TXTR";
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Textures, ReadError> {
        let header = try!(get_chunk_header(reader, Self::TYPE_ID));
        let start_offset = try!(reader.seek(io::SeekFrom::Current(0)));
        let num_textures = try!(reader.read_u32::<LittleEndian>());
        trace!("{} textures", num_textures);
        // Read texture entry offsets
        for _ in 0..num_textures {
            // For now just discard them
            try!(reader.read_u32::<LittleEndian>());
        }
        let reader_offset = try!(reader.seek(io::SeekFrom::Current(0))) as u32;
        let data_offset = reader_offset + (num_textures * 8);
        let mut textures = Vec::new();
        for _ in 0..num_textures {
            let unk = try!(reader.read_u32::<LittleEndian>());
            let offset = try!(reader.read_u32::<LittleEndian>());
            trace!("unk: {}, offset: {}", unk, offset - data_offset);
            textures.push(Texture {
                unknown: unk,
                offset: offset - data_offset,
            });
        }
        let rel_offset = try!(reader.seek(io::SeekFrom::Current(0))) - start_offset;
        let data = try!(read_into_byte_vec(reader, header.size - rel_offset as usize));
        Ok(Textures {
            textures: textures,
            texture_data: data,
        })
    }
    fn write<W: GameDataWrite>(&self, writer: &mut W, _input: ()) -> io::Result<()> {
        try!(writer.write_all(Self::TYPE_ID));
        try!(writer.write_i32::<LittleEndian>(self.content_size()));
        try!(writer.write_u32::<LittleEndian>(self.textures.len() as u32));
        let writer_offset = try!(writer.seek(io::SeekFrom::Current(0)));
        let num_textures = self.textures.len() as u32;
        // Write offset table
        for i in 0..num_textures {
            let offset_table_len = num_textures * 4;
            try!(writer.write_u32::<LittleEndian>(writer_offset as u32 + offset_table_len +
                                                  (i * 8)));
        }
        let writer_offset = try!(writer.seek(io::SeekFrom::Current(0)));
        let texture_data_offset = writer_offset as u32 + (num_textures * 8);
        for t in &self.textures {
            try!(writer.write_u32::<LittleEndian>(t.unknown));
            try!(writer.write_u32::<LittleEndian>(texture_data_offset + t.offset));
        }
        try!(writer.write_all(&self.texture_data));
        Ok(())
    }
    fn content_size(&self) -> i32 {
        let num_textures = self.textures.len();
        let num_textures_size = 4;
        let texture_offsets_size = num_textures * 4;
        let texture_entries_size = num_textures * 8;
        (num_textures_size + texture_offsets_size + texture_entries_size +
         self.texture_data.len()) as i32
    }
}

impl<'a> Chunk<'a> for Audio {
    const TYPE_ID: &'static [u8; 4] = b"AUDO";
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        let header = try!(get_chunk_header(reader, Self::TYPE_ID));
        let num_audio = try!(reader.read_u32::<LittleEndian>());
        trace!("num audio: {}", num_audio);
        // Get offsets
        let mut offsets = Vec::new();
        let audio_data_offset = try!(reader.seek(io::SeekFrom::Current(0))) as u32 +
                                (num_audio * 4);
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
            size: header.size as u32,
        })
    }
    fn write<W: GameDataWrite>(&self, writer: &mut W, _: ()) -> io::Result<()> {
        try!(writer.write_all(Self::TYPE_ID));
        try!(writer.write_i32::<LittleEndian>(self.content_size()));
        try!(writer.write_u32::<LittleEndian>(self.audio.len() as u32));
        let audio_data_offset = try!(writer.seek(io::SeekFrom::Current(0))) as u32 +
                                (self.offsets.len() as u32 * 4);
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
    fn content_size(&self) -> i32 {
        self.size as i32
    }
}

fn texture_data_offset(textures: &Textures, base_offset: u32) -> u32 {
    let mut offset = base_offset + CHUNK_HEADER_LEN as u32;
    let num_textures = textures.textures.len() as u32;
    offset += 4; // num_textures
    offset += num_textures * 4; // offset table
    offset += num_textures * 8; // texture table
    offset
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

struct ChunkHeader {
    type_id: [u8; TYPE_ID_LEN],
    size: usize,
}

fn read_chunk_header<R: GameDataRead>(reader: &mut R) -> Result<ChunkHeader, ReadError> {
    let offset = try!(reader.seek(io::SeekFrom::Current(0)));
    let mut type_id = [0u8; TYPE_ID_LEN];
    try!(reader.read_exact(&mut type_id));
    let size = try!(reader.read_i32::<LittleEndian>());
    info!("Read chunk {} with size {:>9} @ {:>9}",
          String::from_utf8_lossy(&type_id),
          size,
          offset);
    Ok(ChunkHeader {
        type_id: type_id,
        size: size as usize,
    })
}

fn get_chunk_header<R: GameDataRead>(reader: &mut R,
                                     should_be: &[u8])
                                     -> Result<ChunkHeader, ReadError> {
    let header = try!(read_chunk_header(reader));
    if &header.type_id == should_be {
        Ok(header)
    } else {
        Err(ReadError::InvalidChunkTypeId(header.type_id))
    }
}

fn read_into_byte_vec<R: GameDataRead>(reader: &mut R, len: usize) -> Result<Vec<u8>, io::Error> {
    let mut vec = Vec::with_capacity(len);
    unsafe {
        vec.set_len(len);
        try!(reader.read_exact(&mut vec));
    }
    Ok(vec)
}
