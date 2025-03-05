use arboard::Clipboard;
use eframe::egui;
use rfd::FileDialog;
use std::{fs::OpenOptions, io::Write, sync::{Arc, Mutex}, thread, time::Duration};
use chrono::Local;

struct ClipboardApp {
    file_path: Option<String>,
    clipboard_monitor: Option<thread::JoinHandle<()>>,
}

impl ClipboardApp {
    fn new() -> Self {
        Self { 
            file_path: None,
            clipboard_monitor: None
        }
    }
}

impl eframe::App for ClipboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📋 Clipboard Manager");

            if self.file_path.is_none() {
                if ui.button("Select File to Save Clipboard Data").clicked() {
                    if let Some(path) = FileDialog::new().save_file() {
                        let file_path = path.display().to_string();
                        self.file_path = Some(file_path.clone());

                        // Start monitoring clipboard in the background
                        let file_path_clone = file_path.clone();
                        self.clipboard_monitor = Some(thread::spawn(move || {
                            monitor_clipboard(file_path_clone);
                        }));
                    }
                }
            } else {
                ui.label(format!("📁 Saving to: {}", self.file_path.as_ref().unwrap()));
            }

            // Close Button to Exit App
            if ui.button("❌ Close Application").clicked() {
                std::process::exit(0);
            }
        });
    }
}

fn monitor_clipboard(file_path: String) {
    let clipboard = Arc::new(Mutex::new(Clipboard::new().expect("Failed to access clipboard")));
    let mut last_content = String::new();

    loop {
        if let Ok(mut clip) = clipboard.lock() {
            if let Ok(new_content) = clip.get_text() {
                if new_content != last_content {
                    last_content = new_content.clone();
                    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

                    let mut file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&file_path)
                        .expect("Failed to open file");

                    writeln!(file, "[{}]\n {}\n", timestamp, new_content)
                        .expect("Failed to write to file");
                }
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Clipboard Manager", options, Box::new(|_cc| Box::new(ClipboardApp::new())));
}
