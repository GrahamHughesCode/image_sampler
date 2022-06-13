use std::process;
use std::env;
use image_sampler::{Setting, run};

fn main() {

    let args: Vec<String> = env::args().collect();
    let settings = Setting::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if settings.help {
        help()
    }

    let output = run(&settings).unwrap_or_else(|err| {
        println!("Problem runinng: {}", err);
        process::exit(1);
    });

    println!("{}", output);
}

fn help()
{
    println!("Usage: image_sampler [--help] [file] [small] [medium] [large] [(number)x(number)]");
    println!("Previews images using true color.");
    process::exit(1);
}