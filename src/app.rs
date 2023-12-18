use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::{fmt::Display, fs::File};

use egui::{Button, Color32, Label, Stroke};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};

use simple_virtual_assembler::assembler::parsing_err::ParsingError;
use simple_virtual_assembler::components::connection;
use simple_virtual_assembler::vm::instruction::Instruction;
use simple_virtual_assembler::vm::virtual_machine::VirtualMachine;
use simple_virtual_assembler::{
    assembler::assembler::Assembler, components::connection::Connection,
};

use simple_virtual_assembler::language::Language;

use crate::help_window::HelpWindow;
use crate::sva_shell::SVAShell;

use serde::{Deserialize, Serialize};

use serde_json;

use wasm_bindgen::prelude::*;
use web_sys::{js_sys::Array, *};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SvaUI {
    language: Language,
    //vm_shell: SVAShell,
    #[serde(skip)]
    vms: Vec<SVAShell>,
    connections: Rc<RefCell<Vec<Connection>>>,
    connection_started: Rc<RefCell<bool>>,
    disconnect_mode: Rc<RefCell<bool>>,
    ui_size: f32,
    help_widow: HelpWindow,

    port_connections_color_palle: [Color32; 7],
    current_port_connection_color_index: usize,
}

impl Default for SvaUI {
    fn default() -> Self {
        Self {
            // Example stuff:
            // vm_shell: SVAShell::new(0, "First VM window".to_string()),
            language: Language::En,
            vms: Vec::new(),
            connections: Rc::new(RefCell::new(Vec::new())),
            connection_started: Rc::new(RefCell::new(false)),
            ui_size: 1.0,
            help_widow: HelpWindow { is_open: false },
            disconnect_mode: Rc::new(RefCell::new(false)),
            port_connections_color_palle: initialize_colors(),
            current_port_connection_color_index: 0,
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

    pub fn set_language(&mut self, language: Language) {
        //TODO:
        //self.vms.iter_mut().for_each(|vm| vm.set_language(language));

        match language {
            Language::Pl => self
                .vms
                .iter_mut()
                .for_each(|vm| vm.set_language(Language::Pl)),
            Language::En => self
                .vms
                .iter_mut()
                .for_each(|vm| vm.set_language(Language::En)),
        }
    }

    fn switch_port_connection_color(&mut self) {
        if self.current_port_connection_color_index < 6 {
            self.current_port_connection_color_index += 1;
        } else {
            self.current_port_connection_color_index = 0;
        }
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

        ctx.set_pixels_per_point(self.ui_size);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::ScrollArea::horizontal().show(ui, |ui| {
                egui::menu::bar(ui, |ui| {
                    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                    {
                        ui.menu_button("File", |ui| {
                            if ui.button("import").clicked() {}

                            if ui.button("export").clicked() {
                                ui.label("pressed");
                                let serialized_state = serde_json::to_string(&self);

                                match serialized_state {
                                    Ok(data) => ui.label(data),

                                    Err(err) => ui.label(err.to_string()),
                                };
                                //let mut file = File::create("state.json").unwrap();
                            }

                            if ui.button("Quit").clicked() {
                                _frame.close();
                            }
                        });
                        ui.add_space(16.0);
                    }

                    egui::widgets::global_dark_light_mode_switch(ui);

                    ui.separator();
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{:?}", self.language.string_code()))
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_value(&mut self.language, Language::Pl, "Polski")
                                .changed()
                            {
                                self.set_language(Language::Pl);
                            }
                            if ui
                                .selectable_value(&mut self.language, Language::En, "English")
                                .changed()
                            {
                                self.set_language(Language::En);
                            }
                        });
                    ui.separator();
                    ui.add(
                        egui::Slider::new(&mut self.ui_size, 0.75..=3.0)
                            .step_by(0.25)
                            .text("delay"),
                    );
                    ui.separator();

                    ui.separator();
                    ui.heading("Simple virtual assembler ui");
                    ui.separator();

                    ui.menu_button("Add", |ui| {
                        if ui.button("vm").clicked() {
                            let id = self.vms.last().map_or(0, |last| last.get_id() + 1);
                            let mut x = SVAShell::new(
                                id,
                                "Vm".to_string(),
                                self.connection_started.clone(),
                                self.connections.clone(),
                                self.disconnect_mode.clone(),
                                *self
                                    .port_connections_color_palle
                                    .get(self.current_port_connection_color_index)
                                    .unwrap_or(&Color32::BLUE),
                            );
                            self.vms.push(x);
                        }
                    });

                    let mut connection_button_text = "connect";
                    let mut disconnect_button_text = "diconnect";
                    let mut change_current_connection_color = false;

                    if *self.connection_started.borrow_mut() {
                        ctx.set_cursor_icon(egui::CursorIcon::Cell);
                        connection_button_text = "Stop connecting";
                        disconnect_button_text = "disconnect";
                    } else if *self.disconnect_mode.borrow_mut() {
                        ctx.set_cursor_icon(egui::CursorIcon::NotAllowed);
                        disconnect_button_text = "stop disconnecting";
                        connection_button_text = "connect";
                    } else {
                        ctx.set_cursor_icon(egui::CursorIcon::Default);
                        connection_button_text = "connect";
                        disconnect_button_text = "disconnect";
                    }

                    let start_connection_button =
                        Button::new(connection_button_text).stroke(Stroke::new(
                            4.0,
                            self.port_connections_color_palle
                                [self.current_port_connection_color_index],
                        ));

                    if !*self.disconnect_mode.borrow_mut() {
                        if ui.add_enabled(true, start_connection_button).clicked() {
                            let mut conn_started = self.connection_started.borrow_mut();
                            *conn_started = !*conn_started;

                            *self.disconnect_mode.borrow_mut() = false;

                            if *conn_started {
                                let mut conn = Connection::new();
                                self.connections.borrow_mut().push(conn);
                            } else {
                                change_current_connection_color = true;
                            }
                        }
                    }
                    if change_current_connection_color {
                        self.switch_port_connection_color();
                    }

                    // let ttt = Button::new(change_current_connection_color.to_string()).fill(
                    //     self.port_connections_color_palle[self.current_port_connection_color_index],
                    // );

                    //ui.add(ttt);

                    if !*self.connection_started.borrow_mut() {
                        if ui.button(disconnect_button_text).clicked() {
                            let mut dissconnec_mode = self.disconnect_mode.borrow_mut();
                            *dissconnec_mode = !*dissconnec_mode;
                        }
                    }

                    if ui.button("Help").clicked() {
                        self.help_widow.toggle_open_close();
                    }

                    self.help_widow.show(ctx, ui);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.label(self.language.to_string());
            ui.separator();

            for index in 0..self.vms.len() {
                let vm = &mut self.vms[index];
                vm.set_port_connection_color(
                    self.port_connections_color_palle[self.current_port_connection_color_index],
                );
                vm.show(ctx, ui);
            }

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
/// TODO:
/// choose an appriorpet colors
fn initialize_colors() -> [Color32; 7] {
    // [
    //     Color32::RED,
    //     Color32::BLUE,
    //     Color32::GOLD,
    //     Color32::GRAY,
    //     Color32::KHAKI,
    //     Color32::DEBUG_COLOR,
    //     Color32::LIGHT_YELLOW,
    // ]
    //////////////
    //  [
    //     Color32::from_rgb(0, 128, 128),   // Teal
    //     Color32::from_rgb(72, 209, 204),  // Medium Turquoise
    //     Color32::from_rgb(0, 206, 209),   // Dark Turquoise
    //     Color32::from_rgb(102, 205, 170), // Medium Aquamarine
    //     Color32::from_rgb(32, 178, 170),  // Light Sea Green
    //     Color32::from_rgb(95, 158, 160),  // Cadet Blue
    //     Color32::from_rgb(64, 224, 208),  // Turquoise
    // ]
    ////////////////////
    [
        Color32::from_rgb(255, 87, 34),  // Red-Orange
        Color32::from_rgb(63, 81, 181),  // Indigo
        Color32::from_rgb(0, 150, 136),  // Teal
        Color32::from_rgb(255, 193, 7),  // Amber
        Color32::from_rgb(33, 150, 243), // Blue
        Color32::from_rgb(103, 58, 183), // Deep Purple
        Color32::from_rgb(76, 175, 80),  // Green
    ]
}
