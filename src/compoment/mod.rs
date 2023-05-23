use crate::api;
use eframe::{
    egui::{self, Context, Ui},
    epaint::Color32,
    glow::NONE,
};
use egui_extras::{Size, StripBuilder};
use log::info;
use std::sync::Arc;
use tokio::runtime;

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
    open_panel: PanelIndex,
}

impl Default for PanelTab {
    fn default() -> Self {
        Self {
            rt: runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
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
                    return "bin-win32"
                }
                else if cfg!(target_os = "macos") {
                    return "bin-maxos"
                }
                "unknow"
            },
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

    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        let (sender, reciver) = std::sync::mpsc::channel();
        ui.horizontal(|ui| {
            let mut resp = ui.selectable_value(&mut self.open_panel, PanelIndex::Binary, "Binary");
            resp |= ui.selectable_value(&mut self.open_panel, PanelIndex::Lib, "Lib");
            resp |= ui.selectable_value(&mut self.open_panel, PanelIndex::Unknown, "Unknown");

            if resp.changed() {
                let idx = self.open_panel.to_owned();
                let _ = &self.rt.spawn_blocking(move || {
                    let inner_idx = idx.to_owned();
                    log::info!("call change api req");
                    let ret = Self::async_remote_list(&inner_idx);

                    let _ = sender.send(ret);
                    
                });

                log::info!("fake end");
            }
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                ui.vertical_centered(|ui| {
                    //chk index api
                    if let Ok(rc) = reciver.recv() {
                        let pm_list= rc.unwrap_or_default();
                        for p in pm_list {
                            
                        }
                    }
                    else {
                        ui.heading("No Data");
                    }

                    // match self.open_panel {
                    //     PanelIndex::Binary => {}
                    //     PanelIndex::Lib => {}
                    //     PanelIndex::Unknown => {
                            
                    //     }
                    // };
                })
            });
        });
    }
}
