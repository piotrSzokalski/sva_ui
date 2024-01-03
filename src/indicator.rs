use egui::{Context, Ui, Widget};

use crate::custom_logger::CustomLogger;

#[derive(Debug)]
enum ValueFormat {
    Dec,
    Bin,
    Hex,
    Unicode,
}

impl Default for ValueFormat {
    fn default() -> Self {
        ValueFormat::Dec
    }
}

#[derive(Default)]
pub struct Indicator {
    data: i32,
    format: ValueFormat,
    text: String,
}

impl Indicator {
    pub fn new(data: i32) -> Self {
        Self {
            data,
            format: ValueFormat::Dec,
            text: data.to_string(),
        }
    }
    pub fn set_data(&mut self, data: i32) {
        self.data = data;
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        //CustomLogger::log(&!format!("showing INDICARO with {}", self.data));
        let formatted_value = self.write_in_format();
        if ui.button(formatted_value).clicked() {
            //CustomLogger::log(&format!("{:?}", self.format));
            self.switch_format();
        }
    }

    fn write_in_format(&mut self) -> String {
        match self.format {
            ValueFormat::Dec => format!("{}", self.data),
            ValueFormat::Hex => format!("{:b}", self.data),
            ValueFormat::Bin => format!("{:X}", self.data),
            ValueFormat::Unicode => {
                if self.data < 0 {
                    return "Invalid Char".to_owned();
                } else {
                    if let Some(char) = char::from_u32(self.data as u32) {
                        return format!("\'{}\'", char);
                    }
                    return "Invalid Char".to_owned();
                }
            }
        }
    }

    fn switch_format(&mut self) {
        match self.format {
            ValueFormat::Dec => self.format = ValueFormat::Bin,
            ValueFormat::Bin => self.format = ValueFormat::Hex,
            ValueFormat::Hex => self.format = ValueFormat::Unicode,
            ValueFormat::Unicode => self.format = ValueFormat::Dec,
        }
    }
}

impl Widget for Indicator {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let x = ui.button(self.text);
        if x.clicked() {
            //self.change_format();
        }
        x
    }
}
