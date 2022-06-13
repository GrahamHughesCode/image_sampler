use regex::Regex;
use colored::*;
use image::*;
use image::imageops::FilterType;

pub enum Size{
  Small,
  Medium,
  Large,
  Custom(u32, u32)
}

pub struct Setting
{
  pub help: bool,
  pub size: Size,
  pub file: String,
}

impl Setting {
    pub fn new(args: &[String]) -> Result<Setting, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file = args[1].clone();

        if file == "--help" {
            return Ok(Setting {
              help: true,
              size: Size::Small,
              file: "help".to_string(),
          });
        }

        if std::path::Path::new(&file).exists() == false
        {
            return Err("file does not exist");
        }

        let mut size: Size = Size::Small;

        if args.len() == 3
        {
            let size_str = args[2].clone();
            size = match size_str.to_lowercase().as_str()
            {
                "small" => { Size::Small },
                "medium" => { Size::Medium }
                "large" => { Size::Large }
                _ =>
                {
                    //checking for manual pixel set [number of x pixels]x[number of y pixels]
                    //with preserved value aspect ratio
                    let re = Regex::new(r"([0-9]+x[0-9]+)").unwrap();
                    if re.is_match(size_str.as_str())
                    {
                        let v: Vec<&str> = size_str.split('x').collect();
                        Size::Custom(v[0].parse().unwrap(),
                                      v[1].parse().unwrap())
                    }
                    else
                    {
                        return Err("size not formated right");
                    }
                }
            }
        }

        Ok(Setting {
            help: false,
            size: size,
            file: file,
        })
    }
}

pub fn run(settings: &Setting) ->  Result<String, &'static str> {
    let img = image::open(&settings.file).unwrap();
    let x_pixels:u32;
    let y_pixels:u32;

    match settings.size
    {
        Size::Small =>
        {
            x_pixels = 50;
            y_pixels = (img.dimensions().1 as f32 * (x_pixels as f32 /img.dimensions().0 as f32)) as u32;
        }
        Size::Medium =>
        {
            x_pixels = 100;
            y_pixels = (img.dimensions().1 as f32 * (x_pixels as f32 /img.dimensions().0 as f32)) as u32;
        },
        Size::Large =>
        {
            x_pixels = 200;
            y_pixels = (img.dimensions().1 as f32 * (x_pixels as f32 /img.dimensions().0 as f32)) as u32;
        },
        Size::Custom(x, y) =>
        {
            x_pixels = x;
            y_pixels = y;
        }
    }

    // scaling the image
    let scaled = img.resize(x_pixels, y_pixels, FilterType::Nearest);
    let img = scaled;
    let x_size = img.dimensions().0 - 1;
    
    // outputting the image
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

    Ok(output)
}