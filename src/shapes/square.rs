use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::rc::Rc;

pub struct Square {
    center: Point3,
    normal: Vec3,
    u_axis: Vec3, // First edge direction
    v_axis: Vec3, // Second edge direction
    size: f64,
    mat: Rc<dyn Material>,
}

impl Square {
    pub fn new(center: Point3, normal: Vec3, size: f64, material: Rc<dyn Material>) -> Self {
        let unit_normal = crate::vec3::unit_vector(normal);

        // Create perpendicular axes for the square
        // Choose an arbitrary vector that's not parallel to normal
        let temp = if unit_normal.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let u_axis = crate::vec3::unit_vector(crate::vec3::cross(unit_normal, temp));
        let v_axis = crate::vec3::cross(unit_normal, u_axis);

        Square {
            center,
            normal: unit_normal,
            u_axis,
            v_axis,
            size,
            mat: material,
        }
    }

    // Helper function: create a horizontal square (facing up)
    pub fn horizontal(center: Point3, size: f64, material: Rc<dyn Material>) -> Self {
        Square::new(center, Vec3::new(0.0, 1.0, 0.0), size, material)
    }

    // Helper function: create a vertical square (facing toward camera)
    pub fn vertical(center: Point3, size: f64, material: Rc<dyn Material>) -> Self {
        Square::new(center, Vec3::new(0.0, 0.0, 1.0), size, material)
    }
}

impl Hittable for Square {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Step 1: Check if ray hits the infinite plane
        let ray_dot_normal = crate::vec3::dot(r.direction(), self.normal);

        // If ray is parallel to the plane, no intersection
        if ray_dot_normal.abs() < 1e-8 {
            return false;
        }

        // Calculate t value for plane intersection
        let center_to_origin = self.center - r.origin();
        let t = crate::vec3::dot(center_to_origin, self.normal) / ray_dot_normal;

        // Check if intersection is within our t range
        if t <= t_min || t >= t_max {
            return false;
        }

        // Step 2: Find the intersection point
        let hit_point = r.at(t);

        // Step 3: Check if hit point is inside the square bounds
        let center_to_hit = hit_point - self.center;

        // Project onto the square's local coordinate system
        let u_coord = crate::vec3::dot(center_to_hit, self.u_axis);
        let v_coord = crate::vec3::dot(center_to_hit, self.v_axis);

        // Check bounds
        let half_size = self.size / 2.0;
        if u_coord.abs() > half_size || v_coord.abs() > half_size {
            return false;
        }

        // We have a hit! Fill the hit record
        rec.t = t;
        rec.p = hit_point;
        rec.set_face_normal(r, self.normal);
        rec.mat = Some(self.mat.clone());

        true
    }
}
