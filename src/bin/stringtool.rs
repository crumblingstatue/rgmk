extern crate rgmk;

use rgmk::{ChunkContent, StringTable};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn dump_strings(table: &StringTable) {
    for string in &table.strings {
        println!("{}", string)
    }
}

fn rebuild_from_strings<P: AsRef<Path>>(path: P, table_offset: u32) -> io::Result<StringTable> {
    let file = try!(File::open(path));
    let reader = BufReader::new(file);
    let mut offsets = Vec::new();
    let mut strings = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        strings.push(line);
    }
    let count = strings.len();
    // number_of_strings_value + offsets
    let offsets_len: u32 = 4 + (4 * count as u32);
    let mut offset: u32 = table_offset;
    for i in 0..count {
        offsets.push(offsets_len + offset);
        // length_value + characters + null byte
        offset += 4 + strings[i].len() as u32 + 1;
    }
    Ok(StringTable {
        offsets: offsets,
        strings: strings,
    })
}

fn main() {
    let mut args = std::env::args_os().skip(1);
    let mode = args.next().expect("Expected dump/rebuild as 1st argument");
    let mode = mode.into_string().unwrap();
    let path = args.next().expect("Expected path as 2nd argument");
    let mut root = rgmk::load_from_file(&path).unwrap();
    let mut offset = rgmk::CHUNK_HEADER_LEN as i32;
    if let ChunkContent::Form(ref mut chunks) = root.content {
        let mut insert_at = 0;
        let mut table_overw = None;
        for (i, chunk) in chunks.iter_mut().enumerate() {
            offset += rgmk::CHUNK_HEADER_LEN as i32;
            if let ChunkContent::StringTable(ref table) = chunk.content {
                match &mode[..] {
                    "dump" => {
                        dump_strings(&table);
                        return;
                    }
                    "rebuild" => {
                        let strings_path = args.next()
                                               .expect("Expected strings path as 3nd argument");
                        let table = rebuild_from_strings(&strings_path, offset as u32).unwrap();
                        insert_at = i;
                        table_overw = Some(table.clone());
                    }
                    _ => {
                        println!("Invalid mode \"{}\"", mode);
                        return;
                    }
                }
            }
            offset += chunk.size;
        }
        chunks[insert_at].content = ChunkContent::StringTable(table_overw.unwrap());
    }
    root.save_to_file(path).unwrap();
}
