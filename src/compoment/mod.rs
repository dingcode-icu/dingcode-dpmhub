use crate::api;
use eframe::{
    egui::{self, Context, Ui},
    epaint::Color32, glow::NONE,
};
use egui_extras::{Size, StripBuilder};
use log::info;
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
            PanelIndex::Binary => "bin",
            PanelIndex::Lib => "ccv2",
            PanelIndex::Unknown => "unknown",
        };
        c
    }

    fn async_remote_list(idx: &PanelIndex) -> Option<Vec<super::api::model::DpmCellInfo>> {
        let pm_type = Self::get_open_pmtype(idx);
        info!("async remote with type :{}", pm_type);
        let resp = api::ApiRsvr::get_pmlist(pm_type);
        resp.ok()
    }

    pub fn ui(&mut self, &rt: tokio::runtime::Runtime, ctx: &egui::Context, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let mut resp = ui.selectable_value(&mut self.open_panel, PanelIndex::Binary, "Binary");
            resp |= ui.selectable_value(&mut self.open_panel, PanelIndex::Lib, "Lib");
            resp |= ui.selectable_value(&mut self.open_panel, PanelIndex::Unknown, "Unknown");
            let mut c : Option<Vec<api::model::DpmCellInfo>>= None;
            if resp.changed() {
                log::info!("change open_panel {:?}", self.open_panel);
                let idx = self.open_panel.to_owned();


                
                
                
                let t: &tokio::task::JoinHandle<()> = &self.rt.spawn_blocking( move || {
                    let inner_idx = idx.to_owned();
                    log::info!("call change api req");
                    let ret = Self::async_remote_list(&inner_idx);
                    log::info!("req ret is {:?}", ret);
                    c = ret;
     
                });
                log::info!("fake end");
                
            }
            if c.is_some() {
                log::info!("final update");
            }
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                ui.vertical_centered(|ui| {
                    //chk index api
                    // if let Some(r) = self.async_remote_list() {
                    //
                    // } else {
                    //
                    // }

                    match self.open_panel {
                        PanelIndex::Binary => {}
                        PanelIndex::Lib => {}
                        PanelIndex::Unknown => {
                            ui.heading("No Data");
                        }
                    };
                })
            });
        });
    }
}
