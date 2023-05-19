use eframe::egui;

use crate::config;

pub fn run<T:Default + eframe::App + 'static>() -> Result<(), eframe::Error> {
    let options: eframe::NativeOptions = eframe::NativeOptions { 
        vsync: false,
        resizable: false,
        centered: true,
        initial_window_size: Some(egui::vec2(480., 800.)),
        ..Default::default()
    };
    eframe::run_native(
        &config::CFG.title,
        options,
        Box::new(|_cc| Box::<T>::default()),
    )
}