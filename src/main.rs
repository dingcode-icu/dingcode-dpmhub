#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[macro_use]
extern crate lazy_static;

use eframe::egui::{self};
use fast_log::Config;
// use dir
mod api;
mod cmd;
mod compoment;
mod config;
mod error;
mod native_opt;

struct MainApp {
    panel_tab: compoment::panel_tab::PanelTab,
}

fn main() -> Result<(), eframe::Error> {
    //log
    let log_devf = std::env::current_dir().unwrap().join("logs/dpmhub_gui.log");
    fast_log::init(
        Config::new()
            .level(log::LevelFilter::Info)
            .file_loop(
                &log_devf.display().to_string(),
                fast_log::consts::LogSize::MB(2),
            )
            .console()
            .chan_len(Some(100000)),
    )
    .unwrap();

    native_opt::run::<MainApp>()
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            panel_tab: compoment::panel_tab::PanelTab::default(),
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
