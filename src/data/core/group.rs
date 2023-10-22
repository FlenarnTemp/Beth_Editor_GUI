use super::record;

use crate::buffer::bytebuffer_in as buffer;

#[derive(Debug)]
pub struct Group {
    pub type_: String,
    pub data_len: u32,
    pub label: LabelType,
    pub group_type: u32,
    pub timestamp: u16,
    pub vc: u16,
    pub unk1: u32,
    pub subgroups: Vec<Group>,
    pub records: Vec<record::Record>,
}

#[derive(Debug)]
pub enum LabelType {
    StringValue(String),
    Coordinates { y: u16, x: u16 },
    BinaryData(Vec<u8>),
    ByteData(u8),
}

impl Group {
    pub fn read(buffer: &mut buffer::ByteBufferIn, mut class_type: String) -> Option<Self> {
        let mut group = Group {
            type_: class_type,
            data_len: buffer.read_dword() - 24, // 'data_len' includes header length.
            label: LabelType::BinaryData(buffer.read_bytes(4)),
            group_type: buffer.read_dword(),
            timestamp: buffer.read_word(),
            vc: buffer.read_word(),
            unk1: buffer.read_dword(),
            subgroups: Vec::new(),
            records: Vec::new(),
        };


        if let LabelType::BinaryData(binary_data) = &group.label {
            match group.group_type {
                0 => { /* Top (Type) */
                    let decoded_string = String::from_utf8_lossy(binary_data).to_string();
                    group.label = LabelType::StringValue(decoded_string);
                }
    
                2 /* Interior Block Number */ | 3 /* Interior Sub-block Number */ => {
                    group.label = LabelType::ByteData(binary_data[0]);
                }
    
                4 /* Exterior Cell Block */ | 5 /* Exterior Cell Sub-block */ => {
                    let mut temp_buffer = buffer::ByteBufferIn::new(binary_data.to_vec());
                    group.label = LabelType::Coordinates { y: temp_buffer.read_word(), x: temp_buffer.read_word() }
                }
    
                1 /* World Children (Parent - WRLD) */              |
                6 /* Cell Children (Parent - CELL) */               |
                7 /* Topic Children (Parent - DIAL) */              |
                8 /* Cell Persistent Children (Parent - Cell) */    |
                9 /* Cell Temporary Children (Parent - Cell) */     =>
                {
                    let mut temp_buffer = buffer::ByteBufferIn::new(binary_data.to_vec());
                    group.label = LabelType::StringValue(format!("{:x}", temp_buffer.read_dword()))
                }
                _ => {
                    // TODO: We should never end up here.
                }
            }
        }

        //println!("Processing: {:?}, {:?}, {:?}", group.group_type, group.type_, group.label);

        let mut data = buffer::ByteBufferIn {
            data: buffer.data[buffer.offset..(buffer.offset + group.data_len as usize)].to_vec(),
            offset: 0,
        };
        buffer.offset += group.data_len as usize;

        if let LabelType::StringValue(string_data) = &group.label {
            if string_data == "CELL" || string_data == "QUST" || string_data == "WRLD" {
                // FIXME: Disabled for now.
            } else {
                while data.available() > 0 {
                    class_type = data.read_string(4);

                    if let Some(record) = record::Record::read_default(&mut data, class_type) {
                        group.records.push(record);
                    }
                }
            }
        } else {
            while data.available() > 0 {
                class_type = data.read_string(4);

                if let Some(record) = record::Record::read_default(&mut data, class_type) {
                    group.records.push(record);
                }
            }
        }

        Some(group)
    }
}