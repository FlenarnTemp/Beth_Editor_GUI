use crate::data::core::field::{Field, FieldData};
use crate::buffer::bytebuffer_in as buffer;

// Reference:: https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/CLFM
pub fn read_clfm(buffer: &mut buffer::ByteBufferIn) -> Vec<Field> {
    let mut temp_fields = Vec::new();

    while buffer.available() > 0 {
        let mut field = Field::read_common(buffer);

        match field.type_.as_str() {
            "EDID" => {
                temp_fields.push(field.read_z_string_field(buffer));
            }

           "CNAM" => {
                temp_fields.push(field.read_rgba_field(buffer));
           }

           "FNAM" => {
                let temp_data = buffer.read_u32();

                let mut flags: Vec<bool> = vec![false; 3];

                let playable = 0x01;
                let remapping_index = 0x02;
                let extended_lut = 0x03;

                flags[0] = (temp_data & playable) != 0;
                flags[1] = (temp_data & remapping_index) != 0;
                flags[2] = (temp_data & extended_lut) != 0;

                field.data = Some(FieldData::FlagsData(flags));
                temp_fields.push(field);
           }

            _ => {
                println!("Missing type: {} in CLFM parsing, size: {:?}.", field.type_, field.data_len);
                temp_fields.push(field.read_binary_field(buffer));
            }
        }
    }
    temp_fields
}