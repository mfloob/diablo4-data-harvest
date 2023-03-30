pub mod stl;
pub mod aff;

pub trait Parser {
    fn data_view(&self, ui: &mut egui::Ui, filter: &String);

    fn tab_title(&self) -> String;
}


pub trait ParserFile {

}