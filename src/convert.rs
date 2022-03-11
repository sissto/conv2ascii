use anyhow::{bail, Result};
use image::{DynamicImage, GenericImageView, Pixel};
use std::path::Path;

pub struct ASCIIConverter {
    charset: Vec<char>,
    fix_distortion: bool,
}

impl ASCIIConverter {
    pub fn new(charset: Vec<char>, fix_distortion: bool) -> Self {
        return Self {
            charset,
            fix_distortion,
        };
    }

    pub fn convert_from_file(&self, file_path: &str) -> Result<Vec<Vec<char>>> {
        self.validate_from_file(file_path)?;

        let image = image::open(file_path)?;
        let result = self.convert(image);

        return Ok(result);
    }

    pub fn convert_from_data(&self, data: Vec<u8>) -> Result<Vec<Vec<char>>> {
        self.validate_from_data(&data)?;

        let image = image::load_from_memory(data.as_slice())?;
        let result = self.convert(image);

        return Ok(result);
    }

    fn convert(&self, image: DynamicImage) -> Vec<Vec<char>> {
        let mut result: Vec<Vec<char>> = Vec::new();
        let dim = image.dimensions();
        for y in 0..dim.1 {
            if self.fix_distortion && y % 2 != 0 {
                continue;
            }

            let mut chars: Vec<char> = Vec::new();
            for x in 0..dim.0 {
                let px = image.get_pixel(x, y);
                let brightness = px.to_luma();

                let char = self.get_char(brightness.0[0]);
                chars.push(char);
            }
            result.push(chars);
        }
        return result;
    }

    fn get_char(&self, brightness: u8) -> char {
        let index =
            ((brightness as f64 / u8::MAX as f64) * (self.charset.len() - 1) as f64).round();
        return self.charset[index as usize];
    }

    fn validate_from_file(&self, file_path: &str) -> Result<()> {
        if self.charset.is_empty() {
            bail!("charset is not set");
        }
        if file_path.is_empty() {
            bail!("path is not set");
        }
        let file_path = Path::new(file_path);
        if !file_path.exists() || !file_path.is_file() {
            bail!("file or path does not exists");
        }
        return Ok(());
    }

    fn validate_from_data(&self, data: &Vec<u8>) -> Result<()> {
        if self.charset.is_empty() {
            bail!("charset is not set");
        }
        if data.is_empty() {
            bail!("no data given");
        }
        return Ok(());
    }
}
