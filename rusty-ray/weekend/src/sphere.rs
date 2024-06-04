use crate::hittable::{Hittable, HitRecord};
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere {
            center,
            radius: f64::max(0.0, radius),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: FIX THE FOLLOWING TESTS
    #[test]
    fn test_sphere_hit() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Vec3::new(0.0, 0.0, -3.0), Vec3::new(0.0, 0.0, 1.0));
        let mut rec = HitRecord::default();
        assert!(sphere.hit(&ray, 0.0, f64::INFINITY, &mut rec));
        assert_eq!(rec.t, 2.0);
        assert_eq!(rec.p, Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(rec.normal, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_sphere_miss() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Vec3::new(0.0, 0.0, -3.0), Vec3::new(1.0, 1.0, 1.0));
        let mut rec = HitRecord::default();
        assert!(!sphere.hit(&ray, 0.0, f64::INFINITY, &mut rec));
    }

    // TODO: FIX THE FOLLOWING TESTS
    #[test]
    fn test_sphere_tmin_tmax() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Vec3::new(0.0, 0.0, -3.0), Vec3::new(0.0, 0.0, 1.0));
        let mut rec = HitRecord::default();
        assert!(!sphere.hit(&ray, 3.0, f64::INFINITY, &mut rec));
    }
}