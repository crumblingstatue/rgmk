use std::io::{self, Seek, SeekFrom};

pub trait Tell: Seek {
    fn tell(&mut self) -> io::Result<u64>;
}

impl<T: Seek> Tell for T {
    fn tell(&mut self) -> io::Result<u64> {
        self.seek(SeekFrom::Current(0))
    }
}
