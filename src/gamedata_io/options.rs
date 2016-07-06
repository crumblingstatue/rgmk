use std::io;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use {GameDataRead, GameDataWrite};
use gamedata_io::{Chunk, ReadError, get_chunk_header};
use unexposed::Options;

#[derive(Clone, Copy)]
pub struct Offsets {
    pub icon_offset: u32,
    pub const1_offset: u32,
    pub const2_offset: u32,
    pub const3_offset: u32,
    pub const4_offset: u32,
    pub const5_offset: u32,
    pub const6_offset: u32,
    pub const7_offset: u32,
    pub const8_offset: u32,
    pub const9_offset: u32,
    pub const10_offset: u32,
    pub const11_offset: u32,
    pub const12_offset: u32,
    pub const13_offset: u32,
    pub const14_offset: u32,
}

pub fn write_offsets<W: GameDataWrite>(options: &Options,
                                       writer: &mut W,
                                       texture_data_offset: u32,
                                       string_offsets: &[u32])
                                       -> io::Result<()> {
    try!(writer.seek(io::SeekFrom::Current(2 * 4)));
    try!(writer.write_u32::<LittleEndian>(texture_data_offset + options.icon_offset));
    try!(writer.seek(io::SeekFrom::Current(13 * 4)));
    try!(writer.write_u32::<LittleEndian>(string_offsets[options.constant1_name_index]));
    try!(writer.write_u32::<LittleEndian>(string_offsets[options.constant2_name_index]));
    try!(writer.write_u32::<LittleEndian>(string_offsets[options.constant3_name_index]));
    try!(writer.write_u32::<LittleEndian>(string_offsets[options.constant4_name_index]));
    try!(writer.write_u32::<LittleEndian>(string_offsets[options.constant5_name_index]));
    try!(writer.write_u32::<LittleEndian>(string_offsets[options.constant6_name_index]));
    try!(writer.write_u32::<LittleEndian>(string_offsets[options.constant7_name_index]));
    try!(writer.write_u32::<LittleEndian>(string_offsets[options.constant8_name_index]));
    try!(writer.write_u32::<LittleEndian>(string_offsets[options.constant9_name_index]));
    try!(writer.write_u32::<LittleEndian>(string_offsets[options.constant10_name_index]));
    try!(writer.write_u32::<LittleEndian>(string_offsets[options.constant11_name_index]));
    try!(writer.write_u32::<LittleEndian>(string_offsets[options.constant12_name_index]));
    try!(writer.write_u32::<LittleEndian>(string_offsets[options.constant13_name_index]));
    try!(writer.write_u32::<LittleEndian>(string_offsets[options.constant14_name_index]));
    Ok(())
}

impl<'a> Chunk<'a> for Options {
    const TYPE_ID: &'static [u8; 4] = b"OPTN";
    type ReadOutput = (Self, Offsets);
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
            Offsets {
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
    fn write_content<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
        try!(writer.write_u32::<LittleEndian>(self.unk1));
        try!(writer.write_u32::<LittleEndian>(self.unk2));
        // We'll write this later
        try!(writer.seek(io::SeekFrom::Current(4)));
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
        // We'll write these later
        try!(writer.seek(io::SeekFrom::Current(14 * 4)));
        Ok(())
    }
}
