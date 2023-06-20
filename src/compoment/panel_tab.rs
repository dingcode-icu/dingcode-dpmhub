use crate::{
    api::{
        self,
        model::{DpmCellInfo, ECmdStatu, ERequestStatu},
    },
    error::ApiError,
};
use eframe::egui::{self, Ui};
use log::info;
use serde_json::json;
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
    is_first: bool, //初次打开panel
    is_needupdate_dpmloclist: bool,
    api_status: ERequestStatu,
    cmd_status: ECmdStatu,
    pm_infos: Vec<api::model::DpmCellInfo>,
    dpmloc_list: Vec<api::model::DpmCellInfo>,
    open_panel: PanelIndex,
    //children
    modal_loading: modal_loading::ModalLoading,
}

struct PanelTabApi {
    //status
    s_status: Sender<ERequestStatu>,
    r_status: Receiver<ERequestStatu>,

    //dpm
    s_cmd_dpm: Sender<(ECmdStatu, String)>,
    r_cmd_dpm: Receiver<(ECmdStatu, String)>,

    //api
    s_api_pmlist: Sender<Vec<api::model::DpmCellInfo>>,
    r_api_pmlist: Receiver<Vec<api::model::DpmCellInfo>>,

    s_api_dpminstalled: Sender<Vec<api::model::DpmCellInfo>>,
    r_api_dpminstalled: Receiver<Vec<api::model::DpmCellInfo>>,
}

impl Default for PanelTab {
    fn default() -> Self {
        let (s_api_pmlist, r_api_pmlist) = std::sync::mpsc::channel();
        let (s_cmd_dpm, r_cmd_dpm) = std::sync::mpsc::channel();
        let (s_status, r_status) = std::sync::mpsc::channel();
        let (s_api_dpminstalled, r_api_dpminstalled) = std::sync::mpsc::channel();
        let api = PanelTabApi {
            s_status,
            r_status,
            s_cmd_dpm,
            r_cmd_dpm,
            s_api_pmlist,
            r_api_pmlist,
            s_api_dpminstalled,
            r_api_dpminstalled,
        };

        Self {
            api,
            api_status: ERequestStatu::Idle,
            pm_infos: vec![],
            dpmloc_list: vec![],
            is_first: true,
            is_needupdate_dpmloclist: true,
            cmd_status: ECmdStatu::Idle,
            open_panel: Default::default(),
            //children
            modal_loading: modal_loading::ModalLoading::default(),
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

    fn is_installed_pm(&self, name: &str, ver: &str) -> bool {
        for info in &self.dpmloc_list {
            if info.name == name && info.ver == ver {
                return true;
            }
        }
        return false;
    }

    fn async_remote_list(idx: &PanelIndex) -> Result<Vec<api::model::DpmCellInfo>, ApiError> {
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

    fn shell_dpm_install(pm: String, tx: Sender<(ECmdStatu, String)>) {
        log::info!("start dpm install {}", pm);
        std::thread::spawn(move || {
            let rt = runtime::Builder::new_current_thread().build().unwrap();
            let _ = tx.send((ECmdStatu::Running, "install".to_owned()));
            let c = rt.block_on(async move {
                let out = Command::new("dpm").arg("install").arg(pm).output().await;
                if out.is_err() {
                    let _ = tx.send((ECmdStatu::Error, "install".to_owned()));
                }
                let _ = tx.send((ECmdStatu::Idle, "install".to_owned()));
            });
        });
    }

    fn shell_dpm_list(tx: Sender<(ECmdStatu, String)>, d_tx: Sender<Vec<api::model::DpmCellInfo>>) {
        std::thread::spawn(move || {
            let rt = runtime::Builder::new_current_thread().build().unwrap();
            let _ = tx.send((ECmdStatu::Running, "list".to_owned()));
            let c: () = rt.block_on(async move {
                let out = Command::new("dpm").arg("list").output().await;
                if out.is_err() {
                    let _ = tx.send((ECmdStatu::Error, "list".to_owned()));
                }
                let _ = tx.send((ECmdStatu::Idle, "list".to_owned()));
                let json_str = out.and_then(|f| {
                    if !f.status.success() || f.stderr.len() > 0 {
                        log::error!("dpm list cmd raise error:{:?}", f.stderr);
                        return Ok("[]".to_owned());
                    }
                    let out_str = String::from_utf8(f.stdout).unwrap_or_default();
                    let ret: Result<Vec<api::model::DpmCellInfo>, serde_json::Error> =
                        serde_json::from_str(out_str.as_str());
                    if let Ok(r) = ret {
                        let _ = d_tx.send(r.clone());
                        return Ok(json!(r).to_string());
                    }
                    return Ok("[]".to_owned());
                });
                info!("[cmd-dpm list] ret: {:?}", json_str.unwrap());
            });
        });
    }

    fn card_widget(&self, info: &DpmCellInfo, ui: &mut Ui) {
        let loc_info = self.dpmloc_list.iter().find(|f| f.name == info.name);
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading(&info.name);
                ui.separator();
                let is_ins: bool = self.is_installed_pm(&info.name, &info.ver);
                ui.horizontal(|ui| {
                    //uninstall
                    if is_ins {
                        let resp_unins = ui.button("uninstall");
                        if resp_unins.clicked() {}
                    } else {
                        //install
                        let resp_ins = ui.button("install");
                        if resp_ins.clicked() {
                            let _ = Self::shell_dpm_install(
                                format!("{}@{}", &info.name.as_str(), &info.ver.as_str()),
                                self.api.s_cmd_dpm.clone(),
                            );
                        }
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

                if is_ins {
                    if let Some(l) = loc_info {
                        if let Some(c) = &l.scripts {
                            ui.separator();
                            ui.horizontal(|ui| {
                                for cmd_name in c.keys() {
                                    let cmd =  c.get(cmd_name).unwrap_or(&"unknown".to_string()).to_owned();
                                    let  resp = ui.add_sized([50., 30.], egui::Button::new(cmd_name));
                                    if resp.clicked(){
                                        let cmd_ret: Result<std::process::Output, std::io::Error> = std::process::Command::new(&cmd).output();
                                        log::debug!("cmd :{:?}  ret :{:?}", &cmd,  cmd_ret);
                                    }
                                    ui.colored_label(
                                        egui::Color32::GRAY,
                                        format!(
                                            "cmd:{}",
                                            &cmd
                                        ),
                                    );
                                }
                            });
                        }
                    }
                }
            });
        });
    }

    fn reset_dpmloc_list(&mut self) {
        self.is_needupdate_dpmloclist = true;
        self.dpmloc_list = vec![];
    }

    pub fn watch_data(&mut self) {
        //chk update
        if self.is_needupdate_dpmloclist == true {
            self.dpmloc_list = vec![];
            self.is_needupdate_dpmloclist = false;
            Self::shell_dpm_list(
                self.api.s_cmd_dpm.to_owned(),
                self.api.s_api_dpminstalled.to_owned(),
            );
        }

        //status
        if let Ok(s) = self.api.r_status.try_recv() {
            log::debug!("req statu is :{:?}", s);
            self.api_status = s;
        }
        //cmd status
        if let Ok(s) = self.api.r_cmd_dpm.try_recv() {
            log::debug!("cmd status is : {:?}", s);

            if s.0 == ECmdStatu::Idle && s.1 == "install" && self.cmd_status == ECmdStatu::Running {
                self.reset_dpmloc_list();
            }
            self.cmd_status = s.0.to_owned();
        }
        //--api data--
        //pmlist
        if let Ok(pm_list) = self.api.r_api_pmlist.try_recv() {
            log::debug!("api already receive val->{:?}", pm_list);
            if pm_list.len() > 0 {
                self.pm_infos = pm_list;
                self.reset_dpmloc_list();
            }
        }
        //dpmlist
        if let Ok(dpm_list) = self.api.r_api_dpminstalled.try_recv() {
            log::debug!("dpm list receive val->{:?}", dpm_list);
            if dpm_list.len() > 0 {
                self.dpmloc_list = dpm_list
            }
        }
    }

    pub fn handle_modal(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        //modal  installing
        if self.cmd_status == ECmdStatu::Running {
            self.modal_loading.show(ctx, ui);
        } else {
            self.modal_loading.drop();
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        // watch data
        self.watch_data();
        self.handle_modal(ctx, ui);

        // main content
        ui.horizontal(|ui| {
            let mut resp = ui.selectable_value(&mut self.open_panel, PanelIndex::Binary, "Binary");
            resp |= ui.selectable_value(&mut self.open_panel, PanelIndex::Lib, "Lib");
            resp |= ui.selectable_value(&mut self.open_panel, PanelIndex::Unknown, "Unknown");

            if resp.changed() || self.is_first {
                self.pm_infos.clear();
                self.is_first = false;
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

                            ui.add_space(10.);
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
