extern crate rgmk;

use rgmk::ChunkContent;

fn main() {
    let path = std::env::args_os().skip(1).next().expect("Expected path as argument");
    if let ChunkContent::Form(chunks) = rgmk::load_from_file(path).unwrap().content {
        for chunk in chunks {
            if let ChunkContent::StringTable { strings, .. } = chunk.content {
                for string in strings {
                    println!("{}", string)
                }
            }
        }
    }
}
