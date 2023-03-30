pub mod stl;
pub mod aff;

pub trait Parser {
    fn data_view(&self, ui: &mut egui::Ui, filter: &str);
    fn tab_title(&self) -> String;
}

pub trait ParserFile {

}