use std::fmt::format;
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::path::PathBuf;
use crate::hittable::Hit_Record;
use crate::ray::Ray;
use crate::vector::{Custom_Color, Custom_Point, Custom_Vector, unit_vector, dot, cross};

mod vector;
mod ray;
mod utility;
mod hittable;
mod hittable_list;
mod sphere;

fn ray_color (r :&Ray, world :&Hittable_List) -> Custom_Color {
    let mut rec = Hit_Record::default();
    if let true = world.Hit(r, 0.0, utility::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Custom_Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction :Custom_Vector = unit_vector(&r.direction());
    let t :f64 = 0.5 * (unit_direction.y() + 1.0);
    let sunset :Custom_Color = Custom_Color::new(247.0/255.9, 147.0/255.9, 27.0/255.9);
    let sky :Custom_Color = Custom_Color::new(0.5, 0.7, 1.0);

    (1.0 - t) * Custom_Color::new(1.0, 1.0, 1.0) + t * sky
}

unsafe fn present (width :u32, ratio :f64) -> Vec<u8> {
    let image_width :u32 = width;
    let image_height :u32 = (width as f64 / ratio) as u32;
    let aspect_ratio :f64 = ratio;

    let view_height :f64 = 2.0;
    let view_width :f64 = view_height * aspect_ratio;
    let focal_length :f64  = 1.0;

    let origin :Custom_Point = Custom_Point::new(0.0, 0.0, 0.0);
    let horizontal :Custom_Vector = Custom_Vector::new(view_width, 0.0, 0.0);
    let vertical :Custom_Vector = Custom_Vector::new(0.0, view_height, 0.0);
    let corner :Custom_Vector = origin - horizontal/2.0 - vertical/2.0 - Custom_Vector::new(0.0, 0.0, focal_length);
    // left down

    let mut output :Vec<u8> = Vec::new();

    for i in (0..image_height).rev() {
        eprintln!("Remaining {}", i);
        for j in 0..image_width {
            let u :f64 = j as f64 / (image_width - 1) as f64;
            let v :f64 = i as f64 / (image_height - 1) as f64;
            let r :Ray = Ray::new(&origin, &(corner + u*horizontal + v*vertical - origin));

            let mut c :Custom_Vector = Custom_Color::new(0.0, 0.0, 0.0);
            let sphere :Custom_Vector = Custom_Point::new(0.0, 0.0, -2.0);
            let t :f64 = hit_sphere(&r, &sphere, 1.0);

            if t >= 0.0 {
                c = sphere_color(&r);
            } else {
                c = ray_color(&r);
            }

            output.push((c[0] * 255.999) as u8);
            // println!("c[0] : {}", c[0]);
            output.push((c[1] * 255.999) as u8);
            // println!("c[1] : {}", c[1]);
            output.push((c[2] * 255.999) as u8);
            // println!("c[2] : {}", c[2]);
        }
    }

    output
}

fn ppm_header(width :u32, height :u32) -> String {
    format!("P3\n{} {}\n255", width, height)
}

fn main() {
    let width :u32 = 1280;
    let ratio :f64 = 16.0 / 9.0;
    let height :u32 = (width as f64 / ratio) as u32;

    let output :Vec<u8> = unsafe { present(width, ratio) };
    let header :String = ppm_header(width, height);

    let mut fp :File = File::create(PathBuf::from("./output.ppm")).unwrap();

    fp.write_all(header.as_bytes()).unwrap();

    for i in 0..output.len() {
        if i % 3 == 0 {
            fp.write_all("\n".to_string().as_bytes()).unwrap();
        }
        fp.write(output[i].to_string().add(" ").as_bytes()).unwrap();
    }

    let v = Custom_Vector::new_from_zero();

}