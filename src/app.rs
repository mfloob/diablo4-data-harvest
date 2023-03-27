use egui_dock::{Tree, DockArea, Style};

use crate::stl;

pub struct App {
    tree: Tree<String>,
}

impl Default for App {
    fn default() -> Self {
        let tree = Tree::new(vec!["placeholder".to_owned()]);
        Self { tree }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { tree } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Parse stl folder...").clicked() {
                        ui.close_menu();
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            let path = path.display().to_string();
                            let _stl = stl::Stl::run(path).unwrap();
                        }
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel")
            .min_width(150f32)
            .show(ctx, |ui| {
                ui.heading("Side Panel");
                ui.horizontal(|ui| {
                    ui.label("Side placeholder");
                });
        });

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut TabViewer {});
    }
}

struct TabViewer {}

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {tab}"));
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }
}