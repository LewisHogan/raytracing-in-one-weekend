use std::ops::{Add, Div, Index, Mul, Sub};

// The derive means we don't need to manually implement it.
// Copy means this thing is essentially treated as a value type, and a copy of all fields
// is made on move (the original is not moved).
// Clone does a similar thing but is more explicit (.clone instead of doing it automatically).
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Creates a new Vec3.
    ///
    /// Given a type which can be converted into an f64, creates a Vec3.
    ///
    /// * `x` - first component of Vec3
    /// * `y` - second component of Vec3
    /// * `z` - third component of Vec3
    pub fn new<T, S, V>(x: T, y: S, z: V) -> Vec3
    where
        T: Into<f64>,
        S: Into<f64>,
        V: Into<f64>,
    {
        Vec3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    /// Dot product of this and another vector.
    ///
    /// Performs on the dot product on this vector (a) and another vector (b).
    ///
    /// * `other` - The other vector to use in the dot product
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product of this and another vector.
    ///
    /// Produces a vector perpendicular to both vectors.
    ///
    /// * `other` - The other vector to use in the cross product
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Length of the vector.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Squared length of the vector.
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalized(&self) -> Vec3 {
        let length = self.length();
        self.div(length)
    }
}

// Allows accessing the Vec components by component, e.g. my_vec[0]
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Expect index between 0 and 2"),
        }
    }
}

// Allows doing things like my_vec + 2.0 to work, along with my_vec + 2i32
impl<T> Add<T> for Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Self;
    fn add(self, scalar: T) -> Self {
        Vec3 {
            x: self.x + scalar.into(),
            y: self.y + scalar.into(),
            z: self.z + scalar.into(),
        }
    }
}

// Allows vectors to be summed together, e.g. my_vec + my_other_vec
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// This allows bidirectional adds (e.g. 3.0 + my_vector), currently unsure of a better way to do
// so, so only implemented for floats so far
impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        other + self
    }
}

// Allows doing things like my_vec - 2.0 to work, along with my_vec - 2i32
impl<T> Sub<T> for Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Self;
    fn sub(self, scalar: T) -> Self {
        Vec3 {
            x: self.x - scalar.into(),
            y: self.y - scalar.into(),
            z: self.z - scalar.into(),
        }
    }
}

// Allows vectors to be subtracted from each other, e.g. my_vec - my_other_vec
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// This allows bidirectional subs (e.g. 3.0 - my_vector), currently unsure of a better way to do
// so, so only implemented for floats so far
impl Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        other - self
    }
}

// Allows doing things like my_vec * 2.0 to work, along with my_vec * 2i32
impl<T> Mul<T> for Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Self;
    fn mul(self, scalar: T) -> Self {
        Vec3 {
            x: self.x * scalar.into(),
            y: self.y * scalar.into(),
            z: self.z * scalar.into(),
        }
    }
}

// Allows vectors to be multiplied together, e.g. my_vec * my_other_vec
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

// This allows bidirectional multiplys (e.g. 3.0 * my_vector), currently unsure of a better way to do
// so, so only implemented for floats so far
// Note: I didn't implement division (e.g. 3 / my_vector) despite an inverse via multiplication
// being possible because I've never seen a number divided by a vector and am unsure if any
// specific maths depends on that being weirdly
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        other * self
    }
}

// Allows doing things like my_vec / 2.0 to work, along with my_vec / 2i32
impl<T> Div<T> for Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Self;
    fn div(self, scalar: T) -> Self {
        Vec3 {
            x: self.x / scalar.into(),
            y: self.y / scalar.into(),
            z: self.z / scalar.into(),
        }
    }
}

// Allows vectors to be divided by each other, e.g. my_vec / my_other_vec
impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_vector() {
        let v = Vec3 {
            x: 0.0,
            y: 2.0,
            z: 3.0,
        };

        let v2 = Vec3::new(0.0, 2, 3.0);

        assert_eq!(v.x, v2.x, "V.x ({}) should match V2.x ({})", v.x, v2.x);
        assert_eq!(v.y, v2.y, "V.y ({}) should match V2.y ({})", v.y, v2.y);
        assert_eq!(v.z, v2.z, "V.z ({}) should match V2.z ({})", v.z, v2.z);
        assert_eq!(v, v2, "V ({:?}) should match V2 ({:?})", v, v2);
    }

    #[test]
    fn add_vector_scalar() {
        let v = Vec3 {
            x: 0.0,
            y: 2.0,
            z: 3.0,
        };

        let expected_positive = Vec3 {
            x: 3.0,
            y: 5.0,
            z: 6.0,
        };

        let expected_negative = Vec3 {
            x: -3.0,
            y: -1.0,
            z: 0.0,
        };

        assert_eq!(
            v + 3,
            expected_positive,
            "V + 3 ({:?}) should match V2 ({:?})",
            v + 3,
            expected_positive
        );
        assert_eq!(
            v + -3,
            expected_negative,
            "V + -3 ({:?}) should match V2 ({:?})",
            v + -3,
            expected_negative
        );
    }

    #[test]
    fn add_vector_vector() {
        let v = Vec3 {
            x: 3.0,
            y: 2.0,
            z: 5.0,
        };

        let v2 = Vec3::new(5, 6.0, 6.2);

        let expected = Vec3 {
            x: 8.0,
            y: 8.0,
            z: 11.2,
        };

        assert_eq!(
            v + v2,
            expected,
            "V + V2 ({:?}) should match expected ({:?})",
            v + v2,
            expected
        );
    }

    #[test]
    fn sub_vector_scalar() {
        let v = Vec3 {
            x: 0.0,
            y: 2.0,
            z: 3.0,
        };

        let expected_negative = Vec3 {
            x: 3.0,
            y: 5.0,
            z: 6.0,
        };

        let expected_positive = Vec3 {
            x: -3.0,
            y: -1.0,
            z: 0.0,
        };

        assert_eq!(
            v - 3,
            expected_positive,
            "V + 3 ({:?}) should match V2 ({:?})",
            v + 3,
            expected_positive
        );
        assert_eq!(
            v - -3,
            expected_negative,
            "V + -3 ({:?}) should match V2 ({:?})",
            v + -3,
            expected_negative
        );
    }

    #[test]
    fn sub_vector_vector() {
        let v = Vec3 {
            x: 3.0,
            y: 2.0,
            z: 5.0,
        };

        let v2 = Vec3::new(5, 6.0, 6.5);

        let expected = Vec3 {
            x: -2.0,
            y: -4.0,
            z: -1.5,
        };

        assert_eq!(
            v - v2,
            expected,
            "V - V2 ({:?}) should match expected ({:?})",
            v - v2,
            expected
        );
    }
    #[test]
    fn mul_vector_scalar() {
        let v = Vec3 {
            x: 0.0,
            y: 2.0,
            z: 3.0,
        };

        let expected_negative = Vec3 {
            x: 0.0,
            y: -4.0,
            z: -6.0,
        };

        let expected_positive = Vec3 {
            x: 0.0,
            y: 4.0,
            z: 6.0,
        };

        assert_eq!(
            v * 2,
            expected_positive,
            "V * 2 ({:?}) should match V2 ({:?})",
            v * 2,
            expected_positive
        );
        assert_eq!(
            v * -2,
            expected_negative,
            "V * -2 ({:?}) should match V2 ({:?})",
            v * -2,
            expected_negative
        );
    }

    #[test]
    fn mul_vector_vector() {
        let v = Vec3 {
            x: 3.0,
            y: 2.0,
            z: 5.0,
        };

        let v2 = Vec3::new(5, 6.0, 6.5);

        let expected = Vec3::new(15, 12, 32.5);

        assert_eq!(
            v * v2,
            expected,
            "V * V2 ({:?}) should match expected ({:?})",
            v * v2,
            expected
        );
    }

    #[test]
    fn div_vector_scalar() {
        let v = Vec3 {
            x: 0.0,
            y: 2.0,
            z: 3.0,
        };

        let expected_negative = Vec3 {
            x: 0.0,
            y: -1.0,
            z: -1.5,
        };

        let expected_positive = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 1.5,
        };

        assert_eq!(
            v / 2,
            expected_positive,
            "V / 2 ({:?}) should match V2 ({:?})",
            v / 2,
            expected_positive
        );

        assert_eq!(
            v / -2,
            expected_negative,
            "V / -2 ({:?}) should match V2 ({:?})",
            v / -2,
            expected_negative
        );
    }

    #[test]
    fn div_vector_vector() {
        let v = Vec3 {
            x: 3.0,
            y: 2.0,
            z: 15.0,
        };

        let v2 = Vec3::new(5, 2.0, 3);

        let expected = Vec3::new(0.6, 1, 5);

        assert_eq!(
            v / v2,
            expected,
            "V / V2 ({:?}) should match expected ({:?})",
            v / v2,
            expected
        );
    }

    #[test]
    fn dot_vector() {
        let v = Vec3::new(9, 2, 7);
        let v2 = Vec3::new(4, 8, 10);
        let expected = 122f64;

        assert_eq!(
            v.dot(v2),
            expected,
            "V.cross(V2) ({:?}) should match expected ({:?})",
            v.dot(v2),
            expected
        );
    }

    #[test]
    fn cross_vector() {
        let v = Vec3::new(2, 3, 4);
        let v2 = Vec3::new(5, 6, 7);
        let expected = Vec3 {
            x: -3.0,
            y: 6.0,
            z: -3.0,
        };

        assert_eq!(
            v.cross(v2),
            expected,
            "V.dot(V2) ({:?}) should match expected ({:?})",
            v.cross(v2),
            expected
        );
    }

    #[test]
    fn length_vector() {
        let v = Vec3::new(3, 4, 0);
        let expected = 5.0;

        assert_eq!(
            v.length(),
            expected,
            "V.length() ({:?}) should match expected ({:?})",
            v.length(),
            expected
        );
    }

    #[test]
    fn length_squared() {
        let v = Vec3::new(3, 4, 0);
        let expected = 25.0;

        assert_eq!(
            v.length_squared(),
            expected,
            "V.length_squared() ({:?}) should match expected ({:?})",
            v.length(),
            expected
        );
    }

    #[test]
    fn index() {
        let v = Vec3 {
            x: 0.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(v[0], v.x, "V[0] ({}) should equal V.x ({})", v[0], v.x);
        assert_eq!(v[1], v.y, "V[0] ({}) should equal V.x ({})", v[0], v.x);
        assert_eq!(v[2], v.z, "V[0] ({}) should equal V.x ({})", v[0], v.x);
    }

    #[test]
    #[should_panic(expected = "Expect index between 0 and 2")]
    fn invalid_index() {
        let v = Vec3 {
            x: 0.0,
            y: 2.0,
            z: 3.0,
        };

        v[3];
    }

    #[test]
    fn normalize() {
        let v = Vec3::new(3, 1, 2);
        let expected = Vec3::new(0.8017837257372732, 0.2672612419124244, 0.5345224838248488);

        assert_eq!(
            v.normalized(),
            expected,
            "V.normalized() ({:?}) should match normalized ({:?})",
            v.normalized(),
            expected
        );
    }
}
