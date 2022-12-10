use image;
use image::{RgbImage, Rgb};
use std::fs;

fn main() {
    let contents = fs::read_to_string("encrypted.txt").expect("Test");
    let lines: Vec<Vec<i64>> = contents.split("\n").map(|s: &str| s.trim().to_string().split(" ").map(|s: &str| s.parse::<i64>().unwrap()).collect()).collect();
    let img: Vec<Vec<u8>> = lines.into_iter().map(|l: Vec<i64>|
                                         l.into_iter().map(|i: i64| (i.count_ones()%2) as u8).collect()
                                        ).collect();

    let mut output = RgbImage::new(img.len() as u32, img[0].len() as u32);
    for x in 0..img.len() {
        for y in 0..img[x].len() {
            let x1 = x as u32;
            let y1 = y as u32;
            output.put_pixel(x1, y1, Rgb([img[y][x]*255, 0, 0]))
        }
    }
    output.save("solution.png");

}