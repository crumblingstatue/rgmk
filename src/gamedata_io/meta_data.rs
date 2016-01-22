use std::io::prelude::*;
use std::io;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use {GameDataRead, GameDataWrite, MetaData};
use gamedata_io::{Chunk, get_chunk_header, ReadError, Tell};

#[derive(Clone, Copy)]
pub struct Offsets {
    pub game_id_1: u32,
    pub default: u32,
    pub game_id_2: u32,
    pub window_title: u32,
}

pub fn write_offsets<W: GameDataWrite>(meta_data: &MetaData,
                                       writer: &mut W,
                                       offsets: &[u32])
                                       -> io::Result<()> {
    try!(writer.seek(io::SeekFrom::Current(4)));
    trace!("Writing {} at offset {}",
           offsets[meta_data.game_id_1_index],
           try!(writer.tell()));
    try!(writer.write_u32::<LittleEndian>(offsets[meta_data.game_id_1_index]));
    try!(writer.write_u32::<LittleEndian>(offsets[meta_data.default_index]));
    try!(writer.seek(io::SeekFrom::Current(7 * 4)));
    try!(writer.write_u32::<LittleEndian>(offsets[meta_data.game_id_2_index]));
    try!(writer.seek(io::SeekFrom::Current(14 * 4)));
    try!(writer.write_u32::<LittleEndian>(offsets[meta_data.window_title_index]));
    Ok(())
}

impl<'a> Chunk<'a> for MetaData {
    const TYPE_ID: &'static [u8; 4] = b"GEN8";
    type ReadOutput = (Self, Offsets);
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        let header = try!(get_chunk_header(reader, Self::TYPE_ID));
        let possibly_gen8_version = try!(reader.read_u32::<LittleEndian>());
        info!("We are dealing with GEN8 version {}", possibly_gen8_version);
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
            possibly_gen8_version: possibly_gen8_version,
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
            Offsets {
            game_id_1: game_id_1_offset,
            default: default_offset,
            game_id_2: game_id_2_offset,
            window_title: window_title_offset,
        }))
    }
    fn write_content<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
        try!(writer.write_u32::<LittleEndian>(self.possibly_gen8_version));
        // String offsets, writing later
        try!(writer.seek(io::SeekFrom::Current(8)));
        try!(writer.write_u32::<LittleEndian>(self.unk2));
        try!(writer.write_u32::<LittleEndian>(self.unk3));
        try!(writer.write_u32::<LittleEndian>(self.unk4));
        try!(writer.write_u32::<LittleEndian>(self.unk5));
        try!(writer.write_u32::<LittleEndian>(self.unk6));
        try!(writer.write_u32::<LittleEndian>(self.unk7));
        try!(writer.write_u32::<LittleEndian>(self.unk8));
        // String offset, writing later
        try!(writer.seek(io::SeekFrom::Current(4)));
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
        // String offset, writing later
        try!(writer.seek(io::SeekFrom::Current(4)));
        for &v in &self.unknown {
            try!(writer.write_u32::<LittleEndian>(v));
        }
        Ok(())
    }
}
