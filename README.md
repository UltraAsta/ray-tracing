# Ray Tracer Documentation

Welcome to the Rust Ray Tracer! This document explains its features and provides clear code examples so you can create scenes, objects, and adjust rendering settings with confidence.

---

## Features

- **Physically-Based Ray Tracing:** Simulates realistic light transport with recursive ray bounces.
- **Multiple Geometric Primitives:** Supports spheres, cubes, finite cylinders (with caps), and infinite flat planes.
- **Material System:** Includes Lambertian (diffuse) and Metal (reflective) surfaces.
- **Customizable Camera:** Easily adjust position, target, and field of view.
- **Scene Configuration:** Compose scenes with any combination of objects and materials.
- **Background Gradient:** Set the sky/background color for atmospheric effects.
- **High-Resolution Output:** Control image size and sampling for quality.

---

## Quickstart

1. **Build and Run:**
   ```sh
   cargo run --release > output.ppm
   ```
   The rendered image will be written to `output.ppm` (viewable with image viewers that support PPM).

2. **Switch Scenes:**
   In `main.rs`, change the `scene_type` variable to select your scene:
   ```rust
   let scene_type = SceneType::PlaneCube; // or Sphere, AllObjects, AllObjectsAltCamera
   ```

---

## Creating Objects

All objects are created and added to a `HittableList` (the scene). Here are examples for each shape:

### 1. Sphere
```rust
use shapes::Sphere;
use material::Lambertian;
use vec3::Point3;
use color::Color;
use std::rc::Rc;

let sphere_material = Rc::new(Lambertian::new(Color::new(0.8, 0.3, 0.3)));
let sphere = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, sphere_material);
world.add(Box::new(sphere));
```

### 2. Cube
```rust
use shapes::Cube;
use material::Metal;

let cube_material = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.1));
let cube = Cube::new(
    Point3::new(-1.0, 0.0, -1.0), // min corner
    Point3::new(1.0, 2.0, 1.0),   // max corner
    cube_material,
);
world.add(Box::new(cube));
```

### 3. Flat Plane (Square)
```rust
use shapes::Square;

let ground_material = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
let ground = Square::horizontal(
    Point3::new(0.0, 0.0, 0.0), // center
    1000.0,                     // size
    ground_material,
);
world.add(Box::new(ground));
```

### 4. Cylinder (Finite, with Caps)
```rust
use shapes::Cylinder;

let cylinder_material = Rc::new(Lambertian::new(Color::new(0.2, 0.5, 0.8)));
let cylinder = Cylinder::new(
    Point3::new(3.0, 0.0, 1.0), // base center
    vec3::Vec3::new(0.0, 1.0, 0.0), // axis (vertical)
    1.0, // height
    0.7, // radius
    cylinder_material,
);
world.add(Box::new(cylinder));
```

---

## Changing Scene Brightness

Scene brightness is mainly controlled by:
- **Background Gradient:** In `ray_color` (in `main.rs`), change the colors for the sky:
  ```rust
  let t = 0.5 * (unit_direction.y() + 1.0);
  (1.0 - t) * Color::new(1.0, 0.5, 0.2) + t * Color::new(0.2, 0.2, 0.4)
  ```
  - Use lighter colors for a brighter scene, darker for dimmer.
- **Material Colors:** Use higher values (closer to 1.0) for brighter materials.
- **Samples Per Pixel:** Increase `SAMPLES_PER_PIXEL` for smoother, less noisy images (increases render time).

---

## Changing Camera Position and Angle

The camera is set up in `main.rs`:
```rust
let lookfrom = Point3::new(0.0, 3.0, 7.0); // Camera position
let lookat = Point3::new(0.0, 1.0, 0.0);   // Target point

let cam = Camera::new(
    lookfrom,
    lookat,
    Point3::new(0.0, 1.0, 0.0), // up vector
    20.0,                       // vertical field of view (degrees)
    ASPECT_RATIO,
);
```
- **Move the camera:** Change `lookfrom`.
- **Change what it looks at:** Change `lookat`.
- **Adjust FOV:** Change the field of view parameter.

---

## Additional Tips

- **Add Objects:** Add as many shapes as you want to the `world` (scene) before rendering.
- **Materials:** Mix and match Lambertian and Metal for different looks.
- **Rendering:** Output is in PPM format. Use an image viewer or convert to PNG/JPG for easier viewing.

---

## Example: Full Minimal Scene
```rust
let mut world = HittableList::new();
let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
world.add(Box::new(Square::horizontal(Point3::new(0.0, 0.0, 0.0), 1000.0, ground_material)));
let sphere_material = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));
world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, sphere_material)));
// Set up camera and render as shown above.
```

---

## Need More?
- Explore and modify the scene functions in `main.rs` for more complex setups.
- For advanced lighting (e.g. colored backgrounds, sunset effects), edit the `ray_color` function.

Enjoy ray tracing!
