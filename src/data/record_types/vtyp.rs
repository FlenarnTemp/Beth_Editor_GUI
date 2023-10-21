use crate::data::core::field::Field;
use crate::buffer::bytebuffer_in as buffer;

// Reference:: https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/VTYP
pub fn read_vtyp(buffer: &mut buffer::ByteBufferIn) -> Vec<Field> {
    let mut temp_fields = Vec::new();

    while buffer.available() > 0 {
        let field = Field::read_common(buffer).unwrap();

        match field.type_.as_str() {
            "EDID" => {
                let temp_field = field.read_z_string_field(buffer);
                temp_fields.push(temp_field)
            }

            "DNAM" => {
                let temp_field = field.read_binary_field(buffer).unwrap();
                temp_fields.push(temp_field)
            }
           
            _ => {
                println!("Missing type: {} in VTYP parsing.", field.type_);
                temp_fields.push(field.read_binary_field(buffer).unwrap())
            }
        }
    }
    temp_fields
}