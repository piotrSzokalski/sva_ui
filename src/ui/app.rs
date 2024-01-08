use std::cell::{Ref, RefCell};
use std::fs;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;
use std::{fmt::Display, fs::File};

use eframe::glow::NONE;
use egui::epaint::tessellator::path;
use egui::{containers::Window, Context};
use egui::{Button, Color32, Label, ScrollArea, Stroke, Ui};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};

use egui_file::FileDialog;
use env_logger::Logger;
use simple_virtual_assembler::assembler::parsing_err::ParsingError;
use simple_virtual_assembler::components::connection;
use simple_virtual_assembler::vm::instruction::Instruction;
use simple_virtual_assembler::vm::virtual_machine::VirtualMachine;
use simple_virtual_assembler::{
    assembler::assembler::Assembler, components::connection::Connection,
};

use egui_notify::{Toast, Toasts};

use simple_virtual_assembler::language::Language;



use serde::{Deserialize, Serialize};

use serde_json;

use wasm_bindgen::prelude::*;
use web_sys::{js_sys::Array, *};

use crate::storage::connections::{ConnectionManager, self, CONNECTION_NAMES};
use crate::storage::custom_logger::CustomLogger;

use super::connection_widget::ConnectionWidget;
use super::help_window::HelpWindow;
use super::sva_window::SVAWindow;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SvaUI {
    language: Language,
    //vm_shell: SVAShell,
    //#[serde(skip)]
    vms: Vec<SVAWindow>,
    connections: Rc<RefCell<Vec<Connection>>>,
    #[serde(skip)]
    connection_started: Rc<RefCell<bool>>,
    #[serde(skip)]
    disconnect_mode: Rc<RefCell<bool>>,
    ui_size: f32,
    help_widow: HelpWindow,
    #[serde(skip)]
    port_connections_color_palle: [Color32; 7],
    current_port_connection_color_index: usize,
    /// Custom logger
    #[serde(skip)]
    logger: CustomLogger,

    debug_mode: bool,

    #[serde(skip)]
    opened_file: Option<PathBuf>,
    #[serde(skip)]
    open_file_dialog: Option<FileDialog>,

    #[serde(skip)]
    save_file_dialog: Option<FileDialog>,
    #[serde(skip)]
    toasts: Toasts,

    connections_copy: Vec<Connection>
}

impl Default for SvaUI {
    fn default() -> Self {
        rust_i18n::set_locale("en");
        Self {
            // Example stuff:
            // vm_shell: SVAShell::new(0, "First VM window".to_string()),
            language: Language::En,
            vms: Vec::new(),
            connections: Rc::new(RefCell::new(Vec::new())),
            connection_started: Rc::new(RefCell::new(false)),
            ui_size: 1.25,
            help_widow: HelpWindow { is_open: false },
            disconnect_mode: Rc::new(RefCell::new(false)),
            port_connections_color_palle: initialize_colors(),
            current_port_connection_color_index: 0,
            logger: CustomLogger::new(),
            debug_mode: false,
            opened_file: None,
            open_file_dialog: None,
            save_file_dialog: None,
            toasts: Toasts::default(),
            connections_copy: Default::default(),
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
        rust_i18n::set_locale("en");
        if let Some(storage) = cc.storage {
            let mut sav_ui: SvaUI = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            sav_ui.set_connections();
            sav_ui.reconnect_ports();
            sav_ui.logger = CustomLogger::new();
            sav_ui.resned_refrences();
            return sav_ui;
        }
        let mut sva_ui: SvaUI = Default::default();
        sva_ui.logger = CustomLogger::new();
        sva_ui
    }

    pub fn set_language(&mut self, language: Language) {
        //TODO:
        //self.vms.iter_mut().for_each(|vm| vm.set_language(language));
        //self.logger.log2("Change language");

        rust_i18n::set_locale(language.string_code());
        CustomLogger::log("Changing language");
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
    fn disconnect_ports(&mut self) {
        for vm in self.vms.iter_mut() {
            for i in 0..4 {
                {
                    vm.vm.lock().unwrap().disconnect(i);
                }
            }
        }
    }

    fn copy_connections(&mut self) {
        self.connections_copy = ConnectionManager::get_connections().lock().unwrap().clone();
        CustomLogger::log("Copying connections");
        CustomLogger::log(&format!("{:?}", self.connections_copy));
        CustomLogger::log("________________________________________");
    }

    fn set_connections(&mut self) {
        ConnectionManager::set_connection(self.connections_copy.clone());
        self.connections_copy.clear();
    }

    fn reconnect_ports(&mut self) {
        //let mut connections = self.connections.borrow_mut();
        let binding = ConnectionManager::get_connections();
        let mut connections = binding.lock().unwrap();
        for conn in connections.iter_mut() {
            let id_pairs = conn.get_connected_vms_and_ports('P');
            for (vm_id, port_index) in id_pairs {
                let x = self.vms.iter().find(|vm| vm.get_id() == vm_id);
                if x.is_some() {
                    {
                        x.unwrap().vm.lock().unwrap().connect(port_index, conn);
                    }
                }
            }
            for vm in self.vms.iter_mut() {}
        }
    }
    fn resned_refrences(&mut self) {
        for vm in self.vms.iter_mut() {
            vm.set_refs(
                self.connection_started.clone(),
                self.connections.clone(),
                self.disconnect_mode.clone(),
            );
        }
    }
    // RENDMEVER TO DIS CONNECT and RECONTECT
    fn export_to_file(&mut self, path: String) {
        self.copy_connections();
        self.disconnect_ports();

        let serialized_state = serde_json::to_string(&self);
        self.set_connections();
        self.reconnect_ports();
        self.resned_refrences();

        match serialized_state {
            Ok(data) => {
                let file = File::create(path).unwrap();
                let mut writer = BufWriter::new(file);
                // Write the data directly, without using serde_json::to_writer
                writer.write_all(data.as_bytes()).unwrap();
                writer.flush().unwrap();
            }
            Err(err) => {
                self.toasts
                .info(t!("error.export.cant_serialize"))
                .set_duration(Some(Duration::from_secs(10)));
            }
        };
    }

    fn import_file(&mut self, path: String) {
        let data = fs::read_to_string(path);
        match data {
            Ok(data) => {
                let json: Result<SvaUI, serde_json::Error> = serde_json::from_str(&data);
                match json {
                    Ok(sva_ui) => {
                        *self = sva_ui;
                        self.set_connections();
                        self.reconnect_ports();
                        self.resned_refrences();
                    }
                    Err(err) => {
                        CustomLogger::log(&format!("{} \n {}", t!("error.import.bad_json"), err));
                        self.toasts
                            .info(t!("error.import.bad_json"))
                            .set_duration(Some(Duration::from_secs(10)));
                    }
                }
            }
            Err(err) => {
                CustomLogger::log(&format!("Could not open file \n {}", err));
                self.toasts
                            .info(t!("error.file.cant_open"))
                            .set_duration(Some(Duration::from_secs(10)));
            }
        }
    }

    /// Shows debug window with logs and global variables
    fn show_debug_window(&mut self, ctx: &Context, ui: &mut Ui) {
        egui::Window::new(t!("window.debug"))
            .open(&mut self.debug_mode)
            .show(ctx, |ui| {
                ui.collapsing("variables", |ui| {
                    ui.label("Connection state");
                    ui.separator();
                    ui.label(format!("{:?}", CONNECTION_NAMES));
                });
                ui.collapsing("logs", |ui| {
                    ScrollArea::vertical().max_height(600.0).show(ui, |ui| {
                        let logs = CustomLogger::get_logs_c();
                        if ui.button(t!("button.clear")).clicked() {
                            CustomLogger::clear_logs();
                        }
                        for log in logs.iter() {
                            ui.separator();
                            ui.label(log);
                        }
                    });
                });
            });
    }
}

impl eframe::App for SvaUI {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.copy_connections();
        self.disconnect_ports();
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {

        //println!("Ports disconnected");
    }

    fn auto_save_interval(&self) -> Duration {
        Duration::MAX
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        ctx.set_pixels_per_point(self.ui_size);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::ScrollArea::horizontal().show(ui, |ui| {
                egui::menu::bar(ui, |ui| {
                    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                    {
                        ui.menu_button(t!("menu.file"), |ui| {
                            if ui.button(t!("menu.file.import")).clicked() {
                                let mut dialog = FileDialog::open_file(self.opened_file.clone());
                                dialog.open();
                                self.open_file_dialog = Some(dialog);
                            }
                            if ui.button(t!("menu.file.export")).clicked() {
                                let mut dialog = FileDialog::save_file(self.opened_file.clone());
                                dialog.open();
                                self.save_file_dialog = Some(dialog);
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
                            .text("ui scale"),
                    );
                    ui.separator();
                    if ui.button(t!("button.clear")).clicked() {
                        self.vms.clear();
                        self.connections = Rc::new(RefCell::new(Vec::new()));
                        ConnectionManager::clear_connections();
                    }
                    ui.separator();

                    ui.menu_button(t!("button.add"), |ui| {
                        if ui.button("vm").clicked() {
                            let id = self.vms.last().map_or(0, |last| last.get_id() + 1);
                            let mut x = SVAWindow::new(
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

                    if ui.button(t!("label.help")).clicked() {
                        self.help_widow.toggle_open_close();
                    }

                    if ui.button("Debug").clicked() {
                        self.debug_mode = !self.debug_mode;
                    }

                    self.help_widow.show(ctx, ui);

                    // debug window
                    self.show_debug_window(ctx, ui);

                    // notifications
                    self.toasts.show(ctx);
                });
            });
        });

        // Central panel

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's


            ui.collapsing("connections", |ui| {
                if ui.button("add").clicked() {
                    ConnectionManager::create_connection();
                }
   
                ui.separator();

                
                let conns = ConnectionManager::get_connections().lock().unwrap().clone();
                for mut c in conns {
                    ConnectionWidget::new(c).show(ctx, ui);
                }
                

            });
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
            // _____________________________________________

            // open file dialog
            if let Some(dialog) = &mut self.open_file_dialog {
                if dialog.show(ctx).selected() {
                    if let Some(file) = dialog.path() {
                        self.opened_file = Some(PathBuf::from(file));
                        CustomLogger::log(&format!("{:?}", self.opened_file));
                        self.import_file(
                            self.opened_file
                                .clone()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_owned(),
                        );
                    }
                }
            }

            // save to file dialog
            if let Some(dialog) = &mut self.save_file_dialog {
                if dialog.show(ctx).selected() {
                    if let Some(file) = dialog.path() {
                        self.opened_file = Some(PathBuf::from(file));
                        CustomLogger::log(&format!("{:?}", self.opened_file));
                        self.export_to_file(
                            self.opened_file
                                .clone()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_owned(),
                        );
                    }
                }
            }
        }); // Central panel
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
