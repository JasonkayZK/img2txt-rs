use clap::Parser;

use img2txt_rs::{load_image, print_image};

const DEFAULT_TARGET_WIDTH: u32 = 120;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    img_path: String,
    #[clap(short, long, value_parser, default_value_t = DEFAULT_TARGET_WIDTH)]
    size: u32,
}

fn main() {
    let args = Args::parse();
    let img = load_image(&args.img_path, args.size).unwrap();
    print_image(img);
}
