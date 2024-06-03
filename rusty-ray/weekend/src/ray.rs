use crate::vec3::Vec3;

#[derive(Default)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.orig // returns immutable reference to orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir // returns immutable reference to dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_at() {
        let orig = Vec3::new(1.0, 2.0, 3.0);
        let dir = Vec3::new(0.0, 1.0, 0.0);
        let ray = Ray::new(orig, dir);
        
        let t = 2.5;
        let expected_point = Vec3::new(1.0, 4.5, 3.0);
        assert_eq!(ray.at(t), expected_point);
    }

    #[test]
    fn test_ray_default() {
        let ray = Ray::default();
        let expected_origin = Vec3::default();
        let expected_direction = Vec3::default();
        assert_eq!(ray.origin(), &expected_origin);
        assert_eq!(ray.direction(), &expected_direction);
    }
}