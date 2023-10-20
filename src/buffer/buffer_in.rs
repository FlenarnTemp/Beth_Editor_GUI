use std::fs;
use std::io::{self, Read, Write};

struct ByteBuffer_In {
    data: Vec<u8>,
    offset: usize,
}

impl ByteBuffer_In {
    fn new(data: Vec<u8>) -> Self {
        ByteBuffer_In { data, offset: 0 }
    }

    fn available(&self) -> usize {
        self.data.len() - self.offset
    }

    fn load(path: &str) -> io::Result<Self> {
        let mut file = fs::File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(ByteBuffer_In::new(data))
    }

    fn clone(&self) -> Self {
        ByteBuffer_In {
            data: self.data.clone(),
            offset: self.offset,
        }
    }

    fn save(&self, path: &str) -> io::Result<()> {
        let mut file = fs::File::create(path)?;
        file.write_all(&self.data)?;
        Ok(())
    }

    fn read_byte(&mut self) -> Option<u8> {
        if self.available() > 0 {
            let byte = self.data[self.offset];
            self.offset += 1;
            Some(byte)
        } else {
            None
        }
    }
}
