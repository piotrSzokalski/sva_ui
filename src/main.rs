#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use backtrace::Backtrace;
use std::fs::File;
use std::io::Write;

use chrono::{DateTime, Utc};

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    std::panic::set_hook(Box::new(|panic_info| {
        // This closure will be called on panic
        let mut message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };

        let backtrace = Backtrace::new();
        let backtrace_str = format!("{:?}", backtrace);

        message = message + "\n" + &backtrace_str;
        // Save the panic message to a file

        let current_datetime: DateTime<Utc> = Utc::now();
        let formatted_datetime = current_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        let painc_path = format!("SVA_panic_log{}.txt", formatted_datetime);
        save_to_file(painc_path, &message);

        // Save logs to file
        //let logs_path = format!("SVA_logs{}.txt", formatted_datetime);

        // save_to_file(logs_path, &CustomLogger::get_logs_c().join("\n"));

        // You can customize this to save the error message to a different file or perform any other action
    }));

    let native_options = eframe::NativeOptions {
        //initial_window_size: Some([400.0, 300.0].into()),
        //min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "sva_ui",
        native_options,
        Box::new(|cc| Box::new(sva_ui::SvaUI::new(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(sva_ui::SvaUI::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}

fn save_to_file(path: String, message: &str) {
    if let Ok(mut file) = File::create(path) {
        if let Err(err) = writeln!(file, "{}", message) {
            eprintln!("Failed to write panic log to file: {}", err);
        }
    } else {
        eprintln!("Failed to create panic log file");
    }
}
