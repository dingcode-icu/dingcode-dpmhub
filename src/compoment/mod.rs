use crate::api::{self, model::DpmCellInfo};
use eframe::{
    egui::{self, Context, Ui},
    epaint::Color32,
    glow::NONE,
};
use egui_extras::{Size, StripBuilder};
use log::info;
use std::{string, sync::Arc};
use tokio::{runtime, task::spawn_blocking};

// ----------------------------------------------------------------------------
// PanelTab

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum PanelIndex {
    Binary,
    Lib,
    Unknown,
}

impl Default for PanelIndex {
    fn default() -> Self {
        Self::Binary
    }
}

pub struct PanelTab {
    rt: tokio::runtime::Runtime,
    is_first: bool,
    pm_list: Vec<DpmCellInfo>,
    open_panel: PanelIndex,
}

impl Default for PanelTab {
    fn default() -> Self {
        Self {
            rt: runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            pm_list: vec![],
            is_first: false,
            open_panel: Default::default(),
        }
    }
}

pub async fn test() {}

impl PanelTab {
    fn get_open_pmtype(idx: &PanelIndex) -> &str {
        let c = match idx {
            PanelIndex::Binary => {
                if cfg!(windows) {
                    return "bin-win32";
                } else if cfg!(target_os = "macos") {
                    return "bin-maxos";
                }
                "unknow"
            }
            PanelIndex::Lib => "ccv2",
            PanelIndex::Unknown => "unknown",
        };
        c
    }

    fn async_remote_list(idx: &PanelIndex) -> Option<Vec<super::api::model::DpmCellInfo>> {
        let pm_type = Self::get_open_pmtype(idx);
        info!("async remote with type :{}", pm_type);
        let resp = api::ApiRsvr::get_pmlist(pm_type);
        if resp.is_err() {
            log::error!("resp remote api raise error:{:?}", resp.as_ref().err());
        }
        resp.ok()
    }

    fn api_get_pmlist(&self, sender: std::sync::mpsc::Sender<Option<Vec<DpmCellInfo>>>) {
        let idx = self.open_panel.to_owned();
        let _ = &self.rt.spawn_blocking(move || {
            let inner_idx = idx.to_owned();
            log::info!("call change api req");
            let ret = Self::async_remote_list(&inner_idx);

            let _ = sender.send(ret);
        });
    }

    fn dpm_install(pm: &str) -> (bool, String) {
        let result = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let child = std::process::Command::new("dpm")
                    .args(vec!["install", "tracer-inspector@0.0.3"])
                    .spawn()
                    .expect("[dpm-cmd ]failed!");
                let c = child.wait_with_output().expect("failed to wait on child");
                let ret: String = String::from_utf8_lossy(&c.stdout).into_owned();
                if c.status.success() {
                    return (true, ret);
                }
                println!("{}", String::from_utf8_lossy(&c.stdout).into_owned());
                return (false, String::from_utf8_lossy(&c.stderr).into_owned());
            });

        result
    }

    fn card_widget(info: &DpmCellInfo, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading(&info.name);
                ui.horizontal(|ui| {
                    let resp = ui.button("install");
                    if resp.clicked() {
                        let r = Self::dpm_install(
                            format!("{}@{}", &info.name.as_str(), &info.ver.as_str()).as_str(),
                        );
                        println!("-->>install code: {}, ret:{}", r.0, r.1);
                        // if r.0 {

                        // }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("runtime:");
                    ui.label(&info.runtime);
                });
                ui.horizontal(|ui| {
                    ui.label("ver:");
                    ui.label(&info.ver);
                });
                ui.horizontal(|ui| {
                    ui.label("size:");
                    ui.label(String::from(info.cont_size.unwrap_or_default().to_string()));
                });
                ui.horizontal(|ui| {
                    ui.label("license:");
                    ui.label(&info.license.to_owned().unwrap_or_default());
                });
                ui.label("Desc:");
                ui.label(if info.descript.len() > 0 {
                    &info.descript
                } else {
                    "No Description"
                });
            });
        });
    }

    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        //todo: check the perform to create channel var
        let (sender, reciver) = std::sync::mpsc::channel();

        ui.horizontal(|ui| {
            let mut resp = ui.selectable_value(&mut self.open_panel, PanelIndex::Binary, "Binary");
            resp |= ui.selectable_value(&mut self.open_panel, PanelIndex::Lib, "Lib");
            resp |= ui.selectable_value(&mut self.open_panel, PanelIndex::Unknown, "Unknown");

            if resp.changed() || !self.is_first {
                self.is_first = true;
                self.api_get_pmlist(sender);
                log::info!("fake end");
            }
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                ui.vertical_centered(|ui| {
                    //chk index api
                    if let Ok(rc) = reciver.recv() {
                        let pm_list = rc.unwrap_or_default();
                        self.pm_list = pm_list;
                    };

                    if self.pm_list.len() > 0 {
                        let mut my_value = 42;
                        for p in &self.pm_list {
                            Self::card_widget(p, ui);
                        }
                    } else {
                        ui.heading("No Data");
                    }
                })
            });
        });
    }
}
