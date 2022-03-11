use clap::Parser;

/// Converts an image to ASCII code.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The image input path.
    /// If not specified then image from clipboard is used.
    #[clap(short, long)]
    pub path: Option<String>,

    /// The output path.
    /// If not specified then the output is set to clipboard.
    #[clap(short, long)]
    pub output: Option<String>,

    /// Do not fix image distortion.
    #[clap(short, long)]
    pub raw: bool,
}
