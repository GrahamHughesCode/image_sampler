use colored::*;
use image::*;
use image::imageops::FilterType;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = args[1].clone();
    let img = image::open(file).unwrap();
    let mut x_pixels:u32 = 100;
    let mut y_pixels:u32 = 100;
    if args.len() >= 3
    {
        match args[2].as_str()
        {
            "small" => x_pixels = 100,
            "medium" => x_pixels = 200,
            "large" => x_pixels = 300,
            _ => println!("error")
        } 
        y_pixels = (img.dimensions().1 as f32 * (x_pixels as f32 /img.dimensions().0 as f32)) as u32;
    }
    println!("x: {} y: {}" ,x_pixels,y_pixels);
    let scaled = img.resize(x_pixels, y_pixels, FilterType::Nearest);
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
