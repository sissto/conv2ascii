use anyhow::Error;
use clap::StructOpt;
use conv2ascii::arguments::Args;
use conv2ascii::convert::ASCIIConverter;
use conv2ascii::export::{ClipboardExporter, Exporter, FileExporter};
use conv2ascii::import::ClipboardImporter;

const DEFAULT_CHARSET: [char; 10] = [' ', '.', ':', ';', '-', '+', 'o', 'O', '&', '@'];

fn main() {
    let args = Args::parse();

    let charset = DEFAULT_CHARSET.to_vec();
    let converter = ASCIIConverter::new(charset, !args.raw);
    let conv_result: Option<Result<Vec<Vec<char>>, Error>>;

    if args.path.is_some() {
        let path = args.path.as_deref().unwrap();
        conv_result = Some(converter.convert_from_file(path));
    } else {
        let importer = ClipboardImporter;
        let data = importer.import();
        conv_result = Some(converter.convert_from_data(data));
    }

    let result = conv_result.unwrap();
    if result.is_ok() {
        let chars = result.ok().unwrap();

        let exporter = get_exporter(&args);
        exporter.export(chars);

        println!("Image conversion completed.");
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
