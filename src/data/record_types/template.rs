use crate::data::core::field::Field;
use crate::buffer::bytebuffer_in as buffer;

// Reference:: https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/XXXX
pub fn read_xxxx(buffer: &mut buffer::ByteBuffer_In) -> Vec<Field> {
    let mut temp_fields = Vec::new();

    while buffer.available() > 0 {
        let field = Field::read_common(buffer);

        match field.type_.as_str() {
            "EDID" => {
                temp_fields.push(field.read_z_string_field(buffer));
            }
           
            _ => {
                println!("Missing type: {} in XXXX parsing.", field.type_);
                temp_fields.push(field.read_binary_field(buffer));
            }
        }
    }
    temp_fields
}