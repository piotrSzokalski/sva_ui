use egui::{Context, Ui};
use simple_virtual_assembler::vm::virtual_machine::VmStatus;

#[derive(Debug, PartialEq)]
pub enum ComponentAction {
    DoNothing,
    ToggleVmVisibility(usize),
    RenameVm(usize),
    RemoveVm(usize),
    ToggleRamVisibility(usize),
    RenameRam(usize),
    RemoveRam(usize),
}

pub struct ComponentListWidget {
    id: usize,
    name: String,
    is_active: bool,
    status: Option<VmStatus>,
    is_vm: bool,
    has_stack: bool,
}

impl ComponentListWidget {
    pub fn new(
        id: usize,
        name: String,
        is_active: bool,
        status: Option<VmStatus>,
        is_vm: bool,
        has_stack: bool,
    ) -> Self {
        ComponentListWidget {
            id,
            name,
            is_active,
            status,
            is_vm,
            has_stack,
        }
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) -> ComponentAction {
        let mut action = ComponentAction::DoNothing;
        ui.vertical(|ui| {
            ui.heading(&self.name);
            ui.separator();
            let show_hide_button_text = if self.is_active { "hide" } else { "show" };
            if ui.button(show_hide_button_text).clicked() {
                action = if self.is_vm {
                    ComponentAction::ToggleVmVisibility(self.id)
                } else {
                    ComponentAction::ToggleRamVisibility(self.id)
                }
            }
            if ui.button("rename").clicked() {
                action = if self.is_vm {
                    ComponentAction::RenameVm(self.id)
                } else {
                    ComponentAction::RenameRam(self.id)
                }
            }
            if ui.button("remove").clicked() {
                action = if self.is_vm {
                    ComponentAction::RemoveVm(self.id)
                } else {
                    ComponentAction::RemoveRam(self.id)
                }
            }
        });
        action
    }
}
