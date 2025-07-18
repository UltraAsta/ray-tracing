use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    material::Material,
    ray::Ray,
    shapes::Square,
    vec3::{Point3, Vec3},
};

pub struct Cube {
    pub sides: HittableList,
}

impl Cube {
    pub fn new(p_min: Point3, p_max: Point3, material: Rc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        // Calculate dimensions dynamically
        let width = p_max.x() - p_min.x();
        let height = p_max.y() - p_min.y();
        let depth = p_max.z() - p_min.z();

        // Calculate center positions for each face
        let center_x = (p_min.x() + p_max.x()) / 2.0;
        let center_y = (p_min.y() + p_max.y()) / 2.0;
        let center_z = (p_min.z() + p_max.z()) / 2.0;

        // Front face (positive Z)
        let front_face = Box::new(Square::new(
            Point3::new(center_x, center_y, p_max.z()),
            Vec3::new(0.0, 0.0, 1.0),
            width.max(height), // Use the larger dimension for the square
            material.clone(),
        ));

        // Back face (negative Z)
        let back_face = Box::new(Square::new(
            Point3::new(center_x, center_y, p_min.z()),
            Vec3::new(0.0, 0.0, -1.0), // Note: flipped normal
            width.max(height),
            material.clone(),
        ));

        // Top face (positive Y)
        let top_face = Box::new(Square::new(
            Point3::new(center_x, p_max.y(), center_z),
            Vec3::new(0.0, 1.0, 0.0),
            width.max(depth),
            material.clone(),
        ));

        // Bottom face (negative Y)
        let bottom_face = Box::new(Square::new(
            Point3::new(center_x, p_min.y(), center_z),
            Vec3::new(0.0, -1.0, 0.0), // Note: flipped normal
            width.max(depth),
            material.clone(),
        ));

        // Right face (positive X)
        let right_face = Box::new(Square::new(
            Point3::new(p_max.x(), center_y, center_z),
            Vec3::new(1.0, 0.0, 0.0),
            height.max(depth),
            material.clone(),
        ));

        // Left face (negative X)
        let left_face = Box::new(Square::new(
            Point3::new(p_min.x(), center_y, center_z),
            Vec3::new(-1.0, 0.0, 0.0), // Note: flipped normal
            height.max(depth),
            material,
        ));

        sides.add(front_face);
        sides.add(back_face);
        sides.add(top_face);
        sides.add(bottom_face);
        sides.add(right_face);
        sides.add(left_face);

        Cube { sides }
    }

    // Helper constructors for common cube types
    pub fn centered(center: Point3, size: f64, material: Rc<dyn Material>) -> Self {
        let half_size = size / 2.0;
        let p_min = Point3::new(
            center.x() - half_size,
            center.y() - half_size,
            center.z() - half_size,
        );
        let p_max = Point3::new(
            center.x() + half_size,
            center.y() + half_size,
            center.z() + half_size,
        );

        Cube::new(p_min, p_max, material)
    }

    pub fn from_size(
        corner: Point3,
        width: f64,
        height: f64,
        depth: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        let p_min = corner;
        let p_max = Point3::new(corner.x() + width, corner.y() + height, corner.z() + depth);

        Cube::new(p_min, p_max, material)
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }
}

// Even more advanced: Rectangular Box (different dimensions for each axis)
pub struct RectangularBox {
    pub sides: HittableList,
}

impl RectangularBox {
    pub fn new(p_min: Point3, p_max: Point3, material: Rc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        // Calculate actual dimensions
        let width = (p_max.x() - p_min.x()).abs();
        let height = (p_max.y() - p_min.y()).abs();
        let depth = (p_max.z() - p_min.z()).abs();

        // Calculate centers
        let center_x = (p_min.x() + p_max.x()) / 2.0;
        let center_y = (p_min.y() + p_max.y()) / 2.0;
        let center_z = (p_min.z() + p_max.z()) / 2.0;

        // Create faces with exact dimensions (requires rectangular square support)
        // For now, we'll use the maximum dimension approach

        // Front and back faces (YZ plane)
        sides.add(Box::new(Square::new(
            Point3::new(center_x, center_y, p_max.z()),
            Vec3::new(0.0, 0.0, 1.0),
            width.max(height),
            material.clone(),
        )));

        sides.add(Box::new(Square::new(
            Point3::new(center_x, center_y, p_min.z()),
            Vec3::new(0.0, 0.0, -1.0),
            width.max(height),
            material.clone(),
        )));

        // Top and bottom faces (XZ plane)
        sides.add(Box::new(Square::new(
            Point3::new(center_x, p_max.y(), center_z),
            Vec3::new(0.0, 1.0, 0.0),
            width.max(depth),
            material.clone(),
        )));

        sides.add(Box::new(Square::new(
            Point3::new(center_x, p_min.y(), center_z),
            Vec3::new(0.0, -1.0, 0.0),
            width.max(depth),
            material.clone(),
        )));

        // Left and right faces (XY plane)
        sides.add(Box::new(Square::new(
            Point3::new(p_max.x(), center_y, center_z),
            Vec3::new(1.0, 0.0, 0.0),
            height.max(depth),
            material.clone(),
        )));

        sides.add(Box::new(Square::new(
            Point3::new(p_min.x(), center_y, center_z),
            Vec3::new(-1.0, 0.0, 0.0),
            height.max(depth),
            material,
        )));

        RectangularBox { sides }
    }
}

impl Hittable for RectangularBox {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }
}
