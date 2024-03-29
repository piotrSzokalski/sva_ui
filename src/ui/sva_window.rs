use egui::Button;
use egui::Color32;
use egui::Stroke;
use simple_virtual_assembler::vm::flag::Flag;

use std::panic;

use std::sync::Arc;
use std::sync::Mutex;

use std::thread::JoinHandle;
use std::time::Duration;

use egui::Context;
use egui::Ui;

use simple_virtual_assembler::assembler::parsing_err::ParsingError;

use simple_virtual_assembler::components::port::Port;
use simple_virtual_assembler::vm::instruction::Instruction;
use simple_virtual_assembler::vm::virtual_machine::{VirtualMachine, VmStatus};

use egui_code_editor::{CodeEditor, ColorTheme};

use simple_virtual_assembler::assembler::assembler::Assembler;

use simple_virtual_assembler::language::Language;

use crate::storage::connections_manager::ConnectionManager;
use crate::storage::connections_manager::CONNECTIONS;
use crate::storage::custom_logger::CustomLogger;
use crate::storage::toasts::ToastsManager;

use super::indicator_widget::IndicatorWidget;
use super::syntax::sva_syntax;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SVAWindow {
    /// Id
    id: usize,
    /// Tile
    name: String,

    /// Simple virtual machine
    pub vm: Arc<Mutex<VirtualMachine>>,
    /// Assembler for simple virtual machine
    assembler: Assembler,
    /// Code before assembly
    code: String,
    /// ( Currently useless ) Program to be executed by vm
    program: Vec<Instruction>,
    /// Error assembling code to program
    parsing_error: Option<ParsingError>,
    /// ( Currently useless ) Parsing error message
    parsing_error_msg: String,
    /// Language
    language: Language,
    /// 'Start' or 'Stop' text for button
    control_button_text: String,

    //  --------------------------------------RCs--------------------------------------
    /// Connecting in progress

    /// Connections

    /// Disconnect port on clikc

    //  -------------------------------------------------------------------------------
    ///Delay ms
    delay_ms: u64,

    vm_state: (i32, usize, Flag, [i32; 4], [i32; 6], VmStatus, u32),

    #[serde(skip)]
    indicators: [IndicatorWidget; 13],

    conn_ids: [Option<usize>; 4],

    stack_present: bool,

    stack_data: Vec<i32>,

    max_hight: f32,
    /// determine if window is open, closed windows get deleted
    active: bool,

    ports_collapsed: bool,

    #[serde(skip)]
    vm_join_handle: Option<JoinHandle<()>>,

    vm_status: VmStatus,
}

impl Default for SVAWindow {
    fn default() -> Self {
        Self {
            id: 0,
            name: format!("{}", 0).to_owned(),
            vm: Arc::new(Mutex::new(VirtualMachine::new())),
            assembler: Assembler::new(),
            code: String::new(),
            program: Vec::new(),

            parsing_error_msg: String::new(),
            parsing_error: None,
            language: Language::En,
            control_button_text: "Start".to_owned(),

            delay_ms: 1000,

            vm_state: (0, 0, Flag::EQUAL, [0; 4], [0; 6], VmStatus::Initial, 0),

            indicators: [
                IndicatorWidget::new("acc".to_owned()),
                IndicatorWidget::new("pc".to_owned()),
                IndicatorWidget::new("flag".to_owned()),
                IndicatorWidget::new("r0".to_owned()),
                IndicatorWidget::new("r1".to_owned()),
                IndicatorWidget::new("r2".to_owned()),
                IndicatorWidget::new("r3".to_owned()),
                IndicatorWidget::new("p0".to_owned()),
                IndicatorWidget::new("p1".to_owned()),
                IndicatorWidget::new("p2".to_owned()),
                IndicatorWidget::new("p3".to_owned()),
                IndicatorWidget::new("status".to_owned()),
                IndicatorWidget::new(t!("sva_shell.indicator.delay").to_owned()),
            ],
            conn_ids: [None; 4],
            stack_present: false,
            stack_data: Vec::new(),
            max_hight: 1000.0,
            active: true,
            ports_collapsed: false,
            vm_join_handle: None,
            vm_status: VmStatus::Initial,
        }
    }
}

impl SVAWindow {
    pub fn new(id: usize, stack_present: bool, max_hight: f32) -> SVAWindow {
        let mut s = SVAWindow {
            id,
            name: format!("vm:{}", id),
            vm: Arc::new(Mutex::new(VirtualMachine::new())),
            assembler: Assembler::new(),
            code: String::new(),
            program: Vec::new(),

            parsing_error_msg: String::new(),
            parsing_error: None,
            language: Language::En,
            control_button_text: "Start".to_owned(),

            delay_ms: 1000,

            vm_state: (0, 0, Flag::EQUAL, [0; 4], [0; 6], VmStatus::Initial, 0),

            indicators: Default::default(),
            conn_ids: [None; 4],
            stack_present,
            stack_data: Vec::new(),
            max_hight,
            active: true,
            ports_collapsed: false,
            vm_join_handle: None,
            vm_status: VmStatus::Initial,
        };
        if stack_present {
            s.assembler = Assembler::new().with_stack();
            s.vm = Arc::new(Mutex::new(VirtualMachine::new().with_stack(32)))
        }
        s.vm.lock().unwrap().set_delay(1000);
        s
    }

    pub fn handle_poison_error(&mut self) {
        ToastsManager::show_info(format!("{} {} ", t!("error.restarting_vm"), self.id), 10);
        if self.stack_present {
            self.vm = Arc::new(Mutex::new(VirtualMachine::new()));
            self.try_assemble_and_load();
        } else {
            self.vm = Arc::new(Mutex::new(VirtualMachine::new().with_stack(32)));
            self.try_assemble_and_load();
        }
    }

    pub fn reset_vm(&mut self) {
        let mut poison_err = false;
        {
            let vm_lock = self.vm.lock();
            match vm_lock {
                Ok(mut vm) => vm.clear_registers(),
                Err(_err) => poison_err = true,
            }
        }
        if poison_err {
            self.handle_poison_error();
        }
    }

    pub fn join_vm(&mut self) {
        if let Some(join_handle) = self.vm_join_handle.take() {
            join_handle.join().unwrap();
        }
    }

    pub fn get_status(&self) -> VmStatus {
        self.vm_status
    }

    pub fn halt_vm(&mut self) {
        VirtualMachine::halt(self.vm.clone());
    }

    pub fn stop_vm(&mut self) {
        VirtualMachine::stop(self.vm.clone());
    }

    pub fn resume_vm(&mut self) {
        if self.vm_join_handle.is_some() {
            VirtualMachine::resume(self.vm.clone());
        } else {
            self.vm_join_handle = Some(VirtualMachine::start(self.vm.clone()));
        }
    }

    pub fn has_stack(&self) -> bool {
        self.stack_present
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_max_height(&mut self, height: f32) {
        self.max_hight = height;
    }

    pub fn set_language(&mut self, language: Language) {
        self.assembler.set_language(language);
    }

    pub fn show_stack(&mut self, _ctx: &Context, ui: &mut Ui) {
        if !self.stack_present {
            return;
        }
        ui.collapsing(t!("sva_shell.collapsing_stack"), |ui| {
            egui::ScrollArea::horizontal()
                .max_height(self.max_hight * 0.20)
                .enable_scrolling(true)
                .show(ui, |ui| {
                    ui.separator();
                    ui.horizontal(|ui| {
                        for item in self.stack_data.iter().rev() {
                            let _ = ui.button(&item.to_string());
                        }
                    });
                    ui.add_space(10.0);
                });
        });
    }

    fn show_ports(&mut self, ui: &mut Ui) {
        self.ports_collapsed = ui
            .collapsing(t!("sva_shell.collapsing_ports"), |ui| {
                let mut poison_error = false;
                ui.vertical(|ui| {
                    let mut ports = [
                        Port::new(0),
                        Port::new(0),
                        Port::new(0),
                        Port::new(0),
                        Port::new(0),
                        Port::new(0),
                    ];

                    {
                        match self.vm.lock() {
                            Ok(vm) => {
                                ports = vm.get_ports();
                            }
                            Err(_err) => {
                                poison_error = true;
                            }
                        }
                    }
                    if poison_error {
                        self.handle_poison_error();
                    }
                    let mut index = 0;
                    for mut p in ports {
                        let port_is_connected = match p.clone() {
                            Port::Connected(_, _) => true,
                            Port::Disconnected(_) => false,
                        };

                        let mut port_color = Color32::GRAY;

                        if ConnectionManager::get_current_id_index().is_some() && !port_is_connected
                        {
                            let in_dark_mode = ui.style().visuals.dark_mode;

                            port_color = if in_dark_mode {
                                Color32::YELLOW
                            } else {
                                Color32::BLUE
                            }
                        } else if ConnectionManager::in_disconnect_mode() && port_is_connected {
                            port_color = Color32::DARK_RED;
                        }

                        let port_button =
                            Button::new(format!("{}", p)).stroke(Stroke::new(1.0, port_color));

                        ui.horizontal(|ui| {
                            ui.label(format!("p:{}", index));
                            if ui.add_enabled(true, port_button).clicked() {
                                if let Some(conn_index) = ConnectionManager::get_current_id_index()
                                {
                                    if let Some(conn) = ConnectionManager::get_connections()
                                        .lock()
                                        .unwrap()
                                        .get_mut(conn_index)
                                    {
                                        if !port_is_connected {
                                            let id = self.id.to_string() + "P" + &index.to_string();

                                            {
                                                let lock = self.vm.lock();
                                                match lock {
                                                    Ok(mut vm) => {
                                                        vm.connect_with_id(index, conn, id)
                                                    }
                                                    Err(_err) => poison_error = true,
                                                }
                                            }
                                            if poison_error {
                                                self.handle_poison_error();
                                            }
                                        } else {
                                            ToastsManager::show_info(
                                                t!("toast_info.can_connect_connected_port"),
                                                10,
                                            )
                                        }
                                    }
                                } else if ConnectionManager::in_disconnect_mode() {
                                    let conn_id = p.get_conn_id();
                                    let conn_index =
                                        ConnectionManager::get_connection_index_by_id(conn_id);
                                    if let Some(conn_i) = conn_index {
                                        let mut conns_lock = CONNECTIONS.lock().unwrap();
                                        let conn = conns_lock.get_mut(conn_i);
                                        if let Some(conn_ref) = conn {
                                            {
                                                let lock = self.vm.lock();
                                                match lock {
                                                    Ok(mut vm) => vm.disconnect(index),
                                                    Err(_err) => poison_error = true,
                                                }
                                                let p_id =
                                                    self.id.to_string() + "P" + &index.to_string();
                                                CustomLogger::log(&p_id);
                                                conn_ref.remove_port_id(p_id);
                                            }
                                            if poison_error {
                                                self.handle_poison_error();
                                            }
                                        }
                                    }
                                }
                            }

                            if index < 5 {
                                index += 1;
                            }

                            if let Some(id) = p.get_conn_id() {
                                if let Some(conn_name) = ConnectionManager::get_name(id) {
                                    ui.label(conn_name);
                                }
                            }
                        });
                    }
                });
            })
            .fully_open();
    }

    fn show_registers(
        &mut self,
        ui: &mut Ui,
        acc: i32,
        ctx: &Context,
        pc: usize,
        flag: Flag,
        r: [i32; 4],
    ) {
        let _labels = ["acc", "pc", "flag", "r:0-3", "p:0-3", "status", "delay"];

        ui.horizontal(|ui| {
            self.indicators[0].set(acc, "acc").show(ctx, ui);
            self.indicators[1]
                .set(pc.try_into().unwrap_or(0), "pc")
                .show(ctx, ui);

            // flag
            ui.label("flag");
            let _ = ui.button(flag.to_string());

            self.indicators[2].set(r[0], "r0").show(ctx, ui);
            self.indicators[2].set(r[1], "r1").show(ctx, ui);
            self.indicators[2].set(r[2], "r2").show(ctx, ui);
            self.indicators[2].set(r[3], "r3").show(ctx, ui);
        });
    }

    fn show_code_editor(&mut self, ui: &mut Ui) {
        let in_dark_mode = ui.style().visuals.dark_mode;
        let editor_them = if in_dark_mode {
            ColorTheme::GITHUB_DARK
        } else {
            ColorTheme::GITHUB_LIGHT
        };
        let id = ui.make_persistent_id("Vm code heder");
        egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false)
            .show_header(ui, |ui| {
                ui.label(t!("sva_shell.code_block"));
            })
            .body(|ui| {
                let max_height = if self.stack_present {
                    self.max_hight * 0.30 * (1.0 + !self.ports_collapsed as i32 as f32)
                } else {
                    self.max_hight * 0.4 * (1.0 + !self.ports_collapsed as i32 as f32)
                };
                egui::ScrollArea::neither()
                    .max_height(max_height)
                    .show(ui, |ui| {
                        let code_editor = CodeEditor::default()
                            .id_source("code editor")
                            .with_rows(12)
                            .with_fontsize(14.0)
                            .with_theme(editor_them)
                            .with_syntax(sva_syntax())
                            .with_numlines(true)
                            .show(ui, &mut self.code);
                        if code_editor.response.changed() {
                            self.try_assemble_and_load();
                        }
                    });
            });
    }

    fn show_vm_control_buttons(&mut self, ui: &mut Ui, vm_status: VmStatus) {
        let next_instruction = self.vm.lock().unwrap().get_next_instruction();

        if let Some(instruction) = next_instruction {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(t!("sva_shell.next_instruction") + ":");
                ui.label(
                    egui::RichText::new(instruction.to_string())
                        .color(egui::Color32::from_rgb(255, 100, 000)),
                );
            });
        }

        let mut poison_err = false;
        ui.separator();

        if let Some(parsing_error) = &self.parsing_error {
            ui.label(
                egui::RichText::new(parsing_error.to_string())
                    .color(egui::Color32::from_rgb(255, 0, 0)),
            );
        } else {
            ui.horizontal(|ui| {
                if vm_status == VmStatus::Running {
                    self.control_button_text = t!("sva_shell.button.stop").to_owned();
                }

                match vm_status {
                    VmStatus::Initial => {
                        self.control_button_text = t!("sva_shell.button.start").to_owned()
                    }
                    VmStatus::Running => {
                        self.control_button_text = t!("sva_shell.button.stop").to_owned()
                    }
                    VmStatus::Stopped => {
                        self.control_button_text = t!("sva_shell.button.resume").to_owned()
                    }
                    VmStatus::Finished => {
                        self.control_button_text = t!("sva_shell.button.start").to_owned()
                    }
                }

                if (vm_status == VmStatus::Running || vm_status == VmStatus::Stopped)
                    && ui.button(t!("sva_shell.button.halt")).clicked()
                {
                    VirtualMachine::halt(self.vm.clone());
                }

                if ui.button(&self.control_button_text).clicked() {
                    {
                        {
                            let vm_lock = self.vm.lock();
                            match vm_lock {
                                Ok(mut vm) => {
                                    vm.set_delay(self.delay_ms.try_into().unwrap());
                                }
                                Err(_) => poison_err = true,
                            }
                        }
                        if poison_err {
                            self.handle_poison_error();
                        }
                    }
                    match vm_status {
                        VmStatus::Initial => {
                            self.vm_join_handle = Some(VirtualMachine::start(self.vm.clone()));
                        }
                        VmStatus::Running => {
                            VirtualMachine::stop(self.vm.clone());
                        }
                        VmStatus::Stopped => {
                            if self.vm_join_handle.is_some() {
                                VirtualMachine::resume(self.vm.clone());
                            } else {
                                self.vm_join_handle = Some(VirtualMachine::start(self.vm.clone()));
                            }
                        }
                        VmStatus::Finished => {
                            VirtualMachine::start(self.vm.clone());
                            //self.reset_vm();
                            //VirtualMachine::resume(self.vm.clone());
                        }
                    }
                }

                if ui.button(t!("sva_shell.button.step")).clicked() {
                    self.step();
                }
                if ui.button(t!("sva_shell.button.reset")).clicked() {
                    self.reset_vm();
                }
                ui.separator();
                if ui
                    .add(
                        egui::Slider::new(&mut self.delay_ms, 0..=5000)
                            .logarithmic(true)
                            .text(t!("sva_shell.indicator.delay")),
                    )
                    .changed()
                {
                    self.vm
                        .lock()
                        .unwrap()
                        .set_delay(self.delay_ms.try_into().unwrap());
                }
            });
        }
        ui.separator();
    }

    /// Tries Assembles code to instructions and loads them to vm
    pub fn try_assemble_and_load(&mut self) {
        let res = self.assembler.parse(&self.code);

        match res {
            Ok(program) => {
                {
                    match self.vm.lock() {
                        Ok(mut vm) => {
                            //CustomLogger::log("Loading program");
                            vm.load_program(program);
                        }
                        Err(err) => ToastsManager::show_err(format!("{:?}", err), 10),
                    }
                }
                self.parsing_error = None
            }
            Err(err) => self.parsing_error = Some(err),
        }
    }
    /// Execute one instruction FIXME:
    fn step(&mut self) {
        let mut poison_err = false;
        {
            let vm_lock = self.vm.lock();

            match vm_lock {
                Ok(mut vm) => {
                    if vm.get_pc() >= vm.get_program().len() {
                        vm.clear_registers();
                    }
                }
                Err(_err) => poison_err = true,
            }
        }
        if poison_err {
            self.handle_poison_error();
        }
        panic::catch_unwind(|| {
            self.vm.lock().unwrap().execute();
        })
        .unwrap_or_else(|_err| {
            self.handle_poison_error();
        });
    }

    pub fn show(&mut self, ctx: &Context, _ui: &mut Ui) {
        let mut poison_error = false;
        {
            match self.vm.lock() {
                Ok(vm) => {
                    self.vm_state = vm.get_state_for_display();
                    if self.stack_present {
                        self.stack_data = vm.get_stack();
                    }
                }
                Err(err) => {
                    poison_error = true;

                    ToastsManager::show_err(format!("{}", err), 10);
                }
            }
        }
        if poison_error {
            self.handle_poison_error();
        }
        let (acc, pc, flag, r, _p, vm_status, _delay) = self.vm_state;
        self.vm_status = vm_status;
        // window
        egui::Window::new(&self.name)
            .id(egui::Id::new(format!("vm:{}", self.id)))
            .max_height(self.max_hight)
            .max_width(500.0)
            .show(ctx, |ui| {
                self.show_code_editor(ui);

                self.show_vm_control_buttons(ui, vm_status);

                self.show_registers(ui, acc, ctx, pc, flag, r);

                self.show_ports(ui);

                self.show_stack(ctx, ui);

                // uncomment for debugging
                //ui.label(self.vm.lock().unwrap().to_string());

                if self.delay_ms > 10 {
                    ctx.request_repaint_after(Duration::from_millis(self.delay_ms));
                } else {
                    ctx.request_repaint_after(Duration::from_millis(10));
                }
            });
    }
}
