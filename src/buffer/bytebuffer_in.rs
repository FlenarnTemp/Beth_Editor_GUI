use std::fs;
use std::io::{self, Read, Write};

#[derive(Debug)]
pub struct ByteBufferIn{
    pub data: Vec<u8>,
    pub offset: usize,
}

impl ByteBufferIn {
    pub fn new(data: Vec<u8>) -> Self {
        ByteBufferIn { data, offset: 0 }
    }

    pub fn available(&self) -> usize {
        self.data.len() - self.offset
    }

    pub fn load(path: &str) -> io::Result<Self> {
        let mut file = fs::File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(ByteBufferIn::new(data))
    }

    pub fn clone(&self) -> Self {
        ByteBufferIn {
            data: self.data.clone(),
            offset: self.offset,
        }
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        let mut file = fs::File::create(path)?;
        file.write_all(&self.data)?;
        Ok(())
    }

    pub fn read_u8(&mut self) -> u8 {
        if self.available() > 0 {
            let byte = self.data[self.offset];
            self.offset += 1;
            byte
        } else {
            panic!("Buffer empty on read attempt.")
        }
    }

    pub fn read_u8_vec(&mut self, len: usize) -> Vec<u8> {
        let start = self.offset;
        let end = self.offset + len;
        
        if end <= self.data.len() {
            let bytes = self.data[start..end].to_vec();
            self.offset += len;
            bytes
        } else {
            // TODO: Handle the case where there are not enough bytes available.
            // You can choose to panic, return an empty Vec, or handle it differently based on your needs.
            panic!("Not enough bytes available in the buffer");
        }
    }

    pub fn read_u16(&mut self) -> u16 {
        let low = self.read_u8() as u16;
        let high = self.read_u8() as u16;
        low | (high << 8)
    }

    pub fn read_i16(&mut self) -> i16 {
        let value = self.read_u16() as i16;
        value
    }

    pub fn read_u32(&mut self) -> u32 {
        let low = self.read_u16() as u32;
        let high = self.read_u16() as u32;
        low | (high << 16)
    }

    pub fn read_i32(&mut self) -> i32 {
        let value = self.read_u32() as i32;
        value
    }

    pub fn read_u64(&mut self) -> u64 {
        let low = self.read_u32() as u64;
        let high = self.read_u32() as u64;
        low | (high << 32)
    }

    pub fn read_float(&mut self) -> f32 {
        let dword = self.read_u32() as u32;
        let float = f32::from_bits(dword);
        float
    }

    pub fn read_string(&mut self, len: isize) -> String {
        let mut result = String::new();
        if len < 0 {
            while let byte = self.read_u8() {
                if byte == 0 {
                    break;
                }
                result.push(byte as char);
            }
        } else {
            for _ in 0..len {
                if let byte = self.read_u8() {
                    result.push(byte as char);
                } else {
                    break;
                }
            }
        }
        result
    }

    pub fn read(&mut self, len: usize) -> Vec<u8> {
        if self.available() >= len {
            let result = self.data[self.offset..(self.offset + len)].to_vec();
            self.offset += len;
            result
        } else {
            panic!("Buffer empty on read attempt.")
        }
    }
}
