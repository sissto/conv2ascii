use clipboard_win::{formats, get_clipboard};
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
        return Vec::new();
    }
}
