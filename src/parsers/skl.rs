use std::{io::{self, Write, Seek}, fs::{File, self}, collections::HashMap};
use egui::CollapsingHeader;
use itertools::Itertools;
use serde::{Serialize, Deserialize};
use serde_json;

use super::{Parser};
use crate::utils;

#[derive(Serialize, Deserialize)]
pub struct Skl {
    pub files: HashMap<String, SklFile>
}

impl Skl {
    pub fn new() -> Self {
        Self {
            files: HashMap::new()
        }
    }

    fn add_field(&mut self, file_name: &str, value: SklField) {
        let file = self.files.entry(file_name.to_string())
            .or_insert(SklFile::new());

        file.skills.push(value);
    }

    fn header(f: &mut File) -> io::Result<(u32, u32)> {
        let _deadbeef = utils::read_u32(f)?;
        let _file_type = utils::read_u32(f)?;
        utils::padding(f, 4)?;
        let _hash_id = utils::read_u32(f)?;
        utils::padding(f, 16)?; // end first line
        utils::padding(f, 32)?; // end second line

        let skill_tree_offset = utils::read_u32(f)? + 0x10u32;
        let skill_tree_len = utils::read_u32(f)?;
        utils::padding(f, 8)?;

        let _something_offset = utils::read_u32(f)? + 0x10u32;
        let _something_len = utils::read_u32(f)?;
        let _idk = utils::read_u32(f)?;
        
        Ok((skill_tree_offset, skill_tree_len))
    }

    fn field(f: &mut File) -> io::Result<SklField> {
        let id = utils::read_u32(f)?;
        let reward_hash = utils::read_u32(f)?;
        utils::padding(f, 4)?;
        let x = utils::read_f32(f)?;
        let y = utils::read_f32(f)?;
        let is_root = utils::read_u32(f)? == 1u32;
        let req_points = utils::read_u32(f)?;
        utils::padding(f, 12)?;
        let connection_offset = utils::read_u32(f)? + 0x10u32;
        let connection_len = utils::read_u32(f)?;
        let connections = Skl::get_connections(f, connection_offset, connection_len)?;
        utils::padding(f, 16)?;
        
        let skl_field = SklField::new(id, req_points, reward_hash, is_root, x, y, connections);

        Ok(skl_field)
    }

    fn get_connections(f: &mut File, offset: u32, len: u32) -> io::Result<Vec<u32>> {
        let mut connections = Vec::new();
        let old_pos = f.stream_position()?;
        utils::go_to(f, offset.into())?;

        let num_connections = len/4;
        for _ in 0..num_connections {
            let connection = utils::read_u32(f)?;
            connections.push(connection);
        };

        utils::go_to(f, old_pos.into())?;

        Ok(connections)
    }

    pub fn run(&mut self, path: String) -> io::Result<()> {
        let dir = fs::read_dir(path)?;

        for file in dir {
            let f_u = file?;
            let name = f_u.file_name();
            let file_name = name.to_str().unwrap();

            let mut f = File::open(&f_u.path())?;
            let (info_offset, info_len) = Skl::header(&mut f)?;
            utils::go_to(&mut f, info_offset.into())?; // move to info offset
            let num_fields = info_len/64;

            for _ in 0..num_fields {
                let skl_field = Skl::field(&mut f)?;
                self.add_field(file_name, skl_field);
            }
        }

        let json = serde_json::to_string_pretty(&self)?;
        let mut log = File::create("skl.json")?;
        log.write(json.as_bytes())?;

        Ok(())
    }
}

impl Parser for Skl {
    fn data_view(&self, ui: &mut egui::Ui, _filter: &str) { // TODO: Add filtering
        let files = &self.files;
        egui::Grid::new("aff_grid")
            .show(ui, |ui| {                        
                for file_key in files.keys().sorted() {
                    let skills = &files[file_key];
                    ui.collapsing(file_key, |ui| {
                        for skill in skills.skills.iter() {
                            let skill_header = CollapsingHeader::new(format!("id: {}", skill.id))
                                .default_open(true);
                            skill_header.show(ui, |ui| {
                                ui.horizontal(|h| {
                                    h.strong("req_points:");
                                    h.label(format!("{}", skill.req_points));
                                });
                                ui.horizontal(|h| {
                                    h.strong("reward_hash:");
                                    h.label(format!("{}", skill.reward_hash));
                                });
                                ui.horizontal(|h| {
                                    h.strong("is_root:");
                                    h.label(format!("{}", skill.is_root));
                                });
                                ui.horizontal(|h| {
                                    h.strong("x:");
                                    h.label(format!("{}", skill.x));
                                });
                                ui.horizontal(|h| {
                                    h.strong("y:");
                                    h.label(format!("{}", skill.y));
                                });
    
                                ui.push_id(skill.reward_hash, |c| {
                                    c.collapsing("connections", |ui| {
                                        for connection in skill.connections.iter() {
                                            ui.label(format!("{}", connection));
                                        }
                                    });
                                });
                            });
                        }
                    });
                    ui.end_row();
                }
            });
    }

    fn tab_title(&self) -> String {
        ".skl".to_owned()
    }
}

#[derive(Serialize, Deserialize)]
pub struct SklFile {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub skills: Vec<SklField>
}

impl SklFile {
    fn new() -> Self {
        Self {
            skills: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SklField {
    pub id: u32,
    pub req_points: u32,
    pub reward_hash: u32,
    pub is_root: bool,
    pub x: f32,
    pub y:f32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub connections: Vec<u32>
}

impl SklField {
    fn new(id: u32, req_points: u32, reward_hash: u32, is_root: bool, x: f32, y: f32, connections: Vec<u32>) -> Self {
        Self {
            id,
            req_points,
            reward_hash,
            is_root,
            x,
            y,
            connections
        }
    }
}