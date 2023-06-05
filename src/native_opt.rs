use crate::config;
use eframe::egui;

fn setup_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../static/FZLanTYJW.TTF")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

fn setup() {}

pub fn run<T: Default + eframe::App + 'static>() -> Result<(), eframe::Error> {
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
        Box::new(|_cc| {
            setup_font(&_cc.egui_ctx);
            Box::<T>::default()
        }),
    )
}
