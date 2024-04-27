extern crate image;
extern crate imageproc;

use image::{ImageBuffer, Luma, RgbImage};
use imageproc::filter;
use std::{env, ops};

fn main() {
    // Load Arguments.
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        println!("Usage: {} <input.png> <output.png> <Type> <Iterations> \n", args[0]);
        println!("Type: Sharpen, GaussBlur");
        return;
    }

    // Load the image.
    let img = image::open(&args[1]).unwrap();
    let color = img.to_rgb8();
    let emg = img.to_luma8();

    let mut kernel: [f32;9] = [0.0, -1.0, 0.0,-1.0, 5.0, -1.0,0.0, -1.0, 0.0];
    if &args[3] == "Sharpen" {
        kernel = [0.0, -1.0, 0.0,
                -1.0, 5.0, -1.0,
                0.0, -1.0, 0.0];
    } else if &args[3] == "GaussBlur" {
        kernel = [0.075, 0.124, 0.075, 
                0.124, 0.204, 0.124, 
                0.075, 0.124, 0.075];
    } else if &args[3] == "BoxBlur" {
        kernel = [0.111, 0.111, 0.111, 
                0.111, 0.111, 0.111, 
                0.111, 0.111, 0.111]; 
    } else if &args[3] == "PrewittX" {
        kernel = [-1.0, 0.0, 1.0, 
                -1.0, 0.0, 1.0, 
                -1.0, 0.0, 1.0];
    } else if &args[3] == "PrewittY"{
        kernel = [-1.0, -1.0, -1.0, 
                0.0, 0.0, 0.0, 
                1.0, 1.0, 1.0];
    } else if &args[3] == "SobelX"{
        kernel = [-1.0, 0.0, 1.0, 
                -2.0, 0.0, 2.0, 
                -1.0, 0.0, 1.0];
    } else if &args[3] == "SobelY"{
        kernel = [-1.0, -2.0, -1.0, 
                0.0, 0.0, 0.0, 
                1.0, 2.0, 1.0];
    } else if &args[3] == "Laplacian"{
        kernel = [0.0, 1.0, 0.0, 
                1.0, -4.0, 1.0, 
                0.0, 1.0, 0.0];
    }
    //Red Channel Extraction
    let mut red: ImageBuffer<Luma<u8>,  Vec<u8>> = ImageBuffer::new(emg.width(),emg.height());
    for (x,y,pixel) in red.enumerate_pixels_mut(){
        pixel.0[0] = color.get_pixel(x,y).0[0];
    }

    //Green Channel Extraction
    let mut green: ImageBuffer<Luma<u8>,  Vec<u8>> = ImageBuffer::new(emg.width(),emg.height());
    for (x,y,pixel) in green.enumerate_pixels_mut(){
        pixel.0[0] = color.get_pixel(x,y).0[1];
    }

    //Blue Channel Extraction
    let mut blue: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(emg.width(),emg.height());
    for (x,y,pixel) in blue.enumerate_pixels_mut(){
        pixel.0[0] = color.get_pixel(x,y).0[2];
    }

    //Apply the kernel to each color channel.
    let mut filtered_red = filter::filter3x3::<Luma<u8>, f32, u8>(&red, &kernel);
    let mut filtered_green = filter::filter3x3::<Luma<u8>, f32, u8>(&green, &kernel);
    let mut filtered_blue = filter::filter3x3::<Luma<u8>, f32, u8>(&blue, &kernel);

    let iter: i32 = args[4].parse().unwrap();
    if iter > 1 {
        for _i in (ops::Range {start: 1, end: iter}) {
            filtered_red = filter::filter3x3::<Luma<u8>, f32, u8>(&filtered_red, &kernel);
            filtered_green = filter::filter3x3::<Luma<u8>, f32, u8>(&filtered_green, &kernel);
            filtered_blue = filter::filter3x3::<Luma<u8>, f32, u8>(&filtered_blue, &kernel);
        }
    }

    //Combine color channels
    let mut out: RgbImage = RgbImage::new(emg.width(),emg.height());
    for (x,y,pixel) in out.enumerate_pixels_mut(){
        pixel.0[0] = filtered_red.get_pixel(x,y).0[0];
        pixel.0[1] = filtered_green.get_pixel(x,y).0[0];
        pixel.0[2] = filtered_blue.get_pixel(x,y).0[0];
    }

    // Save the filtered image.
    out.save(&args[2]).unwrap();
}