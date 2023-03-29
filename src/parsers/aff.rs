use std::{io::{self, Write}, fs::{File, self}, collections::HashMap};
use serde::Serialize;
use serde_json;

use crate::utils;

#[derive(Serialize)]
pub struct Aff {
    files: HashMap<String, AffFile>
}

impl Aff {
    fn new() -> Self {
        Self {
            files: HashMap::new()
        }
    }

    fn new_file(&mut self, file_name: &str) {
        self.files.insert(file_name.to_owned(), AffFile::new());
    }

    fn add_field(&mut self, file_name: String, value: String) {
        self.files.entry(file_name)
            .and_modify(|k| { 
                k.values.push(value);
            });
    }
    
    fn header(f: &mut File) -> io::Result<(u32, u32)> {
        let _deadbeef = utils::read_u32(f)?;
        let _file_type = utils::read_u32(f)?;
        utils::padding(f, 8)?;

        let _hash_id = utils::read_u32(f)?;
        utils::padding(f, 124)?;

        let info_offset = utils::read_u32(f)? + 16u32;
        let info_len = utils::read_u32(f)?;
        utils::padding(f, 104)?;

        Ok((info_offset, info_len))
    }

    fn info(f: &mut File) -> io::Result<String> {
        let key_offset = utils::read_u32(f)? + 16u32;
        let key_len = utils::read_u32(f)?;
        let buf = utils::read_offset(f, key_offset as u64, key_len as usize)?;
        let key_string = String::from_utf8(buf).unwrap();

        utils::padding(f, 8)?;

        let _something_offset = utils::read_u32(f)? + 16u32;
        let _something_len = utils::read_u32(f)?;

        Ok(key_string)
    }

    pub fn run(path: String) -> io::Result<()> {
        let dir = fs::read_dir(path)?;

        let mut aff = Aff::new();

        for file in dir {
            let f_u = file?;
            let file_name = f_u.file_name().to_str().unwrap().to_owned();

            aff.new_file(file_name.as_str());

            let mut f = File::open(f_u.path())?;
            let (info_offset, info_len) = Aff::header(&mut f)?;
            utils::go_to(&mut f, info_offset.into())?; // move to info_offset

            let num_pairs = info_len/44;
            for i in 0..num_pairs {
                match i % 2 {
                    0 => utils::padding(&mut f, 24)?,
                    _ => utils::padding(&mut f, 16)?
                }
                let value = Aff::info(&mut f)?;
                aff.add_field(file_name.clone(), value.replace(char::from(0), ""));
            }
        }

        let json = serde_json::to_string_pretty(&aff)?;
        let mut log = File::create("aff_list.json")?;
        log.write(json.as_bytes())?;

        Ok(())
    }
}

#[derive(Serialize)]
struct AffFile {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    values: Vec<String>
}

impl AffFile {
    fn new() -> Self {
        Self {
            values: Vec::new()
        }
    }
}