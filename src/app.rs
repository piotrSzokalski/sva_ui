use std::fmt::Display;

use egui::Button;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};

use simple_virtual_assembler::assembler::assembler::Assembler;
use simple_virtual_assembler::assembler::parsing_err::ParsingError;
use simple_virtual_assembler::vm::instruction::Instruction;
use simple_virtual_assembler::vm::virtual_machine::VirtualMachine;

use crate::sva_shell::SVAShell;

use crate::test_window::TetsWindow;

use crate::test::abc;

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
enum Language {
    /// Polish
    Pl,
    /// English
    En,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Language::Pl => "Polski",
            Language::En => "English",
        };

        write!(f, "{}", name)
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SvaUI {
    language: Language,
    //vm_shell: SVAShell,
    vms: Vec<SVAShell>,
}

impl Default for SvaUI {
    fn default() -> Self {
        Self {
            // Example stuff:
            // vm_shell: SVAShell::new(0, "First VM window".to_string()),
            language: Language::En,
            vms: Vec::new(),
        }
    }
}

impl SvaUI {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for SvaUI {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);

                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.language.to_string()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.language, Language::Pl, "Polski");
                        ui.selectable_value(&mut self.language, Language::En, "English");
                    });
                ui.heading("Simple virtual assembler ui");

                ui.menu_button("Add", |ui| {
                    if ui.button("vm").clicked() {
                        let id = self.vms.last().map_or(0, |last| last.get_id() + 1);
                        let mut x = SVAShell::new(id, "Vm".to_string());
                        self.vms.push(x);
                    }
                })
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.separator();

  
            for index in 0..self.vms.len() {
                let vm = &mut self.vms[index];
                vm.show(ctx, ui);
            }
            
           // self.vm_shell.show(ctx, ui);

            ////

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
