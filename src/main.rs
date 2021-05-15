use colored::*;
use image::*;
use image::imageops::FilterType;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = args[1].clone();
    let img = image::open(file).unwrap();
    let mut x_pixels:u32 = 50;
    let mut y_pixels:u32 = 50;
    if args.len() >= 3
    {
        match args[2].as_str()
        {
            "small" => x_pixels = 50,
            "medium" => x_pixels = 100,
            "large" => x_pixels = 200,
            _ => println!("Error not a size. Defaulting to small")
        } 
        y_pixels = (img.dimensions().1 as f32 * (x_pixels as f32 /img.dimensions().0 as f32)) as u32;
    }
    let scaled = img.resize(x_pixels, y_pixels, FilterType::Nearest);
    let img = scaled; 
    let x_size = img.dimensions().0 - 1;
    let mut output = String::new();
    for pixel in img.pixels() {
        let (x, _y, c) = pixel;
        if x == x_size
        {
            output.push_str(&format!("{}\n", "\u{0020}".on_truecolor(c[0], c[1], c[2])));
        }
        else
        {
            output.push_str(&format!("{}", "\u{0020}".on_truecolor(c[0], c[1], c[2])));
        }
    }
    println!("{}", output);
}
