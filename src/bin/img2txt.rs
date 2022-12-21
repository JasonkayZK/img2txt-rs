use clap::Parser;
use img2txt_rs::{load_image, print_image};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    img_path: String,
}

fn main() {
    let args = Args::parse();
    let img = load_image(&args.img_path).unwrap();
    print_image(img);
}
