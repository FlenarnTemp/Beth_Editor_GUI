use std::{io::Read, collections::HashMap};
use lazy_static::lazy_static;

use flate2::read::DeflateDecoder;

use crate::{buffer::bytebuffer_in as buffer, data::record_types::*};

use super::field::Field;

const FLAG_DEFLATE: u32 = 0x00040000;

type FieldReaderFN = fn(&mut buffer::ByteBufferIn) -> Vec<Field>;
lazy_static! {
    static ref TYPE_TO_FIELD_READER: HashMap<String, FieldReaderFN> = initialize_type_to_reader();
}

#[derive(Debug)]
pub struct RecordAndData {
    pub record: Record,
    pub data: buffer::ByteBufferIn,
}

#[derive(Debug)]
pub struct Record {
    pub type_: String,
    pub data_len: u32,
    pub flags: u32,
    pub id: u32,
    pub timestamp: Vec<u8>,
    pub vc1: u16,
    pub version: u16,
    pub vc2: u16,
    pub fields: Vec<Field>,
}

impl Record {
    pub fn read_common(buffer: &mut buffer::ByteBufferIn) -> Option<RecordAndData> {
        let record = Record {
            type_: buffer.read_string(4),
            data_len: buffer.read_dword(),
            flags: buffer.read_dword(),
            id: buffer.read_dword(),
            timestamp: buffer.read_bytes(2),
            vc1: buffer.read_word(),
            version: buffer.read_word(),
            vc2: buffer.read_word(),
            fields: Vec::new(),
        };

        let mut data = buffer::ByteBufferIn {
            data: buffer.data[buffer.offset..(buffer.offset + record.data_len as usize)].to_vec(),
            offset: 0,
        };

        buffer.offset += record.data_len as usize;

        // Handle compressed records.
        // TODO - converted from JavaScript, verify functionality (NPC_ records is a solid reference).
        if (record.flags & FLAG_DEFLATE) != 0 {
            let decompressed_len = data.read_dword();
            let mut decompressed = Vec::with_capacity(decompressed_len as usize);
            let mut decoder = DeflateDecoder::new(&data.data[data.offset..]);
            decoder.read_exact(&mut decompressed).unwrap();
            data.data = decompressed;
            data.offset = 0;
        }

        Some(RecordAndData { record, data })
    }

    pub fn read_default(buffer: &mut buffer::ByteBufferIn) -> Option<Self> {
        if let Some(RecordAndData {
            mut record, 
            mut data,
        }) = Self::read_common(buffer)
        {
            record.fields = match TYPE_TO_FIELD_READER.get(&record.type_) {
                Some(reader) => reader(&mut data),
                None => {
                    Vec::new()
                }
            };

            Some(record)
        } else {
            None
        }
    }
}

fn initialize_type_to_reader() -> HashMap<String, FieldReaderFN> {
    macro_rules! insert_mapping {
        ($map:expr, $($label:expr => $reader:expr),*) => {
            {
                let mut map = $map;
                $(map.insert($label.to_string(), $reader as FieldReaderFN);)*
                map
            }
        };
    }

    let label_to_record_reader = insert_mapping!(
        HashMap::new(),
        "TES4" => tes4::read_tes4,
        "GMST" => gmst::read_gmst,
        "VTYP" => vtyp::read_vtyp
    );

    label_to_record_reader
}