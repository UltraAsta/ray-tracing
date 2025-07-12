mod camera;
mod color;
mod common;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::{Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use std::io;
use std::rc::Rc;
use vec3::{Point3, Vec3};

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    // calculating from point to center which is the equation for is (p-c)
    let oc = r.origin() - center;
    let a = vec3::dot(r.direction(), r.direction());
    let half_b = vec3::dot(oc, r.direction());
    let c = vec3::dot(oc, oc) - radius * radius;

    let discriminant = half_b * half_b - a * c;

    // check if it's hit or miss, to only render what's in front of us and not see what is behind
    if discriminant < 0.0 {
        -1.0
    } else {
        // we do the smallest t because we want the first intersection point
        (-half_b - f64::sqrt(discriminant)) / a
    }
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // if we exxeeded the ray bounce limit, no more light is gathered
    // a guard against recursion
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    // check if the ray hits something
    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        let mut attenuation = Color::default();
        let mut scattered = Ray::default();

        if rec
            .mat
            .as_ref()
            .unwrap()
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        // // calculate the bounce direction
        // let direction = rec.normal + Vec3::random_unit_vector();
        // // Create a new ray from the hit point in the bounce direction
        // return 0.5 * ray_color(&Ray::new(rec.p, direction), world, depth - 1);

        return Color::new(0.0, 0.0, 0.0);
    }

    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    // to calculate which way the ray is pointing
    let unit_direction = vec3::unit_vector(r.direction());
    // calcuating the blend factor
    let t = 0.5 * (unit_direction.y() + 1.0);

    // blend the 2 colors
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // image

    // aspect ratio for the virtual viewport and not the final image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    // fixed width for the final image (for now)
    const IMAGE_WIDTH: i32 = 400;
    // calculate the optimal height of the final image by dividing the width by the aspect ratio
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLE_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    let cam = Camera::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 3.0));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    // using rev to make sure the final image is not vertically flipped
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaning: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLE_PER_PIXEL {
                let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);

                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            color::write_color(&mut io::stdout(), pixel_color, SAMPLE_PER_PIXEL);
        }
    }
    eprint!("\nDone.\n");
}
