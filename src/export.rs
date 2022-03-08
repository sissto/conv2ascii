use clipboard::{ClipboardContext, ClipboardProvider};
use std::{fmt::Write, fs::File, io::Write as IOWrite, path::Path};

pub trait Exporter {
    fn export(&self, data: Vec<Vec<char>>);

    fn get_result_text(&self, data: Vec<Vec<char>>) -> String {
        let result_text = data.iter().fold(String::new(), |mut text, row| {
            let row_str: String = row.iter().collect();
            write!(&mut text, "{}\n", row_str).unwrap();
            return text;
        });
        return result_text;
    }
}

pub struct ClipboardExporter;

impl Exporter for ClipboardExporter {
    fn export(&self, data: Vec<Vec<char>>) {
        if data.is_empty() {
            return;
        }

        let result = self.get_result_text(data);

        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(result).unwrap();
    }
}

pub struct FileExporter {
    pub path: String,
}

impl Exporter for FileExporter {
    fn export(&self, data: Vec<Vec<char>>) {
        if data.is_empty() {
            return;
        }

        let file_path = Path::new(&self.path);
        if file_path.is_dir() {
            return;
        }

        let result = self.get_result_text(data);
        let mut output_file = File::create(file_path).unwrap();
        write!(&mut output_file, "{}", result).unwrap();
    }
}
