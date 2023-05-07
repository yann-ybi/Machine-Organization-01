use csc411_image::{Read, GrayImage};
use std::env;

fn main(){

    let input = env::args().nth(1); // input gets the argument of index 1

    let img = GrayImage::read(input.as_deref()).unwrap(); 

    let denom = img.denominator as f64;
    let mut acc:f64 = 0.0;

    // for every pixel in the image divide that pixel value by the highest pixel value possible and add it to our accumulator acc
    for pixel in img.pixels.iter() {
        acc += pixel.value as f64 / denom;
    }
    let av =  acc  / img.pixels.len() as f64;

    println!("{:.3}", av);
}