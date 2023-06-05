use eframe::egui::{self, Ui};
use egui_modal::Modal;

pub struct ModalLoading {
    modal: Option<Modal>, 
    name: String,
    txt_title: String,
}
 
impl Default for ModalLoading {
    fn default() -> Self {
        Self {
            modal: None,
            name: "Loading".to_owned(),
            txt_title: "Installing".to_owned(),
        }
    }
}

impl ModalLoading {
    pub fn show(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        let modal = Modal::new(ctx, &self.name);
        modal.show(|ui| {
            modal.title(ui, &self.txt_title);
            ui.vertical_centered(|ui|{ 
                ui.spinner();
            });
        });
        modal.open();
        self.modal = Some(modal);
    }
    
    pub fn drop(&mut self) {
        if self.modal.is_some() {
            self.modal.as_ref().unwrap().close();
            self.modal = None;
        }
    }
}
