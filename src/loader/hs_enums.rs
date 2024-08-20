use byteorder::{ReadBytesExt, BE};
use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Debug, Clone)]
pub struct HSEnum {
    pub value: u32,
    pub length: u32,
    pub name: String,
}

impl HSEnum {
    pub fn read(reader: &mut BufReader<File>) -> Result<Vec<Self>, std::io::Error> {
        let count = reader.read_i32::<BE>()?;
        (0..count)
            .map(|_| {
                let value = reader.read_u32::<BE>()?;
                let length = reader.read_u32::<BE>()?;

                let mut name_buffer = vec![0; length as usize];
                reader.read_exact(&mut name_buffer)?;

                Ok(HSEnum {
                    value,
                    length,
                    name: String::from_utf8_lossy(&name_buffer)
                        .trim_end_matches('\0')
                        .to_string(),
                })
            })
            .collect()
    }
}
