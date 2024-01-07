use std::sync::{Mutex, MutexGuard};

static LOGGS: Mutex<Vec<String>> = Mutex::new(Vec::new());

use chrono::{Utc, DateTime};

pub struct CustomLogger {
    events: Mutex<Vec<String>>,
}

impl CustomLogger {
    pub fn new() -> Self {
        CustomLogger {
            events: Mutex::new(Vec::new()),
        }
    }

    pub fn log2(&mut self, event: &str) {
        let current_datetime: DateTime<Utc> = Utc::now();
        let formatted_datetime = current_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        self.events.lock().unwrap().push(formatted_datetime + "|" + &event.to_string());
    }

    pub fn get_logs2(&self) -> MutexGuard<'_, Vec<String>> {
        self.events.lock().unwrap()
    }

    pub fn log( event: &str) {
        let current_datetime: DateTime<Utc> = Utc::now();
        let formatted_datetime = current_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        LOGGS.lock().unwrap().push(formatted_datetime + "|" + &event.to_string());
    }

    pub fn get_logs_c() -> Vec<String> {
        LOGGS.lock().unwrap().clone()
    }

    pub fn clear_logs() {
        LOGGS.lock().unwrap().clear();
    }

}
