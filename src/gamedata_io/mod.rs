//! Implementation of Reading/Writing the Game Maker Studio data format.

use std::io::{self, Seek};
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
mod sprites;
mod code;

pub use self::strings::StringReadError;

trait Tell: Seek {
    fn tell(&mut self) -> io::Result<u64> {
        self.seek(io::SeekFrom::Current(0))
    }
}

impl<T: Seek> Tell for T {}

quick_error! {
    #[derive(Debug)]
    /// Error when reading GameData from a reader.
    pub enum ReadError {
        /// An I/O error.
        Io(err: io::Error) {
            from()
            display("I/O error: {}", err)
        }
        /// A byte order error.
        ByteOrder(err: byteorder::Error) {
            from()
            display("Byte order error: {}", err)
        }
        /// A string read error.
        String(err: StringReadError) {
            from()
            display("Error when reading string: {}", err)
        }
        /// Expected a specific type of chunk, got something else.
        UnexpectedChunk(what: [u8; 4], expected: &'static [u8; 4]) {
            display("Unexpected chunk type: \"{}\" ({:?}). Expected: \"{}\" ({:?})",
                    String::from_utf8_lossy(what),
                    what,
                    String::from_utf8_lossy(*expected),
                    expected)
        }
    }
}

const TYPE_ID_LEN: usize = 4;
const CHUNK_HEADER_LEN: u32 = TYPE_ID_LEN as u32 + 4;

pub fn read<R: GameDataRead>(reader: &mut R) -> Result<GameData, ReadError> {
    get_chunk_header(reader, b"FORM")?;
    let (mut meta, meta_string_offsets) = MetaData::read(reader)?;
    let (mut opts, opt_offsets) = Options::read(reader)?;
    let extn = Extn::read(reader)?;
    let (mut sounds, sound_string_offsets) = Sounds::read(reader)?;
    let audio_groups = AudioGroups::read(reader)?;
    let (mut sprites, sprite_name_offsets) = Sprites::read(reader)?;
    let backgrounds = Backgrounds::read(reader)?;
    let paths = Paths::read(reader)?;
    let (mut scripts, script_name_offsets) = Scripts::read(reader)?;
    let shaders = Shaders::read(reader)?;
    let (mut fonts, font_string_offsets) = Fonts::read(reader)?;
    let timelines = Timelines::read(reader)?;
    let objects = Objects::read(reader)?;
    let rooms = Rooms::read(reader)?;
    let dafl = Dafl::read(reader)?;
    let tpag = Tpag::read(reader)?;
    let (mut code, code_name_offsets) = Code::read(reader)?;
    let (mut variables, var_name_offsets) = Variables::read(reader)?;
    let (mut functions, fun_name_offsets) = Functions::read(reader)?;
    let (strings, offsets) = Strings::read(reader)?;
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
    for (i, off) in sprite_name_offsets.into_iter().enumerate() {
        for (j, &soff) in offsets.iter().enumerate() {
            if off - 4 == soff {
                sprites.sprites[i].name_index = j;
                break;
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
    for (i, off) in code_name_offsets.into_iter().enumerate() {
        for (j, &soff) in offsets.iter().enumerate() {
            if off - 4 == soff {
                code.code_chunks[i].name_index = j;
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
    let textures_offset = reader.tell()? as u32 + CHUNK_HEADER_LEN;
    let textures = Textures::read(reader)?;
    let texture_data_offset = texture_data_offset(&textures, textures_offset);
    opts.icon_offset = opt_offsets.icon_offset - texture_data_offset;
    let audio = Audio::read(reader)?;
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

pub fn write<W: GameDataWrite>(data: &GameData, writer: &mut W) -> io::Result<()> {
    writer.write_all(b"FORM")?;
    // Skip writing the chunk size, we'll go back to it later
    let size_offset = writer.tell()?;
    writer.seek(io::SeekFrom::Current(4))?;
    let metadata_offset = writer.tell()? + CHUNK_HEADER_LEN as u64;
    data.metadata.write(writer)?;
    let options_offset = writer.tell()? + CHUNK_HEADER_LEN as u64;
    data.options.write(writer)?;
    data.extn.write(writer)?;
    let sounds_offset = writer.tell()? + CHUNK_HEADER_LEN as u64;
    data.sounds.write(writer)?;
    if let Some(ref agrp) = data.audio_groups {
        agrp.write(writer)?;
    }
    let sprites_offset = writer.tell()? + CHUNK_HEADER_LEN as u64;
    data.sprites.write(writer)?;
    data.backgrounds.write(writer)?;
    data.paths.write(writer)?;
    let scripts_offset = writer.tell()? + CHUNK_HEADER_LEN as u64;
    data.scripts.write(writer)?;
    data.shaders.write(writer)?;
    let fonts_offset = writer.tell()? + CHUNK_HEADER_LEN as u64;
    data.fonts.write(writer)?;
    data.timelines.write(writer)?;
    data.objects.write(writer)?;
    data.rooms.write(writer)?;
    data.dafl.write(writer)?;
    data.tpag.write(writer)?;
    let code_offset = writer.tell()? + CHUNK_HEADER_LEN as u64;
    data.code.write(writer)?;
    let variables_offset = writer.tell()? + CHUNK_HEADER_LEN as u64;
    data.variables.write(writer)?;
    let functions_offset = writer.tell()? + CHUNK_HEADER_LEN as u64;
    data.functions.write(writer)?;
    let strings_offset = writer.tell()? + CHUNK_HEADER_LEN as u64;
    let string_offsets = string_offsets(&data.strings, strings_offset as u32);
    data.strings.write(writer)?;
    let textures_offset = writer.tell()? + CHUNK_HEADER_LEN as u64;
    let texture_data_offset = texture_data_offset(&data.textures, textures_offset as u32);
    data.textures.write(writer)?;
    data.audio.write(writer)?;
    let finished_offset = writer.tell()?;
    // Seek back and write offset data for chunks that need it
    writer.seek(io::SeekFrom::Start(code_offset))?;
    code::write_offsets(&data.code, writer, &string_offsets)?;
    writer.seek(io::SeekFrom::Start(metadata_offset))?;
    meta_data::write_offsets(&data.metadata, writer, &string_offsets)?;
    writer.seek(io::SeekFrom::Start(options_offset))?;
    options::write_offsets(&data.options, writer, texture_data_offset, &string_offsets)?;
    writer.seek(io::SeekFrom::Start(sounds_offset))?;
    sounds::write_offsets(&data.sounds, writer, &string_offsets)?;
    writer.seek(io::SeekFrom::Start(sprites_offset))?;
    sprites::write_offsets(&data.sprites, writer, &string_offsets)?;
    writer.seek(io::SeekFrom::Start(scripts_offset))?;
    scripts::write_offsets(&data.scripts, writer, &string_offsets)?;
    writer.seek(io::SeekFrom::Start(fonts_offset))?;
    fonts::write_offsets(&data.fonts, writer, &string_offsets)?;
    writer.seek(io::SeekFrom::Start(variables_offset))?;
    variables::write_offsets(&data.variables, writer, &string_offsets)?;
    writer.seek(io::SeekFrom::Start(functions_offset))?;
    functions::write_offsets(&data.functions, writer, &string_offsets)?;
    // Now seek back and write the chunk size
    let size = (finished_offset - size_offset) - 4;
    writer.seek(io::SeekFrom::Start(size_offset))?;
    writer.write_u32::<LittleEndian>(size as u32)?;
    Ok(())
}

fn string_offsets(strings: &Strings, base_offset: u32) -> Vec<u32> {
    let mut offset = base_offset + 4 + (strings.strings.len() as u32 * 4);
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
    type ReadOutput;
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError>;
    fn write_content<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()>;
    fn write<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
        use gamedata_io::Tell;
        writer.write_all(Self::TYPE_ID)?;
        let size_offset = writer.tell()?;
        // Skip writing the content length, we'll write it later
        writer.seek(io::SeekFrom::Current(4))?;
        // Write the content
        self.write_content(writer)?;
        // Go back and write the content length
        let finished_offset = writer.tell()?;
        let size = (finished_offset - size_offset) - 4;
        writer.seek(io::SeekFrom::Start(size_offset))?;
        trace!("Writing chunk size: {} at offset {}", size, size_offset);
        writer.write_u32::<LittleEndian>(size as u32)?;
        // Seek back to where we came from
        writer.seek(io::SeekFrom::Start(finished_offset))?;
        trace!("Now back at offset {}", finished_offset);
        Ok(())
    }
}

macro_rules! unk_chunk {
    ($name:ident, $typeid:expr) => {
        impl<'a> Chunk<'a> for $name {
            const TYPE_ID: &'static [u8; 4] = $typeid;
            type ReadOutput = Self;
            fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
                let chunk_header = get_chunk_header(reader, Self::TYPE_ID)?;
                Ok($name {
                    raw: read_into_byte_vec(reader, chunk_header.size)?
                })
            }
            fn write_content<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
                writer.write_all(&self.raw)
            }
        }
    }
}

unk_chunk!(Extn, b"EXTN");
unk_chunk!(AudioGroups, b"AGRP");
unk_chunk!(Backgrounds, b"BGND");
unk_chunk!(Paths, b"PATH");
unk_chunk!(Shaders, b"SHDR");
unk_chunk!(Timelines, b"TMLN");
unk_chunk!(Objects, b"OBJT");
unk_chunk!(Rooms, b"ROOM");
unk_chunk!(Dafl, b"DAFL");
unk_chunk!(Tpag, b"TPAG");

fn texture_data_offset(textures: &Textures, base_offset: u32) -> u32 {
    let mut offset = base_offset;
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
    let mut type_id = [0u8; TYPE_ID_LEN];
    reader.read_exact(&mut type_id)?;
    let size = reader.read_u32::<LittleEndian>()?;
    Ok(ChunkHeader {
        type_id: type_id,
        size: size as usize,
    })
}

fn get_chunk_header<R: GameDataRead>(
    reader: &mut R,
    should_be: &'static [u8; 4],
) -> Result<ChunkHeader, ReadError> {
    let header = read_chunk_header(reader)?;
    if header.type_id == *should_be {
        let offset = reader.tell()?;
        info!(
            "Identified chunk {} with size {:>9} @ {:>9}",
            String::from_utf8_lossy(&header.type_id),
            header.size,
            offset - 8
        );
        Ok(header)
    } else {
        Err(ReadError::UnexpectedChunk(header.type_id, should_be))
    }
}

fn read_into_byte_vec<R: GameDataRead>(reader: &mut R, len: usize) -> Result<Vec<u8>, io::Error> {
    let mut vec = vec![0; len];
    reader.read_exact(&mut vec)?;
    Ok(vec)
}
