use image;
use num_complex;
use std::io;

fn get_res () -> Result<(usize,usize), io::Error> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    let nums: Vec<usize> = buf.trim().split(" ").map(|v| v.parse::<usize>().unwrap()).collect();

    Ok((nums[0], nums[1]))
}
fn main() {


    let imgx = 800;
    let imgy = 800;

    let scale_x = 3.0 / imgx as f32;
    let scale_y = 3.0 / imgy as f32;

    let mut buf = image::ImageBuffer::new(imgx, imgy);


    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scale_x - 1.5;
            let cy = x as f32 * scale_y - 1.5;

            let mut z = num_complex::Complex::new(0.0, 0.0); 
            let c = num_complex::Complex::new(x as f64*3.0/800.0-1.5, y as f64*3.0/800.0-1.5);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = buf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([i as u8, data[0], i as u8]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    buf.save("fractal.png").unwrap();
}