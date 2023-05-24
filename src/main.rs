#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[macro_use]
extern crate lazy_static;

use eframe::egui::{self};
use tokio::runtime;
mod compoment;
mod config;
mod api;
mod error;
mod cmd;
mod native_opt;

struct MainApp {
    rt:tokio::runtime::Runtime,
    panel_tabled: compoment::PanelTab,
}


fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    native_opt::run::<MainApp>()
}


impl Default for MainApp {
    fn default() -> Self {
        Self {
            rt: runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap(),
            panel_tabled: compoment::PanelTab::default(),
        }
    }
}


impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dpm for cocos-creator-v2");
            ui.separator(); 
            
            self.panel_tabled.ui(ctx, ui);
        });
    }
}