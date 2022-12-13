use std::{fs::File, io::BufWriter, path::Path};

use rand::Rng;
use ray_tracing_in_one_weekend::camera::Camera;
use ray_tracing_in_one_weekend::{hit::*, ray::*, sphere::*, vec::*};

fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    // let t = hit_sphere(Point3::new(0., 0., -1.), 0.5, r);
    // if t > 0. {
    //     let n = (r.at(t) - Vec3::new(0., 0., -1.)).normalized();
    //     0.5 * Color::new(n.x() + 1., n.y() + 1., n.z() + 1.)
    // } else {
    if depth == 0 {
        return Color::new(0., 0., 0.);
    }
    if let Some(rec) = world.hit(r, 0., f64::INFINITY) {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        let r = Ray::new(rec.p, target - rec.p);
        // 0.5 * (rec.normal + Color::new(1., 1., 1.))
        0.5 * ray_color(&r, world, depth - 1)
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    // let a = r.direction().dot(r.direction());
    let a = r.direction().length().powi(2);
    let b = 2. * r.direction().dot(oc);
    let harf_b = oc.dot(r.direction());
    // let c = oc.dot(oc) - radius.powf(2.);
    let c = oc.length().powi(2) - radius * radius;
    // let discriminant = b.powf(2.) - 4. * a * c;
    let discriminant = harf_b * harf_b - a * c;
    if discriminant < 0. {
        -1.
    } else {
        // (-b - discriminant.sqrt()) / (2. * a)
        (-harf_b - discriminant.sqrt()) / a
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 256;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100u64;
    let MAX_DEPTH = 5;

    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut data = vec![];
    let cam = Camera::new();
    let mut rng = rand::thread_rng();

    for j in (0..image_height).rev() {
        if j % 10 == 0 {
            eprintln!("Scanlines remaining: {}", j);
        }
        for i in 0..image_width {
            let mut pixel_color = Color::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();
                let u = ((i as f64) + random_u) / (image_width as f64 - 1.0);
                let v = ((j as f64) + random_v) / (image_height as f64 - 1.0);

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            data.append(&mut pixel_color.print_sample_png(samples_per_pixel));
        }
    }
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, image_width, image_height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let _ = writer.write_image_data(&data);
    eprintln!("Done")
}
