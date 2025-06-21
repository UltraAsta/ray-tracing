use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        // did it hit any object ?
        let mut hit_anything = false;
        // will store the nearest object that the ray hits
        // by default it's set to the furthest possible point
        let mut closest_so_far = t_max;

        for object in &self.objects {
            // check hit for each object in the list
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                // found a hit
                hit_anything = true;
                // update the closest distance
                // now this becomes our new maximum distance
                closest_so_far = temp_rec.t;
                // save the hit info
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
