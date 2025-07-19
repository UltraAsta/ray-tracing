use std::rc::Rc;

use crate::{
    hittable::Hittable,
    material::Material,
    vec3::{self, Point3, Vec3},
};

// this is a flat circle, so it kinda follows the same logic
pub struct Disk {
    center: Point3,
    normal: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Disk {
    pub fn new(center: Point3, normal: Vec3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center,
            normal,
            radius,
            material: mat,
        }
    }

    pub fn vertical(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Disk::new(center, Vec3::new(0.0, 0.0, 1.0), radius, mat)
    }

    pub fn horizontal(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Disk::new(center, Vec3::new(0.0, 1.0, 0.0), radius, mat)
    }
}

impl Hittable for Disk {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        // find the intersection within the plane
        let intersection = vec3::dot(r.direction(), self.normal);

        // if the value is near 0 it does not intersect
        if intersection.abs() < 1e-8 {
            return false;
        }

        let center_to_origin = self.center - r.origin();
        let t = vec3::dot(center_to_origin, self.normal) / intersection;

        // if the t is not within bounds, it means we missed
        if t < t_min || t > t_max {
            return false;
        }

        // check where does the ray hit
        let hit_point = r.at(t);

        // check if the hit is inside the radius
        if (hit_point - self.center).length_squared() >= self.radius * self.radius {
            return false;
        }

        rec.t = t;
        rec.p = hit_point;
        rec.set_face_normal(r, self.normal);
        rec.mat = Some(self.material.clone());

        true
    }
}
