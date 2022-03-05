use clipboard::{ClipboardContext, ClipboardProvider};
use std::fmt::Write;

pub struct Exporter {
    result: Vec<Vec<char>>,
}

impl Exporter {
    pub fn new(result: Vec<Vec<char>>) -> Self {
        return Self { result };
    }

    pub fn export(&self) {
        if self.result.is_empty() {
            return;
        }

        let result_text = self.result.iter().fold(String::new(), |mut text, row| {
            let row_str: String = row.iter().collect();
            write!(&mut text, "{}\n", row_str).unwrap();
            return text;
        });

        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(result_text).unwrap();
    }
}
