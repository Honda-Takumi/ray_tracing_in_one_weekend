use std::{fs::File, io::BufWriter, path::Path};

use ray_tracing_in_one_weekend::ray::*;
use ray_tracing_in_one_weekend::vec::*;

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(Point3::new(0., 0., 1.), 0.5, r) {
        return Color::new(1., 0., 0.)
    }
    let unit_direction = r.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let a = r.direction().dot(&r.direction());
    let b = 2. * r.direction().dot(&oc);
    let c = oc.dot(&oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4. * a * c;
    discriminant < 0.
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 284;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

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
            let u = (i as f64) / (image_width as f64 - 1.0);
            let v = (j as f64) / (image_height as f64 - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(&r);
            // if j % 100 == 0 {
            //     eprintln!("{:?}", pixel_color.print_png());
            // }
            data.append(&mut pixel_color.print_png());
        }
    }
    let path = Path::new(
        r"image.png",
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
