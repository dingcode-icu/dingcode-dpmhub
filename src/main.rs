#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[macro_use]
extern crate lazy_static;

use eframe::egui::{self};
mod api;
mod cmd;
mod compoment;
mod config;
mod error;
mod native_opt;

struct MainApp {
    panel_tab: compoment::PanelTab,
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    //tokio runtime background
    let rt = tokio::runtime::Runtime::new().expect("New tokio runtime failed!");
    let _ = rt.enter();
    native_opt::run::<MainApp>()
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            panel_tab: compoment::PanelTab::default(),
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dpm for cocos-creator-v2");
            ui.separator();

            self.panel_tab.ui(ctx, ui);
        });
    }
}
