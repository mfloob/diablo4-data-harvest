use std::{io::{self, Write}, fs::{File, self}, collections::HashMap};
use serde::Serialize;
use serde_json;

use crate::utils;

#[derive(Serialize)]
pub struct Stl {
    files: HashMap<String, StlFile>
}

impl Stl {
    fn new() -> Self {
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
        let key_offset = utils::read_u32(f)?;
        let key_len = utils::read_u32(f)?;
        let buf = utils::read_offset(f, (key_offset + 16) as u64, key_len as usize)?;
        let key_string = String::from_utf8(buf).unwrap();
        utils::padding(f, 8)?;
    
        let val_offset = utils::read_u32(f)?;
        let val_len = utils::read_u32(f)?;
        let buf = utils::read_offset(f, (val_offset + 16) as u64, val_len as usize)?;
        let val_string = String::from_utf8(buf).unwrap();
        utils::padding(f, 8)?;
    
        Ok((key_string, val_string))
    }

    pub fn run(path: String) -> io::Result<()> {
        let dir = fs::read_dir(path)?;

        let mut stl = Stl::new();
        
        for file in dir {
            let f_u = file?;
            let file_name = f_u.file_name().to_str().unwrap().to_owned();

            stl.new_file(file_name.as_str());
            
            let mut f = File::open(f_u.path())?;
            let info_len = Stl::header(&mut f)?;
            let num_pairs = info_len/40;
            for _ in 0..num_pairs {
                let (key, value) = Stl::info(&mut f)?;
                stl.add_field(file_name.clone(), key.replace(char::from(0), ""), value.replace(char::from(0), ""));
            }
        }

        println!("Parsing finished, pretty printing json");
        let json = serde_json::to_string_pretty(&stl)?;
        println!("Complete");
        let mut log = File::create("string_list.json")?;
        log.write(json.as_bytes())?;

        Ok(())
    }
}

#[derive(Serialize)]
struct StlFile {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    fields: HashMap<String, String>
}

impl StlFile {
    fn new() -> Self {
        Self {
            fields: HashMap::new()
        }
    }
}