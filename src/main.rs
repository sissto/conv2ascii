mod arguments;
mod convert;
mod export;

use arguments::Args;
use clap::StructOpt;
use convert::ASCIIConverter;
use export::{ClipboardExporter, Exporter, FileExporter};

const DEFAULT_CHARSET: [char; 10] = [' ', '.', ':', ';', '-', '+', 'o', 'O', '&', '@'];

fn main() {
    let args = Args::parse();

    let charset = DEFAULT_CHARSET.to_vec();
    let converter = ASCIIConverter::new(charset, args.path.to_string(), !args.raw);
    let result = converter.convert();
    if result.is_ok() {
        let chars = result.ok().unwrap();

        let exporter = get_exporter(&args);
        exporter.export(chars);

        print!("Image conversion completed.");
    } else if result.is_err() {
        println!("Image conversion failed: {}", result.err().unwrap());
    }
}

fn get_exporter(args: &Args) -> Box<dyn Exporter> {
    if args.output.is_some() {
        let output_path = args.output.as_ref().unwrap().to_owned();
        return Box::new(FileExporter { path: output_path });
    } else {
        return Box::new(ClipboardExporter {});
    }
}
