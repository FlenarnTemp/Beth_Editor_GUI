use crate::buffer::bytebuffer_in as buffer;

use super::{record, group::Group};

pub struct Plugin {
    pub header: record::Record,
    pub groups: Vec<Group>
}

impl Plugin {
    pub fn read(buffer: &mut buffer::ByteBuffer_In) -> Option<Self> {
        let mut plugin = Plugin {
            header: record::Record::read_COMMON(buffer)?,
            groups: Vec::new(),
        };

        //TODO - Group processing.

        Some(plugin)
    }
}