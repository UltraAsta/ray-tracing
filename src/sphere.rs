use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, r: f64, m: Rc<dyn Material>) -> Self {
        Self {
            center: center,
            radius: r,
            mat: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // calculating from point to center which is the equation for is (p-c)
        let oc = r.origin() - self.center;
        let a = vec3::dot(r.direction(), r.direction());
        let half_b = vec3::dot(oc, r.direction());
        let c = vec3::dot(oc, oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        // check if it's hit or miss, to only render what's in front of us and not see what is behind
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = f64::sqrt(discriminant);

        // find the nearest root that lies in the nearest range
        let mut root = (-half_b - sqrt_d) / a;

        if root <= t_min || t_max <= root {
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        // conevrt into a unit vector by dividing by the radius
        let outwards_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outwards_normal);
        rec.mat = Some(self.mat.clone());
        true
    }
}
