mod arguments;
mod convert;
mod export;

use arguments::Args;
use clap::StructOpt;
use convert::ASCIIConverter;
use export::Exporter;

const DEFAULT_CHARSET: [char; 10] = [' ', '.', ':', ';', '-', '+', 'o', 'O', '&', '@'];

fn main() {
    let args = Args::parse();
    // let args = Args {
    //     path: "P:/Repos/Lenna.jpg".to_string(),
    //     output: None,
    // };

    let charset = DEFAULT_CHARSET.to_vec();
    let converter = ASCIIConverter::new(charset, args.path);
    let result = converter.convert();
    if result.is_ok() {
        let chars = result.ok().unwrap();
        let exporter = Exporter::new(chars);
        exporter.export();

        print!("Image conversion completed.");
    } else if result.is_err() {
        println!("Image conversion failed: {}", result.err().unwrap());
    }
}
