use clipboard_win::{formats, get_clipboard, get_clipboard_string};
use std::{fs::File, io::Read, path::Path};

pub struct ClipboardImporter;

impl ClipboardImporter {
    pub fn import(&self) -> Vec<u8> {
        let bitmap_data = get_clipboard(formats::Bitmap);
        if bitmap_data.is_ok() {
            return bitmap_data.unwrap();
        }

        let filelist_data = get_clipboard(formats::FileList);
        if filelist_data.is_ok() {
            let file_paths = filelist_data.unwrap();
            let file_path = Path::new(file_paths[0].as_str());
            if file_path.is_file() {
                let mut file = File::open(file_path).unwrap();
                let mut result: Vec<u8> = Vec::new();
                file.read_to_end(&mut result).unwrap();
                return result;
            }
        }

        let string_data = get_clipboard_string();
        if string_data.is_ok() {
            let url = string_data.as_ref().unwrap();
            if url.starts_with("http://") || url.starts_with("https://") {
                return self.import_from_url(url);
            }
        }
        return Vec::new();
    }

    fn import_from_url(&self, url: &str) -> Vec<u8> {
        let mut result = Vec::new();
        let mut response = reqwest::blocking::get(url).unwrap();
        response.read_to_end(&mut result).unwrap();
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_from_url() {
        let url = "https://www.rust-lang.org/static/images/rust-logo-blk.svg";
        let importer = ClipboardImporter {};
        let result = importer.import_from_url(url);
        assert_eq!(true, result.len() > 0)
    }
}
