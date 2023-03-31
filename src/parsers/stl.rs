use std::{io::{self, Write}, fs::{File, self}, collections::HashMap};
use itertools::Itertools;
use serde::{Serialize, Deserialize};
use serde_json;

use super::{Parser};
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

    fn new_file(&mut self, file_name: &str, hash_id: u32) {
        self.files.insert(file_name.to_owned(), StlFile::new(hash_id));
    }

    fn add_field(&mut self, file_name: &String, key: String, value: String) {
        self.files.entry(file_name.to_string())
            .and_modify(|k| { 
                k.fields.insert(key, value);
            });
    }
    
    /// returns length of the info block and hash_id
    fn header(f: &mut File) -> io::Result<(u32, u32)> {
        let _deadbeef = utils::read_u32(f)?;
        utils::padding(f, 8)?;

        let hash_id = utils::read_u32(f)?;
        utils::padding(f, 20)?;

        let info_len = utils::read_u32(f)?;
        utils::padding(f, 8)?;
        
        Ok((info_len, hash_id))
    }

    fn info(f: &mut File) -> io::Result<(String, String)> {
        utils::padding(f, 8)?;
        let key_offset = utils::read_u32(f)? + 0x10u32;
        let key_len = utils::read_u32(f)?;
        let buf = utils::read_offset(f, key_offset as u64, key_len as usize)?;
        let key_string = String::from_utf8(buf).unwrap();
        utils::padding(f, 8)?;
    
        let val_offset = utils::read_u32(f)? + 0x10u32;
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

            let mut f = File::open(f_u.path())?;
            let (info_len, hash_id) = Stl::header(&mut f)?;
            self.new_file(file_name.as_str(), hash_id);
            
            let num_pairs = info_len/40;
            for _ in 0..num_pairs {
                let (key, value) = Stl::info(&mut f)?;
                self.add_field(&file_name, key.replace(char::from(0), ""), value.replace(char::from(0), ""));
            }
        }

        let json = serde_json::to_string_pretty(&self)?;
        let mut log = File::create("stl.json")?;
        log.write(json.as_bytes())?;

        Ok(())
    }
}

impl Parser for Stl {
    fn data_view(&self, ui: &mut egui::Ui, filter: &str) {
        let files = &self.files;
        let keys: Vec<_> =  files.keys().filter(|x| filter.is_empty() || x.to_lowercase().contains(filter)).sorted().collect();
        let scroll = egui::ScrollArea::new([true, true]);
        scroll.show_rows(ui, 
            10f32, 
            keys.len(), 
            |ui, row_range| {
                egui::Grid::new("stl_grid")
                    .show(ui, |ui| {     
                        let range = match keys.len() < row_range.end {
                            true => 0..keys.len(),
                            false => row_range
                        };
                        for item in &keys[range] {
                            if files[*item].fields.len() > 0 {
                                ui.collapsing(*item, |ui| {
                                    ui.horizontal(|h| {
                                        h.strong("hash_id:");
                                        h.label(format!("{} ({:X})", files[*item].hash_id, files[*item].hash_id));
                                    });
                                    let values = &files[*item];
                                    for value in values.fields.keys().sorted() {
                                        ui.horizontal(|ui| {
                                            ui.strong(format!("{}:", value));
                                            ui.label(&values.fields[value]);
                                        });
                                    }
                                });
                            }
                            else {
                                ui.collapsing(*item, |ui| {
                                    ui.horizontal(|h| {
                                        h.strong("hash_id:");
                                        h.label(format!("{}", files[*item].hash_id));
                                    });
                                });
                            }
                            ui.end_row();                            
                        }
                    });
                ui.allocate_space(egui::vec2(ui.available_width(), 1f32));
        });
    }

    fn load_data_file(&self) -> Option<Box<dyn Parser>> {
        match utils::load_or_pick_data_file("stl.json") {
            Some(buf) => {
                let data_str = String::from_utf8(buf).unwrap();
                let stl: Stl = serde_json::from_str(&data_str).unwrap();
                Some(Box::new(stl) as Box<dyn Parser>)
            },
            _ => None
        }
    }

    fn tab_title(&self) -> String {
        ".stl".to_owned()
    }
}

#[derive(Serialize, Deserialize)]
pub struct StlFile {
    pub hash_id: u32,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub fields: HashMap<String, String>
}

impl StlFile {
    fn new(hash_id: u32) -> Self {
        Self {
            hash_id,
            fields: HashMap::new()
        }
    }
}