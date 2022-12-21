use img2txt_rs::{load_image, print_image};

fn main() {
    let img_path = "./examples/example3.jpg".to_string();
    let img = load_image(&img_path).unwrap();
    print_image(img);
}
