use std::fs;
use std::io::{self, Read, Write};

pub struct ByteBuffer_In {
    pub data: Vec<u8>,
    pub offset: usize,
}

impl ByteBuffer_In {
    pub fn new(data: Vec<u8>) -> Self {
        ByteBuffer_In { data, offset: 0 }
    }

    pub fn available(&self) -> usize {
        self.data.len() - self.offset
    }

    pub fn load(path: &str) -> io::Result<Self> {
        let mut file = fs::File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(ByteBuffer_In::new(data))
    }

    pub fn clone(&self) -> Self {
        ByteBuffer_In {
            data: self.data.clone(),
            offset: self.offset,
        }
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        let mut file = fs::File::create(path)?;
        file.write_all(&self.data)?;
        Ok(())
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        if self.available() > 0 {
            let byte = self.data[self.offset];
            self.offset += 1;
            Some(byte)
        } else {
            None
        }
    }

    pub fn read_word(&mut self) -> Option<u16> {
        let low = self.read_byte()? as u16;
        let high = self.read_byte()? as u16;
        Some(low | (high << 8))
    }

    pub fn read_dword(&mut self) -> Option<u32> {
        let low = self.read_word()? as u32;
        let high = self.read_word()? as u32;
        Some(low | (high << 16))
    }

    pub fn read_qword(&mut self) -> Option<u64> {
        let low = self.read_dword()? as u64;
        let high = self.read_dword()? as u64;
        Some(low | (high << 32))
    }

    pub fn read_float(&mut self) -> Option<f32> {
        let dword = self.read_dword()? as u32;
        let float = f32::from_bits(dword);
        Some(float)
    }

    pub fn read_string(&mut self, len: isize) -> Option<String> {
        let mut result = String::new();
        if len < 0 {
            while let Some(byte) = self.read_byte() {
                if byte == 0 {
                    break;
                }
                result.push(byte as char);
            }
        } else {
            for _ in 0..len {
                if let Some(byte) = self.read_byte() {
                    result.push(byte as char);
                } else {
                    break;
                }
            }
        }
        Some(result)
    }

    pub fn read(&mut self, len: usize) -> Option<Vec<u8>> {
        if self.available() >= len {
            let result = self.data[self.offset..(self.offset + len)].to_vec();
            self.offset += len;
            Some(result)
        } else {
            None
        }
    }
}
