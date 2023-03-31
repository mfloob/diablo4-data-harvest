use egui::vec2;
use egui_dock::{Tree, DockArea, Style};
use crate::{Stl,Aff, parsers::{Parser, skl::Skl}, utils};

pub struct AppContext {
    tabs: Tree<FileTab>,
    search: String
}

pub struct App {
    data: Box<AppContext>
}

impl Default for App {
    fn default() -> Self {
        let tabs = Tree::new(Vec::new());
        let search = Default::default();
        let data = AppContext { tabs, search };

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
                            let _stl = Stl::new().run(path).unwrap();
                        }
                    }
                    if ui.button("Parse aff folder...").clicked() {
                        ui.close_menu();
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            let path = path.display().to_string();
                            let _aff = Aff::new().run(path).unwrap();
                        }
                    }
                    if ui.button("Parse skl folder...").clicked() {
                        ui.close_menu();
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            let path = path.display().to_string();
                            let _skl = Skl::new().run(path).unwrap();
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
                            let buf = utils::read_file(path).unwrap();                            
                            let data_str = String::from_utf8(buf).unwrap();

                            let stl: Stl = serde_json::from_str(&data_str).unwrap();
                            let tab = FileTab::new(Box::new(stl) as Box<dyn Parser>);
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
                            let buf = utils::read_file(path).unwrap();
                            let data_str = String::from_utf8(buf).unwrap();

                            let aff: Aff = serde_json::from_str(&data_str).unwrap();
                            let tab = FileTab::new(Box::new(aff) as Box<dyn Parser>);
                            self.data.tabs.push_to_focused_leaf(tab);
                            self.data.search = Default::default();
                    }
                }
                if ui.button("Load skl data").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter(".json files", &["json"])
                        .set_file_name("skl.json")
                        .pick_file() {
                            let path = path.display().to_string();
                            let buf = utils::read_file(path).unwrap();
                            let data_str = String::from_utf8(buf).unwrap();

                            let aff: Skl = serde_json::from_str(&data_str).unwrap();
                            let tab = FileTab::new(Box::new(aff) as Box<dyn Parser>);
                            self.data.tabs.push_to_focused_leaf(tab);
                            self.data.search = Default::default();
                    }
                }
        });

        if self.data.tabs.num_tabs() > 0 {
            DockArea::new(&mut self.data.tabs)
                .style(Style::from_egui(ctx.style().as_ref()))
                .show(ctx, &mut FileViewer {
                    filter: self.data.search.clone()
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

struct FileTab {
    parser: Box<dyn Parser>,
}

impl FileTab {
    fn new(parser: Box<dyn Parser>) -> Self {
        Self {
            parser
        }
    }
}

struct FileViewer {
    filter: String
}

impl egui_dock::TabViewer for FileViewer {
    type Tab = FileTab;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.parser.data_view(ui, &self.filter);
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.parser.tab_title().into()
    }
}