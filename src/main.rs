mod camera;
mod color;
mod common;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod shapes;
mod vec3;

use std::io;
use std::rc::Rc;

use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use shapes::{Cube, Sphere, Square};
use vec3::Point3;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
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
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    // Ground
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Square::horizontal(
        Point3::new(0.0, 0.0, 0.0),
        1000.0,
        ground_material.clone(),
    )));

    // Sphere
    let sphere_material = Rc::new(Metal::new(Color::new(0.2, 0.7, 0.7), 0.1));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 1.0),
        1.0,
        sphere_material,
    )));

    // Cube
    let cube_material = Rc::new(Metal::new(Color::new(0.2, 0.7, 0.7), 0.1));
    let cube = Cube::new(
        Point3::new(-4.5, 0.0, 0.0),
        Point3::new(-2.5, 2.0, 2.0),
        cube_material,
    );
    world.add(Box::new(cube));

    // Cylinder
    let cylinder_material = Rc::new(Lambertian::new(Color::new(0.8, 1.0, 0.2)));
    let cylinder = crate::shapes::cylinder::Cylinder::new(
        Point3::new(3.5, 0.0, 1.0),
        vec3::Vec3::new(0.0, 1.0, 0.0),
        0.8,
        2.0,
        cylinder_material,
    );
    world.add(Box::new(cylinder));

    world
}

enum SceneType {
    Sphere,
    PlaneCube,
    AllObjects,
    AllObjectsAltCamera,
}

fn scene_sphere() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Square::horizontal(
        Point3::new(0.0, 0.0, 0.0),
        1000.0,
        ground_material,
    )));
    let sphere_material = Rc::new(Metal::new(Color::new(0.8, 0.2, 0.2), 0.1));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        sphere_material,
    )));
    world
}

fn scene_plane_cube() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Rc::new(Lambertian::new(Color::new(0.4, 0.15, 0.05)));
    world.add(Box::new(Square::horizontal(
        Point3::new(0.0, 0.0, 0.0),
        1000.0,
        ground_material,
    )));
    let cube_material = Rc::new(Metal::new(Color::new(0.1, 0.2, 0.2), 0.2)); // dimmer
    let cube = Cube::new(
        Point3::new(-1.0, 0.0, -1.0),
        Point3::new(1.0, 2.0, 1.0),
        cube_material,
    );
    world.add(Box::new(cube));
    world
}

fn scene_all_objects() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Square::horizontal(
        Point3::new(0.0, 0.0, 0.0),
        1000.0,
        ground_material.clone(),
    )));
    let sphere_material = Rc::new(Metal::new(Color::new(0.2, 0.7, 0.7), 0.1));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 1.0),
        1.0,
        sphere_material,
    )));
    let cube_material = Rc::new(Metal::new(Color::new(0.2, 0.7, 0.7), 0.1));
    let cube = Cube::new(
        Point3::new(-4.5, 0.0, 0.0),
        Point3::new(-2.5, 2.0, 2.0),
        cube_material,
    );
    world.add(Box::new(cube));
    let cylinder_material = Rc::new(Lambertian::new(Color::new(0.8, 1.0, 0.2)));
    let cylinder = crate::shapes::cylinder::Cylinder::new(
        Point3::new(3.5, 0.0, 1.0),
        vec3::Vec3::new(0.0, 1.0, 0.0),
        0.8,
        2.0,
        cylinder_material,
    );
    world.add(Box::new(cylinder));
    world
}

fn scene_all_objects_alt_camera() -> (HittableList, Point3, Point3) {
    let world = scene_all_objects();
    let lookfrom = Point3::new(0.0, 5.0, 10.0); // Camera from the side

    let lookat = Point3::new(0.0, 1.0, 1.0);
    (world, lookfrom, lookat)
}

fn main() {
    // Image

    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 800;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;

    // Select the scene to render:
    let scene_type = SceneType::AllObjectsAltCamera;

    // World and camera setup
    let (world, lookfrom, lookat) = match scene_type {
        SceneType::Sphere => {
            let w = scene_sphere();
            (w, Point3::new(0.0, 2.0, 5.0), Point3::new(0.0, 1.0, 0.0))
        }
        SceneType::PlaneCube => {
            let w = scene_plane_cube();
            (w, Point3::new(0.0, 3.0, 7.0), Point3::new(0.0, 1.0, 0.0))
        }
        SceneType::AllObjects => {
            let w = scene_all_objects();
            (w, Point3::new(0.0, 3.0, 10.0), Point3::new(0.0, 1.0, 1.0))
        }
        SceneType::AllObjectsAltCamera => {
            let (w, lookfrom, lookat) = scene_all_objects_alt_camera();
            (w, lookfrom, lookat)
        }
    };

    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.05;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        43.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprint!("\nDone.\n");
}
