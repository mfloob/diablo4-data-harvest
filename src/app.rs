use std::{fs::File, io::Read, collections::HashMap};
use egui::vec2;
use itertools::Itertools;
use egui_dock::{Tree, DockArea, Style};

use crate::{Stl,Aff, parsers::{stl::StlFile, aff::AffFile}};

pub struct AppContext {
    stl: Stl,
    aff: Aff,
    tabs: Tree<FileTab>,
    search: String
}

pub struct App {
    data: Box<AppContext>
}

impl Default for App {
    fn default() -> Self {
        let tabs = Tree::new(Vec::new());
        let stl = Stl::new();
        let aff = Aff::new();
        let search = Default::default();
        let data = AppContext { stl, aff, tabs, search };

        Self { 
            data: Box::new(data) 
        }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { data } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Parse stl folder...").clicked() {
                        ui.close_menu();
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            let path = path.display().to_string();
                            let _stl = self.data.stl.run(path).unwrap();
                        }
                    }
                    if ui.button("Parse aff folder...").clicked() {
                        ui.close_menu();
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            let path = path.display().to_string();
                            let _aff = self.data.aff.run(path).unwrap();
                            
                        }
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });

                if self.data.tabs.num_tabs() > 0 {
                    ui.allocate_space(vec2(110f32, 1f32));
                    if ui.button("clear").clicked() {
                        self.data.search = Default::default();
                    }
                    ui.add(egui::TextEdit::singleline(&mut self.data.search)
                        .hint_text("Search in json tabs")
                        .desired_width(f32::INFINITY));
                }
                else {
                    self.data.search = Default::default();
                }
            });
        });

        egui::SidePanel::left("side_panel")
            .min_width(110f32)
            .default_width(110f32)
            .max_width(150f32)
            .show(ctx, |ui| {
                ui.heading("Data Viewers");
                ui.separator();
                if ui.button("Load stl data").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter(".json files", &["json"])
                        .set_file_name("stl.json")
                        .pick_file() {
                            let path = path.display().to_string();
                            let mut f = File::open(path).unwrap();
                            let mut buf = Vec::new();
                            let _ = f.read_to_end(&mut buf).unwrap();
                            let data_str = String::from_utf8(buf).unwrap();

                            let stl = serde_json::from_str(&data_str).unwrap();
                            let tab = FileTab::new(FileType::Stl, stl, Aff::new());
                            self.data.tabs.push_to_focused_leaf(tab);
                            self.data.search = Default::default();
                    }
                }
                if ui.button("Load aff data").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter(".json files", &["json"])
                        .set_file_name("aff.json")
                        .pick_file() {
                            let path = path.display().to_string();
                            let mut f = File::open(path).unwrap();
                            let mut buf = Vec::new();
                            let _ = f.read_to_end(&mut buf).unwrap();
                            let data_str = String::from_utf8(buf).unwrap();

                            let aff = serde_json::from_str(&data_str).unwrap();
                            let tab = FileTab::new(FileType::Aff, Stl::new(), aff);
                            self.data.tabs.push_to_focused_leaf(tab);
                            self.data.search = Default::default();
                    }
                }
        });

        if self.data.tabs.num_tabs() > 0 {
            DockArea::new(&mut self.data.tabs)
                .style(Style::from_egui(ctx.style().as_ref()))
                .show(ctx, &mut FileViewer {
                    search: self.data.search.clone()
                });            
        }
        else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("Load a json file to view");
                });
            });
        }
    }
}

enum FileType {
    Stl,
    Aff
}

struct FileTab {
    file_type: FileType,
    stl: Stl,
    aff: Aff
}

impl FileTab {
    fn new(file_type: FileType, stl: Stl, aff: Aff) -> Self {
        Self {
            file_type,
            stl,
            aff
        }
    }

    fn stl(&self) -> &HashMap<String, StlFile> {
        &self.stl.files
    }

    fn aff(&self) -> &HashMap<String, AffFile> {
        &self.aff.files
    }
}

struct FileViewer {
    search: String
}

impl egui_dock::TabViewer for FileViewer {
    type Tab = FileTab;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab.file_type {
            FileType::Stl => {
                let files = tab.stl();
                egui::Grid::new("stl_grid")
                    .show(ui, |ui| {
                        for item in files.keys().filter(|x| self.search.is_empty() || x.to_lowercase().contains(&self.search)).sorted() {
                            if files[item].fields.len() > 0 {
                                ui.collapsing(item, |ui| {
                                    let values = &files[item];
                                    for value in values.fields.keys().sorted() {
                                        ui.horizontal(|ui| {
                                            ui.strong(format!("{}:", value));
                                            ui.label(&values.fields[value]);
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
            },
            FileType::Aff => {
                let files = tab.aff();
                egui::Grid::new("aff_grid")
                    .show(ui, |ui| {                        
                        for item in files.keys().filter(|x| self.search.is_empty() || x.to_lowercase().contains(&self.search)).sorted() {
                            if files[item].values.len() > 0 {
                                ui.collapsing(item, |ui| {
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
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab.file_type {
            FileType::Stl => ".stl".into(),
            FileType::Aff => ".aff".into()
        }
    }
}