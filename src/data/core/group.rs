use super::record;

use crate::buffer::bytebuffer_in as buffer;

pub struct Group {
    pub type_: String,
    pub data_len: u32,
    pub label: String,
    pub group_type: u32,
    pub timestamp: u16,
    pub vc: u16,
    pub unk1: u32,
    pub subgroups: Vec<Group>,
    pub records: Vec<record::Record>,
}

impl Group {
    pub fn read(buffer: &mut buffer::ByteBuffer_In) -> Option<Self> {
        let mut group = Group {
            type_: buffer.read_string(4)?,
            data_len: buffer.read_dword()? - 24,
            label: buffer.read_string(4)?,
            group_type: buffer.read_dword()?,
            timestamp: buffer.read_word()?,
            vc: buffer.read_word()?,
            unk1: buffer.read_dword()?,
            subgroups: Vec::new(),
            records: Vec::new(),
        };

        let mut data = buffer::ByteBuffer_In {
            data: buffer.data[buffer.offset..(buffer.offset + group.data_len as usize)].to_vec(),
            offset: 0,
        };
        buffer.offset += group.data_len as usize;

        if (group.group_type == 2 || group.group_type == 3 || group.label == "CELL"|| group.label == "QUST" || group.label == "WRLD") {
            // FIXME: Disabled for now. Reference JS implementation for full logic.
        } else {
            while data.available() > 0 {
                /*if let Some(record) = record::Record::read_DEFAULT(&mut data) {
                    group.records.push(record);
                }*/
            }
        }

        Some(group)
    }
}