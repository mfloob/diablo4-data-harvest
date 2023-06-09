use std::{io::{self, Write}, fs::{File, self}, collections::HashMap};
use itertools::Itertools;
use serde::{Serialize, Deserialize};
use serde_json;

use super::{Parser};
use crate::utils;

#[derive(Serialize, Deserialize)]
pub struct Aff {
    pub files: HashMap<String, AffFile>
}

impl Aff {
    pub fn new() -> Self {
        Self {
            files: HashMap::new()
        }
    }

    fn new_file(&mut self, file_name: &str, hash_id: u32) {
        self.files.insert(file_name.to_owned(), AffFile::new(hash_id));
    }

    fn add_field(&mut self, file_name: &str, value: String) {
        self.files.entry(file_name.to_string())
            .and_modify(|k| { 
                k.values.push(value);
            });
    }
    
    fn header(f: &mut File) -> io::Result<(u32, u32, u32)> {
        let _deadbeef = utils::read_u32(f)?;
        let _file_type = utils::read_u32(f)?;
        utils::padding(f, 4)?;

        let hash_id = utils::read_u32(f)?;
        utils::padding(f, 128)?;

        let info_offset = utils::read_u32(f)? + 0x10u32;
        let info_len = utils::read_u32(f)?;
        utils::padding(f, 104)?;

        Ok((info_offset, info_len, hash_id))
    }

    fn info(f: &mut File) -> io::Result<String> {
        let key_offset = utils::read_u32(f)? + 0x10u32;
        let key_len = utils::read_u32(f)?;
        let buf = utils::read_offset(f, key_offset as u64, key_len as usize)?;
        let key_string = String::from_utf8(buf).unwrap();

        utils::padding(f, 8)?;

        let _something_offset = utils::read_u32(f)? + 0x10u32;
        let _something_len = utils::read_u32(f)?;

        Ok(key_string)
    }

    pub fn run(&mut self, path: String) -> io::Result<()> {
        let dir = fs::read_dir(path)?;

        for file in dir {
            let f_u = file?;
            let file_name = f_u.file_name().to_str().unwrap().to_owned();

            let mut f = File::open(f_u.path())?;
            let (info_offset, info_len, hash_id) = Aff::header(&mut f)?;
            self.new_file(file_name.as_str(), hash_id);

            utils::go_to(&mut f, info_offset.into())?; // move to info_offset

            let num_pairs = info_len/44;
            for i in 0..num_pairs {
                match i % 2 {
                    0 => utils::padding(&mut f, 24)?,
                    _ => utils::padding(&mut f, 16)?
                }
                let value = Aff::info(&mut f)?;
                self.add_field(&file_name, value.replace(char::from(0), ""));
            }
        }

        let json = serde_json::to_string_pretty(&self)?;
        let mut log = File::create("aff.json")?;
        log.write(json.as_bytes())?;

        Ok(())
    }
}

impl Parser for Aff {
    fn data_view(&self, ui: &mut egui::Ui, filter: &str) {
        let files = &self.files;
        egui::Grid::new("aff_grid")
            .show(ui, |ui| {                        
                for item in files.keys().filter(|x| filter.is_empty() || x.to_lowercase().contains(filter)).sorted() {
                    if files[item].values.len() > 0 {
                        ui.collapsing(item, |ui| {
                            ui.horizontal(|h| {
                                h.strong("hash_id:");
                                h.label(format!("{} ({:X})", files[item].hash_id, files[item].hash_id));
                            });
                            let values = &files[item];
                            for value in values.values.iter() {
                                ui.horizontal(|ui| {
                                    ui.label(value);
                                });
                            }
                        });
                    }
                    else {
                        ui.label(item);
                    }
                    ui.end_row();
                }
            });
    }

    fn load_data_file(&self) -> Option<Box<dyn Parser>> {
        match utils::load_or_pick_data_file("aff.json") {
            Some(buf) => {
                let data_str = String::from_utf8(buf).unwrap();
                let aff: Aff = serde_json::from_str(&data_str).unwrap();
                Some(Box::new(aff) as Box<dyn Parser>)
            },
            _ => None
        }
    }

    fn tab_title(&self) -> String {
        ".aff".to_owned()
    }
}

#[derive(Serialize, Deserialize)]
pub struct AffFile {
    pub hash_id: u32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub values: Vec<String>
}

impl AffFile {
    fn new(hash_id: u32) -> Self {
        Self {
            hash_id,
            values: Vec::new()
        }
    }
}