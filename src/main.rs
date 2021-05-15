use colored::*;
use image::*;
use image::imageops::FilterType;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = args[1].clone();
    let img = image::open(file).unwrap();
    let scaled = img.resize(200, 200, FilterType::Nearest);
    let img = scaled; 
    let x_size = img.dimensions().0 - 1;
    let mut output = String::new();
    for pixel in img.pixels() {
        let (x, _y, c) = pixel;
        if x == x_size
        {
            output.push_str(&format!("{}\n", "\u{0020}".on_truecolor(c[0], c[1], c[2])));
            // println!("{}\n", "\u{0020}".on_truecolor(c[0], c[1], c[2]));
        }
        else
        {
            output.push_str(&format!("{}", "\u{0020}".on_truecolor(c[0], c[1], c[2])));
            // print!("{}", "\u{0020}".on_truecolor(c[0], c[1], c[2]));
        }
    }
    println!("{}", output);
}
