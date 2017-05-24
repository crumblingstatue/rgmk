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

pub(super) fn write_offsets<W: GameDataWrite>(
    meta_data: &MetaData,
    writer: &mut W,
    offsets: &[u32],
) -> io::Result<()> {
    writer.seek(io::SeekFrom::Current(4))?;
    trace!(
        "Writing {} at offset {}",
        offsets[meta_data.game_id_1_index],
        writer.tell()?
    );
    writer
        .write_u32::<LittleEndian>(offsets[meta_data.game_id_1_index])?;
    writer
        .write_u32::<LittleEndian>(offsets[meta_data.default_index])?;
    writer.seek(io::SeekFrom::Current(7 * 4))?;
    writer
        .write_u32::<LittleEndian>(offsets[meta_data.game_id_2_index])?;
    writer.seek(io::SeekFrom::Current(14 * 4))?;
    writer
        .write_u32::<LittleEndian>(offsets[meta_data.window_title_index])?;
    Ok(())
}

impl<'a> Chunk<'a> for MetaData {
    const TYPE_ID: &'static [u8; 4] = b"GEN8";
    type ReadOutput = (Self, Offsets);
    fn read<R: GameDataRead>(reader: &mut R) -> Result<Self::ReadOutput, ReadError> {
        let header = get_chunk_header(reader, Self::TYPE_ID)?;
        let possibly_gen8_version = reader.read_u32::<LittleEndian>()?;
        info!("We are dealing with GEN8 version {}", possibly_gen8_version);
        let game_id_1_offset = reader.read_u32::<LittleEndian>()?;
        let default_offset = reader.read_u32::<LittleEndian>()?;
        let unk2 = reader.read_u32::<LittleEndian>()?;
        let unk3 = reader.read_u32::<LittleEndian>()?;
        let unk4 = reader.read_u32::<LittleEndian>()?;
        let unk5 = reader.read_u32::<LittleEndian>()?;
        let unk6 = reader.read_u32::<LittleEndian>()?;
        let unk7 = reader.read_u32::<LittleEndian>()?;
        let unk8 = reader.read_u32::<LittleEndian>()?;
        let game_id_2_offset = reader.read_u32::<LittleEndian>()?;
        let unk9 = reader.read_u32::<LittleEndian>()?;
        let unk10 = reader.read_u32::<LittleEndian>()?;
        let unk11 = reader.read_u32::<LittleEndian>()?;
        let unk12 = reader.read_u32::<LittleEndian>()?;
        let window_width = reader.read_u32::<LittleEndian>()?;
        let window_height = reader.read_u32::<LittleEndian>()?;
        let unk13 = reader.read_u32::<LittleEndian>()?;
        let unk14 = reader.read_u32::<LittleEndian>()?;
        let unk15 = reader.read_u32::<LittleEndian>()?;
        let unk16 = reader.read_u32::<LittleEndian>()?;
        let unk17 = reader.read_u32::<LittleEndian>()?;
        let unk18 = reader.read_u32::<LittleEndian>()?;
        let unk19 = reader.read_u32::<LittleEndian>()?;
        let unk20 = reader.read_u32::<LittleEndian>()?;
        let window_title_offset = reader.read_u32::<LittleEndian>()?;
        let mut remaining = header.size - (26 * 4);
        let mut values = Vec::new();
        while remaining > 0 {
            let value = reader.read_u32::<LittleEndian>()?;
            values.push(value);
            remaining -= 4;
        }
        Ok((
            MetaData {
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
            },
        ))
    }
    fn write_content<W: GameDataWrite>(&self, writer: &mut W) -> io::Result<()> {
        writer
            .write_u32::<LittleEndian>(self.possibly_gen8_version)?;
        // String offsets, writing later
        writer.seek(io::SeekFrom::Current(8))?;
        writer.write_u32::<LittleEndian>(self.unk2)?;
        writer.write_u32::<LittleEndian>(self.unk3)?;
        writer.write_u32::<LittleEndian>(self.unk4)?;
        writer.write_u32::<LittleEndian>(self.unk5)?;
        writer.write_u32::<LittleEndian>(self.unk6)?;
        writer.write_u32::<LittleEndian>(self.unk7)?;
        writer.write_u32::<LittleEndian>(self.unk8)?;
        // String offset, writing later
        writer.seek(io::SeekFrom::Current(4))?;
        writer.write_u32::<LittleEndian>(self.unk9)?;
        writer.write_u32::<LittleEndian>(self.unk10)?;
        writer.write_u32::<LittleEndian>(self.unk11)?;
        writer.write_u32::<LittleEndian>(self.unk12)?;
        writer.write_u32::<LittleEndian>(self.window_width)?;
        writer.write_u32::<LittleEndian>(self.window_height)?;
        writer.write_u32::<LittleEndian>(self.unk13)?;
        writer.write_u32::<LittleEndian>(self.unk14)?;
        writer.write_u32::<LittleEndian>(self.unk15)?;
        writer.write_u32::<LittleEndian>(self.unk16)?;
        writer.write_u32::<LittleEndian>(self.unk17)?;
        writer.write_u32::<LittleEndian>(self.unk18)?;
        writer.write_u32::<LittleEndian>(self.unk19)?;
        writer.write_u32::<LittleEndian>(self.unk20)?;
        // String offset, writing later
        writer.seek(io::SeekFrom::Current(4))?;
        for &v in &self.unknown {
            writer.write_u32::<LittleEndian>(v)?;
        }
        Ok(())
    }
}
