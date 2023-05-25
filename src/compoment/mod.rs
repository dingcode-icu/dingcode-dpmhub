use crate::api::{self, model::DpmCellInfo};
use eframe::{
    egui::{self, Ui},
};
use log::info;
use std::{sync::{Arc, Mutex}};
use tokio::{runtime::{self, Runtime}, process::Command};



// Panel----------------------------------------------------------------------------
// PanelTab

pub struct PanelTab {
    rt: tokio::runtime::Runtime,
    is_first: bool,
    open_panel: PanelIndex,
    pm_list_arc: Option<Vec<DpmCellInfo>>
}


lazy_static! {
    pub static ref CFG: Arc<Mutex<PanelTab>> = Arc::new(Mutex::new(PanelTab::default()));
}


impl Default for PanelTab {
    fn default() -> Self {
        Self {
            rt: runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            is_first: false,
            open_panel: Default::default(),
            pm_list_arc:  None
        }
    }
}

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

    fn cmd_dpminstall_pkg(rt: &Runtime) {
        rt.spawn(async move {
            let cc = tokio::process::Command::new("dpm")
                .args(vec!["install", "tracer-inspector@0.0.3"])
                .spawn()
                .expect("[dpm-cmd ]failed!");
            let _c = cc.wait_with_output().await;
            // let mut c= val.lock().unwrap();
            // *c = None;
        });
    }

    fn api_get_pmlist(idx: PanelIndex,  rt: &Runtime) {

        let inner = idx.to_owned();
        rt.spawn(async move {
            let ret = Self::async_remote_list(&inner);
            // let mut d = CFG.lock().unwrap().is_first;
            CFG.lock().unwrap().pm_list_arc = ret;
        });
    }

    fn dpm_install(pm: &str) -> (bool, String) {
        let result = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let c = Command::new("dpm")
                    .args(vec!["install", "tracer-inspector@0.0.3"])
                    .output().await
                    .expect("[dpm-cmd ]failed!");
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

    pub fn ui<'a>(&'a mut self, ctx: &egui::Context, ui: &mut Ui) {
        //todo: check the perform to create channel var
        // let (sender, reciver) = std::sync::mpsc::channel();

        ui.horizontal(|ui| {
            let mut resp = ui.selectable_value(&mut self.open_panel, PanelIndex::Binary, "Binary");
            resp |= ui.selectable_value(&mut self.open_panel, PanelIndex::Lib, "Lib");
            resp |= ui.selectable_value(&mut self.open_panel, PanelIndex::Unknown, "Unknown");

            if resp.changed() || !self.is_first {
                self.is_first = true;
                Self::api_get_pmlist(self.open_panel, &self.rt);
                log::info!("fake end");
            }
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                ui.vertical_centered(|ui| {
                    //chk index api

                    let c=  CFG.lock().unwrap();
                    let pm_list = c.pm_list_arc.as_ref().unwrap();
                    if pm_list.len() > 0 {
                        let mut my_value = 42;
                        for p in pm_list {
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

// Enum----------------------------------------------------------------------------

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
