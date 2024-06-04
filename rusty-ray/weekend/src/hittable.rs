use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestHittable {
        pub hit: bool,
    }

    impl Hittable for TestHittable {
        fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
            if self.hit {
                rec.p = Point3::new(0.0, 0.0, 0.0);
                rec.normal = Vec3::new(1.0, 0.0, 0.0);
                rec.t = 1.0;
                true
            } else {
                false
            }
        }
    }

    #[test]
    fn test_hit() {
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let mut rec = HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
        };

        let hittable = TestHittable { hit: true };
        assert!(hittable.hit(&r, 0.0, 1.0, &mut rec));
        assert_eq!(rec.p, Point3::new(0.0, 0.0, 0.0));
        assert_eq!(rec.normal, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(rec.t, 1.0);

        let hittable = TestHittable { hit: false };
        assert!(!hittable.hit(&r, 0.0, 1.0, &mut rec));
    }
}