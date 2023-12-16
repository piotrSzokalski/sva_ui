use simple_virtual_assembler::components::connection::Connection;

use egui::{containers::Window, widgets::Label, Context};
use egui::{Align, Pos2, Slider, Stroke, TextEdit, Ui, Widget, Color32};

/// Connection used by virtual machines' ports to communicate
#[derive(Clone)]
pub struct ConnectionUI {
    /// Mutex storing shared data
    connection: Connection,
    /// One end
    start_pos: Pos2,
    /// Other end
    end_pos: Pos2,
}

impl ConnectionUI {
    pub fn new() -> ConnectionUI {
        ConnectionUI {
            connection: Connection::new(),
            start_pos: Pos2::new(-1.0, -1.0),
            end_pos: Pos2::new(-1.0, -1.0),
        }
    }
    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.painter().line_segment(
            [self.start_pos, self.end_pos],
            Stroke::new(5.0, Color32::BROWN),
        )
    }
    // pub fn start_connection(&mut self, start_pos: Pos2) -> Self {
    //     self.start_pos = start_pos;
    //     if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
    //         self.end_pos = pointer_pos;
    //     }
        
    //     self
    // }
    pub fn start_connection(&mut self, start_pos: Pos2) -> Self {
        let mut cloned_self = self.clone(); // Assuming ConnectionUI is cloneable
        cloned_self.start_pos = start_pos;
    
        // if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
        //     cloned_self.end_pos = pointer_pos;
        // }
    
        cloned_self
    }
    pub fn try_connect() {

    }
    
}
