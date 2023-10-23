use crate::data::core::field::{Field, FieldData};
use crate::buffer::bytebuffer_in as buffer;

// Reference:: https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/VTYP
pub fn read_vtyp(buffer: &mut buffer::ByteBufferIn) -> Vec<Field> {
    let mut temp_fields = Vec::new();

    while buffer.available() > 0 {
        let mut field = Field::read_common(buffer);

        match field.type_.as_str() {
            "EDID" => {
                temp_fields.push(field.read_z_string_field(buffer));
            }

            "DNAM" => {
                let temp_data = buffer.read_u8();

                let mut flags: Vec<bool> = vec![false; 2];

                let allow_default = 0x01;
                let female = 0x02;

                flags[0] = (temp_data & allow_default) != 0;
                flags[1] = (temp_data & female) != 0;

                field.data = Some(FieldData::FlagsData(flags));
                temp_fields.push(field);
            }
           
            _ => {
                println!("Missing type: {} in VTYP parsing.", field.type_);
                temp_fields.push(field.read_binary_field(buffer));
            }
        }
    }
    temp_fields
}