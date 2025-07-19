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

// A finite cylinder with two caps
pub struct Cylinder {
    pub base_center: Point3, // Center of the base cap
    pub axis: Vec3,          // Normalized axis vector (direction from base to top)
    pub radius: f64,
    pub height: f64,
    pub material: Rc<dyn Material>,
}

impl Cylinder {
    pub fn new(base_center: Point3, axis: Vec3, radius: f64, height: f64, material: Rc<dyn Material>) -> Self {
        Self {
            base_center,
            axis: vec3::unit_vector(axis),
            radius,
            height,
            material,
        }
    }
}

impl Hittable for Cylinder {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        use crate::vec3::dot;
        let oc = r.origin() - self.base_center;
        let axis = self.axis;
        let axis_dot_d = dot(r.direction(), axis);
        let axis_dot_oc = dot(oc, axis);
        let d_perp = r.direction() - axis * axis_dot_d;
        let oc_perp = oc - axis * axis_dot_oc;

        let a = d_perp.length_squared();
        let half_b = dot(d_perp, oc_perp);
        let c = oc_perp.length_squared() - self.radius * self.radius;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        // Tube intersection
        let discriminant = half_b * half_b - a * c;
        if discriminant >= 0.0 {
            let sqrt_d = discriminant.sqrt();
            for &sign in &[-1.0, 1.0] {
                let t = (-half_b + sign * sqrt_d) / a;
                if t < t_min || t > closest_so_far {
                    continue;
                }
                let p = r.at(t);
                let v = dot(p - self.base_center, axis);
                if v < 0.0 || v > self.height {
                    continue;
                }
                closest_so_far = t;
                rec.t = t;
                rec.p = p;
                let outward_normal = vec3::unit_vector(p - self.base_center - axis * v);
                rec.set_face_normal(r, outward_normal);
                rec.mat = Some(self.material.clone());
                hit_anything = true;
            }
        }
        // Cap intersection (bottom and top)
        for &(cap_offset, cap_normal_sign) in &[(0.0, 1.0), (self.height, -1.0)] {
            let cap_center = self.base_center + axis * cap_offset;
            let denom = dot(r.direction(), axis);
            if denom.abs() > 1e-8 {
                let t = dot(cap_center - r.origin(), axis) / denom;
                if t >= t_min && t <= closest_so_far {
                    let p = r.at(t);
                    if (p - cap_center).length_squared() <= self.radius * self.radius {
                        closest_so_far = t;
                        rec.t = t;
                        rec.p = p;
                        let outward_normal = axis * cap_normal_sign;
                        rec.set_face_normal(r, outward_normal);
                        rec.mat = Some(self.material.clone());
                        hit_anything = true;
                    }
                }
            }
        }
        hit_anything
    }
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
        // Always set the normal to oppose the ray direction
        let outward_normal = if intersection < 0.0 { self.normal } else { -self.normal };
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.material.clone());

        true
    }
}
