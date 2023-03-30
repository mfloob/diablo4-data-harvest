use std::{io::{self, Write}, fs::{File, self}, collections::HashMap};
use serde::{Serialize, Deserialize};
use serde_json;

use crate::utils;

#[derive(Serialize, Deserialize)]
pub struct Stl {
    pub files: HashMap<String, StlFile>
}

impl Stl {
    pub fn new() -> Self {
        Self {
            files: HashMap::new()
        }
    }

    fn new_file(&mut self, file_name: &str) {
        self.files.insert(file_name.to_owned(), StlFile::new());
    }

    fn add_field(&mut self, file_name: String, key: String, value: String) {
        self.files.entry(file_name)
            .and_modify(|k| { 
                k.fields.insert(key, value);
            });
    }
    
    /// returns length of the info block
    fn header(f: &mut File) -> io::Result<u32> {
        let _deadbeef = utils::read_u32(f)?;
        utils::padding(f, 12)?;

        let _hash_id = utils::read_u32(f)?;
        utils::padding(f, 16)?;

        let info_len = utils::read_u32(f)?;
        utils::padding(f, 8)?;
        
        Ok(info_len)
    }

    fn info(f: &mut File) -> io::Result<(String, String)> {
        utils::padding(f, 8)?;
        let key_offset = utils::read_u32(f)? + 16u32;
        let key_len = utils::read_u32(f)?;
        let buf = utils::read_offset(f, key_offset as u64, key_len as usize)?;
        let key_string = String::from_utf8(buf).unwrap();
        utils::padding(f, 8)?;
    
        let val_offset = utils::read_u32(f)? + 16u32;
        let val_len = utils::read_u32(f)?;
        let buf = utils::read_offset(f, val_offset as u64, val_len as usize)?;
        let val_string = String::from_utf8(buf).unwrap();
        utils::padding(f, 8)?;
    
        Ok((key_string, val_string))
    }

    pub fn run(&mut self, path: String) -> io::Result<()> {
        let dir = fs::read_dir(path)?;

        for file in dir {
            let f_u = file?;
            let file_name = f_u.file_name().to_str().unwrap().to_owned();

            self.new_file(file_name.as_str());
            
            let mut f = File::open(f_u.path())?;
            let info_len = Stl::header(&mut f)?;
            let num_pairs = info_len/40;
            for _ in 0..num_pairs {
                let (key, value) = Stl::info(&mut f)?;
                self.add_field(file_name.clone(), key.replace(char::from(0), ""), value.replace(char::from(0), ""));
            }
        }

        let json = serde_json::to_string_pretty(&self)?;
        let mut log = File::create("stl.json")?;
        log.write(json.as_bytes())?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct StlFile {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub fields: HashMap<String, String>
}

impl StlFile {
    fn new() -> Self {
        Self {
            fields: HashMap::new()
        }
    }
}