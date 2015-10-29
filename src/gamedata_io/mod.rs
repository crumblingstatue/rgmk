//! Reading GameData from data.win format

use std::io::prelude::*;
use std::io;
use byteorder::{self, ReadBytesExt, WriteBytesExt, LittleEndian};
use super::{GameData, MetaData, Options, Extn, Sounds, AudioGroups, Sprites, Backgrounds, Paths,
            Scripts, Shaders, Fonts, Timelines, Objects, Rooms, Dafl, Tpag, Code, Variables,
            Functions, Strings, Textures, Audio, GameDataRead, GameDataWrite};

mod meta_data;
mod options;
mod sounds;
mod scripts;
mod variables;
mod functions;
mod strings;
mod textures;
mod audio;
mod fonts;

pub use self::strings::StringReadError;

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
const CHUNK_HEADER_LEN: u32 = TYPE_ID_LEN as u32 + 4;

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
    let (mut fonts, font_string_offsets) = try!(Fonts::read(reader));
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
    for (i, f) in font_string_offsets.into_iter().enumerate() {
        trace!("Assinging string indexes for font {}", i);
        for (j, &soff) in offsets.iter().enumerate() {
            if f.name - 4 == soff {
                fonts.fonts[i].name_index = j;
                trace!("name: {}", strings.strings[j]);
            }
            if f.font_name - 4 == soff {
                fonts.fonts[i].font_name_index = j;
                trace!("font name: {}", strings.strings[j]);
                trace!("point size: {}", fonts.fonts[i].point_size);
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

fn form_content_len(data: &GameData) -> u32 {
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
    try!(writer.write_u32::<LittleEndian>(form_content_len(data)));
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
                                                 stringtable_offset + data.strings.content_size() +
                                                 CHUNK_HEADER_LEN))));
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
    try!(data.fonts.write(writer, &string_offsets));
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

fn string_offsets(strings: &Strings, base_offset: u32) -> Vec<u32> {
    let mut offset = base_offset + CHUNK_HEADER_LEN + 4 + (strings.strings.len() as u32 * 4);
    let mut offsets = Vec::new();
    for string in &strings.strings {
        // +4 because functions point right into the string
        offsets.push(offset + 4);
        offset += (string.len() + 1) as u32 + 4;
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
    fn content_size(&self) -> u32;
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
                try!(writer.write_u32::<LittleEndian>(self.content_size()));
                try!(writer.write_all(&self.raw));
                Ok(())
            }
            fn content_size(&self) -> u32 {
                self.raw.len() as u32
            }
        }
    }
}

unk_chunk!(Extn, b"EXTN");
unk_chunk!(AudioGroups, b"AGRP");
unk_chunk!(Sprites, b"SPRT");
unk_chunk!(Backgrounds, b"BGND");
unk_chunk!(Paths, b"PATH");
unk_chunk!(Shaders, b"SHDR");
unk_chunk!(Timelines, b"TMLN");
unk_chunk!(Objects, b"OBJT");
unk_chunk!(Rooms, b"ROOM");
unk_chunk!(Dafl, b"DAFL");
unk_chunk!(Tpag, b"TPAG");
unk_chunk!(Code, b"CODE");

fn texture_data_offset(textures: &Textures, base_offset: u32) -> u32 {
    let mut offset = base_offset + CHUNK_HEADER_LEN;
    let num_textures = textures.textures.len() as u32;
    offset += 4; // num_textures
    offset += num_textures * 4; // offset table
    offset += num_textures * 8; // texture table
    offset
}

struct ChunkHeader {
    type_id: [u8; TYPE_ID_LEN],
    size: usize,
}

fn read_chunk_header<R: GameDataRead>(reader: &mut R) -> Result<ChunkHeader, ReadError> {
    let offset = try!(reader.seek(io::SeekFrom::Current(0)));
    let mut type_id = [0u8; TYPE_ID_LEN];
    try!(reader.read_exact(&mut type_id));
    let size = try!(reader.read_u32::<LittleEndian>());
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
