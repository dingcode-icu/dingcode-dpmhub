use crate::{
    api::{
        self,
        model::{DpmCellInfo, ERequestStatu, ECmdStatu},
    },
    error::ApiError,
};
use eframe::egui::{self, Ui};
use log::info;
use std::sync::{mpsc::Receiver, mpsc::Sender};
use tokio::{
    process::Command,
    runtime::{self},
};

use super::modal_loading;


// Panel----------------------------------------------------------------------------
// PanelTab

pub struct PanelTab {
    //data
    api: PanelTabApi,
    is_first: bool,
    api_status: ERequestStatu,
    cmd_status: ECmdStatu,
    pm_infos: Vec<api::model::DpmCellInfo>,
    open_panel: PanelIndex,
    //children
    modal_loading: modal_loading::ModalLoading
}

struct PanelTabApi {
    //status
    s_status: Sender<ERequestStatu>,
    r_status: Receiver<ERequestStatu>,

    //cmd
    s_cmd_dpm: Sender<ECmdStatu>,
    r_cmd_dpm: Receiver<ECmdStatu>,

    //api
    s_api_pmlist: Sender<Vec<api::model::DpmCellInfo>>,
    r_api_pmlist: Receiver<Vec<api::model::DpmCellInfo>>,
}

impl Default for PanelTab {
    fn default() -> Self {
        let (s_api_pmlist, r_api_pmlist) = std::sync::mpsc::channel();
        let (s_cmd_dpm, r_cmd_dpm) = std::sync::mpsc::channel();
        let (s_status, r_status) = std::sync::mpsc::channel();
        let api = PanelTabApi {
            s_status,
            r_status,
            s_cmd_dpm,
            r_cmd_dpm,
            s_api_pmlist,
            r_api_pmlist,
        };

        Self {
            api,
            api_status: ERequestStatu::Idle,
            pm_infos: vec![],
            is_first: false,
            cmd_status: ECmdStatu::Idle,
            open_panel: Default::default(),
            //children
            modal_loading: modal_loading::ModalLoading::default()
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

    fn async_remote_list(
        idx: &PanelIndex,
    ) -> Result<Vec<api::model::DpmCellInfo>, ApiError> {
        let pm_type = Self::get_open_pmtype(idx);
        info!("async remote with type :{}", pm_type);
        let resp = api::ApiRsvr::get_pmlist(pm_type);
        resp
    }

    fn api_get_pmlist(idx: PanelIndex, tx: Sender<Vec<DpmCellInfo>>, statu: Sender<ERequestStatu>) {
        let inner = idx.to_owned();
        std::thread::spawn(move || {
            let rt = runtime::Builder::new_current_thread().build().unwrap();
            let _ = statu.send(ERequestStatu::Requesting);
            rt.block_on(async move {
                let ret = Self::async_remote_list(&inner);
                println!("{:?}", ret);
                if let Ok(l) = ret {
                    let _ = tx.send(l);
                    let _ = statu.send(ERequestStatu::Idle);
                } else {
                    let _ = statu.send(ERequestStatu::Error);
                }
            });
        });
    }

    fn shell_dpm_install(pm: String, tx: Sender<ECmdStatu>) {
        log::info!("start dpm install {}", pm);
        std::thread::spawn(move || {
            let rt = runtime::Builder::new_current_thread().build().unwrap();
            let _ = tx.send(ECmdStatu::Running);
            let c = rt.block_on(async move {
                let out = Command::new("dpm").arg("install").arg(pm).output().await;
                if out.is_err() {
                    let _ = tx.send(ECmdStatu::Error);
                }
                let _ = tx.send(ECmdStatu::Idle);
            });
        });
    }

    fn card_widget(&self, info: &DpmCellInfo, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading(&info.name);
                ui.horizontal(|ui| {
                    //install
                    let resp_ins = ui.button("install");
                    if resp_ins.clicked() {
                        let r = Self::shell_dpm_install(
                            
                            format!("{}@{}", &info.name.as_str(), &info.ver.as_str()),
                            self.api.s_cmd_dpm.clone(),
                        );
                    }

                    //doc
                    let resp_btn = ui.button("doc");
                    if resp_btn.clicked() {}
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

    pub fn watch_data(&mut self) {
        //status
        if let Ok(s) = self.api.r_status.try_recv() {
            log::info!("req statu is :{:?}", s);
            self.api_status = s;
        }
        //cmd status
        if let Ok(s) = self.api.r_cmd_dpm.try_recv() {
            log::info!("cmd status is : s  ");
            self.cmd_status = s;
        }
        //api data
        if let Ok(pm_list) = self.api.r_api_pmlist.try_recv() {
            log::info!("api already receive val->{:?}", pm_list);
            if pm_list.len() > 0 {
                self.pm_infos = pm_list
            }
        }
    }

    pub fn handle_modal(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        //modal  installing
        if self.cmd_status == ECmdStatu::Running {
            self.modal_loading.show(ctx, ui);
        }
        else {
            self.modal_loading.drop();
        }
    }

    pub fn ui<'a>(&'a mut self, ctx: &egui::Context, ui: &mut Ui) {
        // watch data
        self.watch_data();
        self.handle_modal(ctx, ui);


        // main content
        ui.horizontal(|ui| {
            let mut resp = ui.selectable_value(&mut self.open_panel, PanelIndex::Binary, "Binary");
            resp |= ui.selectable_value(&mut self.open_panel, PanelIndex::Lib, "Lib");
            resp |= ui.selectable_value(&mut self.open_panel, PanelIndex::Unknown, "Unknown");

            if resp.changed() || !self.is_first {
                self.pm_infos.clear();
                self.is_first = true;
                Self::api_get_pmlist(
                    self.open_panel,
                    self.api.s_api_pmlist.clone(),
                    self.api.s_status.clone(),
                );
            }
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                ui.vertical_centered(|ui| {
                    if self.pm_infos.len() > 0 {
                        for p in &self.pm_infos {
                            self.card_widget(&p, ui);
                        }
                    } else {
                        match self.api_status {
                            ERequestStatu::Requesting => {
                                ui.heading("Asyncing...");
                                ui.spinner();
                            }
                            ERequestStatu::Error => {
                                ui.heading("No Data");
                            }
                            _ => {}
                        }
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
