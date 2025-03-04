use arboard::Clipboard;
use std::{fs::OpenOptions, io::Write, thread, time::Duration};

fn main() {
    // Create a clipboard instance
    let mut clipboard = Clipboard::new().expect("Failed to access clipboard");

    // Store the last copied text to avoid duplicates
    let mut last_clipboard_content = String::new();

    loop {
        // Get the current clipboard content
        if let Ok(new_content) = clipboard.get_text() {
            if new_content != last_clipboard_content {
                last_clipboard_content = new_content.clone();
                println!("Copied: {}", new_content);

                // Open the file to append the copied text
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("clipboard_history.txt")
                    .expect("Failed to open file");

                // Write the new clipboard content to the file
                writeln!(file, "{}", new_content).expect("Failed to write to file");
            }
        }
        
        // Check the clipboard every second
        thread::sleep(Duration::from_secs(1));
    }
}
