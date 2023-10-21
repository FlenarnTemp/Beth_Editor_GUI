use crate::buffer::bytebuffer_in as buffer;

use super::{record::{self, Record}, group::Group};

#[derive(Debug)]
pub struct Plugin {
    pub header: record::Record,
    pub groups: Vec<Group>
}

impl Plugin {
    pub fn read(buffer: &mut buffer::ByteBufferIn) -> Option<Self> {
        let mut plugin = Plugin {
            header: Record::read_default(buffer)?,
            groups: Vec::new(),
        };

        //FIXME - Group processing is very lazily implemented and needs a lot of work.
        while buffer.available() > 0 {
            if let Some(group) = Group::read(buffer) {
                plugin.groups.push(group);
            } else {
                break;
            }
        }

        Some(plugin)
    }
}