use std::{io::Read, collections::HashMap};
use lazy_static::lazy_static;

use flate2::read::DeflateDecoder;

use crate::{buffer::bytebuffer_in as buffer, data::record_types::*};

use super::{field::Field, group::Group};

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
    pub id: String,
    pub timestamp: Vec<u8>,
    pub vc1: u16,
    pub version: u16,
    pub vc2: u16,
    pub fields: Vec<Field>,
    pub subgroups: Vec<Group>
}

impl Record {
    pub fn read_common(buffer: &mut buffer::ByteBufferIn, record_type: String) -> Option<RecordAndData> {
        let record = Record {
            type_: record_type,
            data_len: buffer.read_dword(),
            flags: buffer.read_dword(),
            id: format!("{:x}", buffer.read_dword()).to_uppercase(),
            timestamp: buffer.read_bytes(2),
            vc1: buffer.read_word(),
            version: buffer.read_word(),
            vc2: buffer.read_word(),
            fields: Vec::new(),
            subgroups: Vec::new(),
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

    pub fn read_default(buffer: &mut buffer::ByteBufferIn, record_type: String) -> Option<Self> {
        if let Some(RecordAndData {
            mut record, 
            mut data,
        }) = Self::read_common(buffer, record_type.clone())
        {
            if record_type == "GRUP" {
                record.subgroups.push(Group::read(buffer, record_type).unwrap())  
            } else {
                record.fields = match TYPE_TO_FIELD_READER.get(&record_type) {
                    Some(reader) => reader(&mut data),
                    None => {
                        default_reader(&mut data)
                    }
                };
            }
            
            Some(record)
        } else {
            None
        }
    }
}

/*
 * Utilized when no proper reader is defined to a given record type.
*/
fn default_reader(buffer: &mut buffer::ByteBufferIn) -> Vec<Field> {
    let mut temp_fields = Vec::new();

    while buffer.available() > 0 {
        let field = Field::read_common(buffer).unwrap();

        match field.type_.as_str() {
            "EDID" => {
                let temp_field = field.read_z_string_field(buffer);
                temp_fields.push(temp_field)
            }

            _ => {
                temp_fields.push(field.read_binary_field(buffer).unwrap())
            }
        }
    }

    temp_fields
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