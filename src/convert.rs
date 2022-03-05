use anyhow::{bail, Result};
use image::{GenericImageView, Pixel};
use std::path::Path;

pub struct ASCIIConverter {
    charset: Vec<char>,
    path: String,
}

impl ASCIIConverter {
    pub fn new(charset: Vec<char>, path: String) -> Self {
        return Self { charset, path };
    }

    pub fn convert(&self) -> Result<Vec<Vec<char>>> {
        self.validate()?;

        let mut result: Vec<Vec<char>> = Vec::new();
        let image = image::open(&self.path)?;
        let dim = image.dimensions();
        for y in 0..dim.1 {
            let mut chars: Vec<char> = Vec::new();
            for x in 0..dim.0 {
                let px = image.get_pixel(x, y);
                let brightness = px.to_luma();

                let char = self.get_char(brightness.0[0]);
                chars.push(char);
            }
            result.push(chars);
        }

        return Ok(result);
    }

    fn get_char(&self, brightness: u8) -> char {
        let index =
            ((brightness as f64 / u8::MAX as f64) * (self.charset.len() - 1) as f64).round();
        return self.charset[index as usize];
    }

    fn validate(&self) -> Result<()> {
        if self.charset.is_empty() {
            bail!("charset is not set");
        }
        if self.path.is_empty() {
            bail!("path is not set");
        }
        let file_path = Path::new(&self.path);
        if !file_path.exists() || !file_path.is_file() {
            bail!("file or path does not exists");
        }
        //TODO: check file extension
        return Ok(());
    }
}
