use crate::data::core::field::{Field, FieldData};
use crate::buffer::bytebuffer_in as buffer;

// Reference:: https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/IPDS
pub fn read_ipds(buffer: &mut buffer::ByteBufferIn) -> Vec<Field> {
    let mut temp_fields = Vec::new();

    while buffer.available() > 0 {
        let mut field = Field::read_common(buffer);

        match field.type_.as_str() {
            "EDID" => {
                temp_fields.push(field.read_z_string_field(buffer));
            }

            "PNAM" => {
                field.data = Some(FieldData::FormIDArray(vec![
                    format!("{:x}", buffer.read_u32()).to_uppercase(),
                    format!("{:x}", buffer.read_u32()).to_uppercase()
                ]));
                temp_fields.push(field);
            }
           
            _ => {
                println!("Missing type: {} in IPDS parsing, size: {:?}.", field.type_, field.data_len);
                temp_fields.push(field.read_binary_field(buffer));
            }
        }
    }
    temp_fields
}