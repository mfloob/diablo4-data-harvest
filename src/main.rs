#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{io::{self}, env, fs};
use parsers::{stl::Stl, aff::Aff, skl::Skl};

mod utils;
mod parsers;
mod app;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let gui_mode = match args.len() {
        1 => true,
        _ => false // for now, assume more than default arguments is not gui mode
    };

    if gui_mode {
        run_gui();
    }
    else {
        let path = &args[1];
        run_cli(path.to_string())?;
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

fn run_cli(path: String) -> io::Result<()> {
    let p = path.as_str();
    let dir = fs::read_dir(p)?;
    let last_file =  dir.last().unwrap()?.path();
    let extension = last_file.extension().unwrap();

    match extension.to_str() {
        Some("stl") => {
            let mut stl = Stl::new();
            let _ = stl.run(path);
        },
        Some("aff") => {
            let mut aff = Aff::new();
            let _ = aff.run(path);
        },
        Some("skl") => {
            let mut skl = Skl::new();
            let _ = skl.run(path);
        }
        _ => todo!("Not yet implemented")
    }

    Ok(())
}