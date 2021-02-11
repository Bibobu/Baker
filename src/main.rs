extern crate image;
use rand::Rng;
use clap;
use image::gif::{GifEncoder};
use std::fs::File;

fn create_image(
    dim:u32,
    israndom:bool,
    ) -> image::Frame
{

    let imgx = dim.clone();
    let imgy = dim.clone();
    let mut imgbuf = image::DynamicImage::new_rgba8(imgx, imgy).to_rgba8();

    let scalex = 1./imgx as f32;
    let scaley = 1./imgy as f32;

    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = 0 as u8;
        let g = 0 as u8;
        let b = 0 as u8;
        *pixel = image::Rgba([r, g, b, 0]);
    }

    for x in 0..imgx {
        for y in 0..imgy {
        let mut r = 0 as u8;
        let mut b = 0 as u8;
        let _px = (x as f32)*scalex;
        let py = (y as f32)*scaley;
        if py >= 0. && py < 0.5 {
            r = 255 as u8;
        } else if py >= 0.5 && py <= 1. {
            b = 255 as u8;
        }
        let pixel = imgbuf.get_pixel_mut(x, y);
        if israndom {
            let mut rng = rand::thread_rng();
            let randr = rng.gen_range(0..255);
            let randg = rng.gen_range(0..255);
            let randb = rng.gen_range(0..255);
            *pixel = image::Rgba([randr, randg, randb, 255]);
            } else {
            *pixel = image::Rgba([r, 0, b, 255]);
        }
        }
    }
    let myframe = image::Frame::new(imgbuf);
    return myframe;
}

fn update_nonfolded_baker(frame: &image::Frame) -> image::Frame {

    let imgbuf = frame.buffer();
    let imgx = imgbuf.width();
    let imgy = imgbuf.height();

    let mut newbuf = image::DynamicImage::new_rgba8(imgx, imgy).to_rgba8();
    
    for y in 0..imgy {
        for x in 0..imgx {
            let pixel = newbuf.get_pixel_mut(y, x);
            let scaled = (2*x/imgx)*imgx;
            // let oldpixel = imgbuf.get_pixel(2*x-scaled, (y+scaled)/2);
            let oldpixel = imgbuf.get_pixel((y+scaled)/2,2*x-scaled);
            let image::Rgba(data) = *oldpixel;
            *pixel = image::Rgba([data[0], data[1], data[2], 255]);
        }
    }
    let myframe = image::Frame::new(newbuf);
    return myframe;
}

fn update_folded_baker(frame: &image::Frame) -> image::Frame {

    let imgbuf = frame.buffer();
    let imgx = imgbuf.width();
    let imgy = imgbuf.height();

    let mut newbuf = image::DynamicImage::new_rgba8(imgx, imgy).to_rgba8();
    
    let scalex = 1./imgx as f32;
    let scaley = 1./imgy as f32;

    for y in 0..imgy {
        for x in 0..imgx {
            let px = (x as f32)*scalex;
            let _py = (y as f32)*scaley;
            let pixel = newbuf.get_pixel_mut(y, x);
            if px < 0.5{
                let oldpixel = imgbuf.get_pixel(y/2, 2*x);
                let image::Rgba(data) = *oldpixel;
                *pixel = image::Rgba([data[0], data[1], data[2], 255]);
            } else {
                let oldpixel = imgbuf.get_pixel((imgy-1)-y/2, 2*(imgx-1)-2*x);
                let image::Rgba(data) = *oldpixel;
                *pixel = image::Rgba([data[0], data[1], data[2], 255]);
            }
        }
    }
    let myframe = image::Frame::new(newbuf);
    return myframe;
}

fn main() {
    let matches = clap::App::new("Baker's map.")
        .version("0.1")
        .author("Bibobu (germain.clavier@gmail.com)")
        .about("Makes a baker's map gif.")
        .arg(clap::Arg::with_name("output")
             .short("o")
             .long("output")
             .takes_value(true)
             .help("The output file"))
        .arg(clap::Arg::with_name("input")
             .short("i")
             .long("input")
             .takes_value(true)
             .help("The input image if any"))
        .arg(clap::Arg::with_name("dim")
             .short("d")
             .long("dimension")
             .takes_value(true)
             .help("Output dimension, the output is systematically resized to a square"))
        .arg(clap::Arg::with_name("steps")
             .short("n")
             .long("nsteps")
             .takes_value(true)
             .help("Number of frames"))
        .arg(clap::Arg::with_name("random")
             .short("r")
             .long("random")
             .help("Is image randomly generated"))
        .arg(clap::Arg::with_name("verbose")
             .short("v")
             .long("verbose")
             .help("Prints some more info"))
        .arg(clap::Arg::with_name("folded")
             .short("f")
             .long("folded")
             .help("Use folded version of the baker's map."))
        .get_matches();

    let verbose = matches.is_present("verbose");

    let outfile = matches.value_of("output").unwrap_or("Baker.gif");

    if verbose {
        println!("Your output file will be {}", outfile);
    }

    let step_str = matches.value_of("steps");
    let nsteps = match step_str {
        None => 3,
        Some(s) => {
            match s.parse::<usize>() {
                Ok(n) => n,
                Err(_) => 3,
            }
        }
    };

    if verbose {
        println!("Your number of frames is {}", nsteps);
    }

    let dim_str = matches.value_of("dim");
    let dim = match dim_str {
        None => 100,
        Some(s) => {
            match s.parse::<u32>() {
                Ok(n) => n,
                Err(_) => 100,
            }
        }
    };

    if verbose {
        println!("The dimension of your image is {}", dim);
    }

    let isinput = matches.is_present("input");

    let frame = match isinput {
        true => {
            let infile = match matches.value_of("input") {
                None => "toto.png",
                Some(s) => s,
            };
            if verbose {
                println!("Using input {}", infile);
            }
            let image = image::io::Reader::open(infile).unwrap().decode().unwrap();
            image::Frame::new(image.resize_exact(dim, dim, image::imageops::FilterType::Lanczos3).to_rgba8())
        },
        false => {
            println!("Creating new image.");

            let israndom = matches.is_present("random");
            if israndom {
                println!("Your input will be random");
            }

            create_image(dim, israndom)
        },
    };

    let isfolded = matches.is_present("folded");

    if isfolded && verbose {
        println!("Using folded map.");
    }

    let isfolded = matches.is_present("verbose");

    let file_out = File::create(outfile).unwrap();
    let mut encoder = GifEncoder::new_with_speed(file_out, 10);

    let mut frames = vec![frame; nsteps];
    for i in 1..nsteps {
        if verbose {
            println!("Step {}", i);
        }
        frames[i] = match isfolded {
            true => update_folded_baker(&frames[i-1]),
            false => update_nonfolded_baker(&frames[i-1]),
        }
    }
    encoder.encode_frames(frames.into_iter()).unwrap();
}
