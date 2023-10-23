use crate::{buffer::bytebuffer_in as buffer, data::core::field::{Field, FieldData}};

// Reference:: https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/GMST
pub fn read_gmst(buffer: &mut buffer::ByteBufferIn) -> Vec<Field> {
    let mut temp_fields = Vec::new();

    while buffer.available() > 0 {
        let field = Field::read_common(buffer);

        match field.type_.as_str() {
            "EDID" => {
                temp_fields.push(field.read_z_string_field(buffer));
            }
            
            "DATA" => {
                if let FieldData::StringData(string_data) = &temp_fields[0].data.as_mut().unwrap() {
                    let c: char = string_data.chars().next().unwrap();

                    // TODO: Missing several types, not used in Fallout4.esm, but checked for in code:
                    // c(char), h(char), r(RGB), a(RGBA)
                    // FIXME: 's' & 'S' currently reads z_strings, should read l_strings.
                    if c == 's' || c == 'S' { 
                        temp_fields.push(field.read_z_string_field(buffer))
                    } else if c == 'b' {
                        temp_fields.push(field.read_bool_field(buffer))
                    } else if c == 'i' || c == 'u' {
                        temp_fields.push(field.read_u32_field(buffer))
                    } else if c == 'f' {
                        temp_fields.push(field.read_float_field(buffer));
                    } else {
                        let temp_field = field.read_binary_field(buffer);
                        println!("Missing GMST typep value: {}, data: {:?}", c, temp_field.data);
                        temp_fields.push(temp_field);
                    }
                }
            }
            _ => {
                println!("Missing type: {} in GMST parsing.", field.type_);
                temp_fields.push(field.read_binary_field(buffer))
            }
        }
    }
    temp_fields
}