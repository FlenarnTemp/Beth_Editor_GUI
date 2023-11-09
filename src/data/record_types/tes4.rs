
use crate::{buffer::bytebuffer_in as buffer, data::core::field::{Field, FieldData}};

// Reference:: https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/TES4
pub fn read_tes4(buffer: &mut buffer::ByteBufferIn) -> Vec<Field> {
    let mut temp_fields = Vec::new();

    while buffer.available() > 0 {
        let mut field = Field::read_common(buffer);

        match field.type_.as_str() {
            "HEDR" => {
                let data_fields = vec![
                    ("Version".to_string(), 4, FieldData::FloatData(buffer.read_float())),
                    ("Number of Records".to_string(), 4, FieldData::IntData32(buffer.read_u32())),
                    ("Next Object ID".to_string(), 4, FieldData::FormIDData(format!("{:x}", buffer.read_u32()).to_uppercase()))
                ];
                let data_field = Field::create_generic_field("HEDR", field.data_len, data_fields);
                temp_fields.push(data_field);
            }
    
            "CNAM" | "MAST" | "SNAM" => {
                temp_fields.push(field.read_z_string_field(buffer));
            }

            "INTV" | "INCC" => {
                let temp_field = field.read_u32_field(buffer);
                temp_fields.push(temp_field);
            }

            "DATA" => {
                let temp_field = field.read_u64_field(buffer);
                temp_fields.push(temp_field);
            }

            "ONAM" => {
                let mut data_fields: Vec<String> = Vec::new();

                while buffer.available() > 0 {
                    data_fields.push(format!("{:x}", buffer.read_u32()).to_uppercase());
                }
                field.data = Some(FieldData::FormIDArray(data_fields));
                temp_fields.push(field);
            }

            _ => {
                println!("Missing type: {} in TES4 parsing, size: {:?}.", field.type_, field.data_len);
                temp_fields.push(field.read_binary_field(buffer));
            }
        }
    }
    temp_fields
}