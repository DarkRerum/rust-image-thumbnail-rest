extern crate image;

use image::GenericImageView;
use image::imageops;
use image::DynamicImage;
use std::io::Write;
use std::fs::File;
use image::FilterType;

fn crop_image(img: &DynamicImage) -> DynamicImage {
    //let img = image::open("cat.jpg").unwrap();

    //println!("dimensions {:?}", img.dimensions());

    //println!("{:?}", img.color());    
    println!("start thumbnail");
    let g = imageops::thumbnail(img, 100, 100);
    println!("finish thumbnail");

    //resized_buffer.save("resize.png");
    //g.save("thumbnail.png");
    //return g;
    let d = img.dimensions();
    let max: u32;
    if d.0 > d.1 {
        max = d.0;
    } else {
        max = d.1;
    }
    println!("{}", max);
    //g
    //return img.resize_to_fill(thumbnail(100, 100);
    //return img.resize(max, max, FilterType::Nearest).thumbnail(100, 100);
    return img.thumbnail_exact(100, 100);
}

fn download_picture(url: &str, mut buf: &mut Vec<u8>) {    
    let mut body = reqwest::get(url).unwrap();
    println!("downloaded");
    let content_type = body.headers().get("content-type").unwrap().to_str().unwrap();
    println!("content-type: {}", content_type);  
    println!("start copy");
    body.copy_to(&mut buf);
    println!("finish copy");
    //let mut f = File::create("test.jpg").unwrap();

    //f.write_all(&buf);
}

fn main() {
    let url = "https://images.pexels.com/photos/617278/pexels-photo-617278.jpeg";
    let mut buf: Vec<u8> = vec![];
    download_picture(url, &mut buf);
    let picture = image::load_from_memory(&buf).unwrap();
    let th = crop_image(&picture);
    th.save("avatar.jpg");

    println!("Hello world!");
}