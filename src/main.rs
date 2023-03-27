#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{io::{self}, env};

mod utils;
mod stl;
mod app;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let gui_mode = match args.len() {
        1 => true,
        _ => false // for now, assume more than default arguments is not gui mode
    };

    // TODO: some sort of named argument system
    if gui_mode {
        run_gui();
    }
    else {
        let path = &args[1];
        let _stl = stl::Stl::run(path.to_string())?;
    }

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn run_gui() {
    let _result = native().unwrap();
}

#[cfg(target_arch = "wasm32")] // TODO: web
fn run_gui() {
    let _result = todo!("Implement web ui");
}

fn native() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "diablo4 data harvest",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
}
