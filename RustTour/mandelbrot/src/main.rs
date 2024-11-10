use std::env;
use std::usize;
use std::{fs::File, str::FromStr};

use image::codecs::png::PngEncoder;
use image::GrayImage;
use image::ImageEncoder;
use image::{Rgb, RgbImage};
use num::Complex;

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    uper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (lower_right.re - uper_left.re, uper_left.im - lower_right.im);
    Complex {
        re: uper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: uper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    uper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), uper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    let write_img = encoder.write_image(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        image::ExtendedColorType::L8,
    );
    match write_img {
        Ok(_) => Ok(()),
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::WriteZero, e)),
    }
}

#[allow(dead_code)]
fn create_plus_image() {
    let width = 100;
    let height = 100;

    let mut img: GrayImage = GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            if x == width / 2 || y == height / 2 {
                img.put_pixel(x, y, image::Luma([0]));
            } else {
                img.put_pixel(x, y, image::Luma([255]));
            }
        }
    }

    img.save("output.png").expect("Gagal menyimpan gambar");

    println!("Gambar berhasil dibuat: output.png");
}

#[allow(dead_code)]
fn cartesian_plane() {
    let width = 400;
    let height = 400;

    let mut img: RgbImage = RgbImage::from_pixel(width, height, Rgb([255, 255, 255]));

    let axis_color = Rgb([0, 0, 0]);

    for x in 0..width {
        img.put_pixel(x, height / 2, axis_color);
    }

    for y in 0..height {
        img.put_pixel(width / 2, y, axis_color);
    }

    img.save("cartesian_plane.png")
        .expect("Gagal menyimpan gambar");

    println!("Gambar bidang kartesius berhasil dibuat: cartesian_plane.png");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds: (usize, usize) = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    let thread = 8;
    let rows_per_band = bounds.1 / thread + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
        .unwrap()
    }
    write_image(&args[1], &pixels, bounds).expect("error writing image");
}
