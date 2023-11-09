
use crate::buffer::bytebuffer_in::{self as buffer};

#[derive(Debug)]
pub enum FieldData {
    StringData(String),
    BinaryData(Vec<u8>),
    BoolData(bool),
    FieldsData(Vec<Field>),
    FloatData(f32),
    ByteData(u8),
    IntData16(u16),
    IntData32(u32),
    IntData64(u64),
    FormIDData(String),
    FormIDArray(Vec<String>),
    FlagsData(Vec<bool>),
    RGBAData(Vec<u8>)
}

#[derive(Debug)]
pub struct Field {
    pub type_: String,
    pub data_len: u16,
    pub data: Option<FieldData>,
}

impl Field {
    pub fn read_common(buffer: &mut buffer::ByteBufferIn) -> Self {
        Field {
            type_: buffer.read_string(4),
            data_len: buffer.read_u16(),
            data: None,
        }
    }

    pub fn read_default(buffer: &mut buffer::ByteBufferIn) -> Self {
        let field = Self::read_common(buffer);

        match field.type_.as_str() {
            "EDID" => field.read_z_string_field(buffer),
            _ => field.read_binary_field(buffer),
        }
    }

    pub fn create_generic_field(
        field_type: &str,
        main_data_len: u16,
        data: Vec<(String, u16, FieldData)>,
    ) -> Field {
        Field {
            type_: field_type.to_string(),
            data_len: main_data_len,
            data: Some(FieldData::FieldsData(
                data.into_iter()
                    .map(|(type_, data_len, data)| Field {
                        type_,
                        data_len,
                        data: Some(data),
                    })
                    .collect(),
            )),
        }
    }

    pub fn read_rgba_field(mut self, buffer: &mut buffer::ByteBufferIn) -> Self {
        let mut rgba: Vec<u8> = vec![0; 4];
        rgba[0] = buffer.read_u8(); // Red
        rgba[1] = buffer.read_u8(); // Green
        rgba[2] = buffer.read_u8(); // Blue
        rgba[3] = buffer.read_u8(); // Alpha

        self.data = Some(FieldData::RGBAData(rgba));
        self
    }

    pub fn read_z_string_field(mut self, buffer: &mut buffer::ByteBufferIn) -> Self {
        self.data = Some(FieldData::StringData(
            buffer.read_string(self.data_len as isize).trim_end_matches('\0').to_owned(),
        ));
        self
    }

    pub fn read_binary_field(mut self, buffer: &mut buffer::ByteBufferIn) -> Self {
        self.data = Some(FieldData::BinaryData(
            buffer.read_u8_vec(self.data_len as usize),
        ));
        self
    }

    pub fn read_float_field(mut self, buffer: &mut buffer::ByteBufferIn) -> Self {
        self.data = Some(FieldData::FloatData(buffer.read_float()));
        self
    }

    pub fn read_u32_field(mut self, buffer: &mut buffer::ByteBufferIn) -> Self {
        self.data = Some(FieldData::IntData32(buffer.read_u32()));
        self
    }

    pub fn read_u64_field(mut self, buffer: &mut buffer::ByteBufferIn) -> Self {
        self.data = Some(FieldData::IntData64(buffer.read_u64()));
        self
    }

    pub fn read_bool_field(mut self, buffer: &mut buffer::ByteBufferIn) -> Self {
        let bool_u32 = buffer.read_u32();
        self.data = Some(FieldData::BoolData(bool_u32 != 0));
        self
    }
}
