use std::io::prelude::*;
use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use {GameDataRead, GameDataWrite, Options};
use gamedata_io::{Chunk, get_chunk_header, ReadError};

#[derive(Clone, Copy)]
struct Offsets {
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

impl<'a> Chunk<'a> for Options {
    const TYPE_ID: &'static [u8; 4] = b"OPTN";
    type ReadOutput = (Self, Offsets);
    type WriteInput = (&'a [u32], u32);
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
    chunk_write_impl!();
    fn write_content<W: GameDataWrite>(&self,
                                       writer: &mut W,
                                       (input, texture_data_offset): Self::WriteInput)
                                       -> io::Result<()> {
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
        try!(writer.write_u32::<LittleEndian>(input[self.constant1_name_index]));
        try!(writer.write_u32::<LittleEndian>(input[self.constant2_name_index]));
        try!(writer.write_u32::<LittleEndian>(input[self.constant3_name_index]));
        try!(writer.write_u32::<LittleEndian>(input[self.constant4_name_index]));
        try!(writer.write_u32::<LittleEndian>(input[self.constant5_name_index]));
        try!(writer.write_u32::<LittleEndian>(input[self.constant6_name_index]));
        try!(writer.write_u32::<LittleEndian>(input[self.constant7_name_index]));
        try!(writer.write_u32::<LittleEndian>(input[self.constant8_name_index]));
        try!(writer.write_u32::<LittleEndian>(input[self.constant9_name_index]));
        try!(writer.write_u32::<LittleEndian>(input[self.constant10_name_index]));
        try!(writer.write_u32::<LittleEndian>(input[self.constant11_name_index]));
        try!(writer.write_u32::<LittleEndian>(input[self.constant12_name_index]));
        try!(writer.write_u32::<LittleEndian>(input[self.constant13_name_index]));
        try!(writer.write_u32::<LittleEndian>(input[self.constant14_name_index]));
        Ok(())
    }
    fn content_size(&self) -> u32 {
        30 * 4
    }
}
