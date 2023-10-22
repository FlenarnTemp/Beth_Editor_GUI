use std::hash::BuildHasher;

use crate::{buffer::bytebuffer_in as buffer, data::core::field::{Field, FieldData}};

// Reference:: https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/TES4
pub fn read_tes4(buffer: &mut buffer::ByteBufferIn) -> Vec<Field> {
    let mut temp_fields = Vec::new();

    while buffer.available() > 0 {
        let field = Field::read_common(buffer).unwrap();

        match field.type_.as_str() {
            "HEDR" => {
                let data_fields = vec![
                    ("Version".to_string(), 4, FieldData::FloatData(buffer.read_float())),
                    ("Number of Records".to_string(), 4, FieldData::IntData32(buffer.read_dword())),
                    ("Next Object ID".to_string(), 4, FieldData::FormIDData(format!("{:x}", buffer.read_dword()).to_uppercase()))
                ];
                let data_field = Field::create_generic_field("HEDR", field.data_len, data_fields);
                temp_fields.push(data_field)
            }
    
            "CNAM" | "MAST" | "SNAM" => {
                let temp_field = field.read_z_string_field(buffer);
                temp_fields.push(temp_field)
            }

            "INTV" | "INCC" => {
                let temp_field = field.read_int32_field(buffer).unwrap();
                temp_fields.push(temp_field)
            }

            "DATA" => {
                let temp_field = field.read_int64_field(buffer).unwrap();
                temp_fields.push(temp_field)
            }

            "ONAM" => {
                let mut data_fields: Vec<FieldData> = Vec::new();

                while buffer.available() > 0 {
                    data_fields.push(FieldData::FormIDData(format!("{:x}", buffer.read_dword()).to_uppercase()));
                }
                let data_field = Field::create_generic_field("ONAM", field.data_len, data_fields);
                temp_fields.push(data_field)
            }

            _ => {
                println!("Missing type: {} in TES4 parsing.", field.type_);
                temp_fields.push(field.read_binary_field(buffer).unwrap())
            }
        }
    }
    temp_fields
}