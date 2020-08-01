use crate::vec3::Vec3;
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    /// Returns the point along the ray according to parameter t
    ///
    /// Starting at the origin, projects the ray according to it's direction
    /// and the value of t and returns a vector containing the new position
    ///
    /// * `t` - A value describing how far to project the ray
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn at() {
        let origin = Vec3::new(2, 3, 4);
        let direction = Vec3::new(0, 1, 0);
        let ray = Ray::new(origin, direction);

        let expected = Vec3::new(2, 3.5, 4);

        assert_eq!(
            ray.at(0.5),
            expected,
            "Ray.at(0.5) ({:?}) should match expected ({:?})",
            ray.at(0.5),
            expected
        );
    }
}
