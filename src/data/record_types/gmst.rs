use crate::{buffer::bytebuffer_in as buffer, data::core::field::Field};

// Reference:: https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/GMST
pub fn read_gmst(buffer: &mut buffer::ByteBufferIn) -> Vec<Field> {
    let mut temp_fields = Vec::new();

    while buffer.available() > 0 {
        let field = Field::read_common(buffer).unwrap();

        match field.type_.as_str() {
            "EDID" => {
                let temp_field = field.read_z_string_field(buffer);
                temp_fields.push(temp_field)
            }

            //FIXME: z_strings? l_strings?
            "DATA" => {
                temp_fields.push(field.read_binary_field(buffer).unwrap())

                /*if let FieldData::StringData(string_data) = &temp_fields[0].data.as_mut().unwrap() {
                    let c: char = string_data.chars().next().unwrap();
                    if c == 's' {
                        let temp_field = field.read_z_string_field(buffer).unwrap();
                        println!("GMST: {}, data type: {}, data: {:?}", string_data, c, temp_field.data);
                        temp_fields.push(temp_field)
                        
                    } else if c == 'i' {
                        let temp_field = field.read_int32_field(buffer).unwrap();
                        println!("GMST: {}, data type: {}, data: {:?}", string_data, c, temp_field.data);
                        temp_fields.push(temp_field)
                    } else if c == 'f' {
                        temp_fields.push(field.read_binary_field(buffer).unwrap())
                    }
                }*/
            }
            _ => {
                println!("Missing type: {} in GMST parsing.", field.type_);
                temp_fields.push(field.read_binary_field(buffer).unwrap())
            }
        }
    }
    temp_fields
}