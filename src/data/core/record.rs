use std::io::Read;

use flate2::read::{DeflateDecoder};

use crate::buffer::bytebuffer_in as buffer;

const FLAG_DEFLATE: u32 = 0x00040000;

pub struct RecordAndData {
    pub record: Record,
    pub data: buffer::ByteBuffer_In,
}

pub struct Record {
    pub type_: String,
    pub data_len: u32,
    pub flags: u32,
    pub id: u32,
    pub timestamp: u16,
    pub vc1: u16,
    pub version: u16,
    pub vc2: u16,
}

impl Record {
    pub fn read_COMMON(buffer: &mut buffer::ByteBuffer_In) -> Option<RecordAndData> {
        let record = Record {
            type_: buffer.read_string(4)?,
            data_len: buffer.read_dword()?,
            flags: buffer.read_dword()?,
            id: buffer.read_dword()?,
            timestamp: buffer.read_word()?,
            vc1: buffer.read_word()?,
            version: buffer.read_word()?,
            vc2: buffer.read_word()?,
        };

        let mut data = buffer::ByteBuffer_In {
            data: buffer.data[buffer.offset..(buffer.offset + record.data_len as usize)].to_vec(),
            offset: 0,
        };

        buffer.offset += record.data_len as usize;

        // Handle compressed records.
        if (record.flags & FLAG_DEFLATE) != 0 {
            let decompressed_len = data.read_dword()?;
            let mut decompressed = Vec::with_capacity(decompressed_len as usize);
            let mut decoder = DeflateDecoder::new(&data.data[data.offset..]);
            decoder.read_exact(&mut decompressed).unwrap();
            data.data = decompressed;
            data.offset = 0;
        }

        Some(RecordAndData { record, data })
    }
}
