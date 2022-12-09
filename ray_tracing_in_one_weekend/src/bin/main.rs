use std::{fs::File, io::BufWriter, path::Path};

use ray_tracing_in_one_weekend::ray::*;
use ray_tracing_in_one_weekend::vec::*;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut data = vec![];

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            // let r = (i as f64) / (image_width as f64 - 1.0);
            // let g = (j as f64) / (image_height as f64 - 1.0);
            // let b = 0.25f64;

            // let ir = (255.999 * r).floor() as i64;
            // let ig = (255.999 * g).floor() as i64;
            // let ib = (255.999 * b).floor() as i64;
            let u = (i as f64) / (image_width as f64 - 1.0);
            let v = (j as f64) / (image_height as f64 - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(&r);
            // pixel_color.write

            // let pixel_color = Color::new(
            //     (i as f64) / (image_width as f64 - 1.0),
            //     (j as f64) / (image_height as f64 - 1.0),
            //     0.25f64,
            // );
            // println!("{} {} {}", ir, ig, ib);
            // eprintln!("{}", pixel_color.print());
            if j % 100 == 0 {
                eprintln!("{:?}", pixel_color.print_png());
            }
            data.append(&mut pixel_color.print_png());
        }
    }
    let path = Path::new(
        r"C:\Users\honda.takumi21\GitHub\ray_tracing_in_one_weak_end\ray_tracing_in_one_weekend\image.png",
    );
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, image_width, image_height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let _ = writer.write_image_data(&data);
    eprintln!("Done")
}

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
