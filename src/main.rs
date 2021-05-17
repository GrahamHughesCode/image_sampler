use colored::*;
use image::*;
use image::imageops::FilterType;
use std::env;
use regex::Regex;

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
            "small" =>
            {
                x_pixels = 50;
                y_pixels = (img.dimensions().1 as f32 * (x_pixels as f32 /img.dimensions().0 as f32)) as u32;
            }
            "medium" => 
            {
                x_pixels = 100;
                y_pixels = (img.dimensions().1 as f32 * (x_pixels as f32 /img.dimensions().0 as f32)) as u32;
            },
            "large" => 
            {
                x_pixels = 200;
                y_pixels = (img.dimensions().1 as f32 * (x_pixels as f32 /img.dimensions().0 as f32)) as u32;
            },
            _ =>
            {
                //checking for manual pixel set [number of x pixels]x[number of y pixels]
                let re = Regex::new(r"([0-9]+x[0-9]+)").unwrap();
                if re.is_match(args[2].as_str())
                {
                   let v: Vec<&str> = args[2].split('x').collect();
                   x_pixels = v[0].parse().unwrap();
                   y_pixels = v[1].parse().unwrap();
                }
                else
                {
                    println!("Not a vaild size. Defaulting to small.");
                }

            }
        } 
        
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
