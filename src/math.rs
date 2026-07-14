use std::{
    fmt::{self},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

///Struct representing a Vector in a two dimensional spaces

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
//-------------------------------
// --------operator overload-----
// ------------------------------
impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
//component * component mult
impl Mul for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Vec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

// ------------------------------------------
// Vec2 Display
// ------------------------------------------
impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

// ------------------------------------------
//Vec2 Implementation
// ------------------------------------------

impl Vec2 {
    // ------------------------------------------
    //consts
    // ------------------------------------------

    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const UNIT: Self = Self { x: 1.0, y: 1.0 };

    ///returns the length of the vector Squared
    pub fn len_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    ///length of the vector
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// returns a normalized vector of self but keeping the og vector untouched
    pub fn normalize(self) -> Self {
        let length = self.len();
        if length == 0.0f32 {
            return Self { x: 0.0, y: 0.0 };
        }
        Self {
            x: self.x / length,
            y: self.y / length,
        }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
    ///cross product that only return the z component( a escalar )
    pub fn cross(self, rhs: Self) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }
}

// ------------------------------------------
// MACROS
// ------------------------------------------

#[macro_export]
macro_rules! v2 {
    ($x:expr, $y:expr) => {
        $crate::math::Vec2 { x: $x, y: $y }
    };
}

// ------------------------------------------
// UTESTS
// ------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Helper macro to compare floats with a margin of error (epsilon).
    // Prevents a test from failing if the result is 0.30000004 instead of 0.3.
    macro_rules! assert_f32_eq {
        ($a:expr, $b:expr) => {
            assert!(
                ($a - $b).abs() < 1e-5,
                "Precision failure: {} is not equal to {}",
                $a,
                $b
            );
        };
    }

    // ------------------------------------------
    // Creation and Constants Tests
    // ------------------------------------------
    #[test]
    fn test_macro_v2() {
        let v = v2!(1.5, 2.5);
        assert_eq!(v.x, 1.5);
        assert_eq!(v.y, 2.5);
    }

    #[test]
    fn test_constants() {
        assert_eq!(Vec2::ZERO, v2!(0.0, 0.0));
        assert_eq!(Vec2::UNIT, v2!(1.0, 1.0));
    }

    // ------------------------------------------
    // Basic Operators Tests (Addition and Subtraction)
    // ------------------------------------------
    #[test]
    fn test_add() {
        let a = v2!(1.0, 2.0);
        let b = v2!(3.0, -1.0);
        assert_eq!(a + b, v2!(4.0, 1.0));
    }

    #[test]
    fn test_add_assign() {
        let mut a = v2!(1.0, 2.0);
        a += v2!(3.0, 4.0);
        assert_eq!(a, v2!(4.0, 6.0));
    }

    #[test]
    fn test_sub() {
        let a = v2!(5.0, 5.0);
        let b = v2!(2.0, 7.0);
        assert_eq!(a - b, v2!(3.0, -2.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = v2!(5.0, 5.0);
        a -= v2!(2.0, 1.0);
        assert_eq!(a, v2!(3.0, 4.0));
    }

    #[test]
    fn test_neg() {
        let a = v2!(3.0, -4.0);
        assert_eq!(-a, v2!(-3.0, 4.0));
    }

    // ------------------------------------------
    // Multiplication and Division Tests
    // ------------------------------------------
    #[test]
    fn test_mul_vec2() {
        let a = v2!(2.0, 3.0);
        let b = v2!(4.0, -2.0);
        assert_eq!(a * b, v2!(8.0, -6.0));
    }

    #[test]
    fn test_mul_f32() {
        let a = v2!(2.0, -3.0);
        assert_eq!(a * 2.0, v2!(4.0, -6.0));
    }

    #[test]
    fn test_mul_assign_vec2() {
        let mut a = v2!(2.0, 3.0);
        a *= v2!(4.0, 5.0);
        assert_eq!(a, v2!(8.0, 15.0));
    }

    #[test]
    fn test_mul_assign_f32() {
        let mut a = v2!(2.0, 3.0);
        a *= 3.0;
        assert_eq!(a, v2!(6.0, 9.0));
    }

    #[test]
    fn test_div_vec2() {
        let a = v2!(8.0, 15.0);
        let b = v2!(2.0, 3.0);
        assert_eq!(a / b, v2!(4.0, 5.0));
    }

    #[test]
    fn test_div_f32() {
        let a = v2!(8.0, -4.0);
        assert_eq!(a / 2.0, v2!(4.0, -2.0));
    }

    #[test]
    fn test_div_assign_f32() {
        let mut a = v2!(8.0, 6.0);
        a /= 2.0;
        assert_eq!(a, v2!(4.0, 3.0));
    }

    // ------------------------------------------
    // Geometric Functions Tests (Physics)
    // ------------------------------------------
    #[test]
    fn test_len_sq() {
        let a = v2!(3.0, 4.0);
        assert_f32_eq!(a.len_sq(), 25.0);
    }

    #[test]
    fn test_len() {
        let a = v2!(3.0, 4.0);
        assert_f32_eq!(a.len(), 5.0);
    }

    #[test]
    fn test_normalize() {
        let a = v2!(3.0, 4.0);
        let n = a.normalize();
        assert_f32_eq!(n.x, 0.6);
        assert_f32_eq!(n.y, 0.8);
    }

    #[test]
    fn test_normalize_zero() {
        let a = v2!(0.0, 0.0);
        let n = a.normalize();
        assert_eq!(n, v2!(0.0, 0.0));
    }

    #[test]
    fn test_dot_product() {
        let a = v2!(2.0, 3.0);
        let b = v2!(4.0, 5.0);
        // (2 * 4) + (3 * 5) = 8 + 15 = 23
        assert_f32_eq!(a.dot(b), 23.0);
    }

    #[test]
    fn test_cross_product() {
        let a = v2!(2.0, 3.0);
        let b = v2!(4.0, 5.0);
        // (2 * 5) - (3 * 4) = 10 - 12 = -2
        assert_f32_eq!(a.cross(b), -2.0);
    }

    #[test]
    fn test_display() {
        let a = v2!(1.5, -2.5);
        assert_eq!(format!("{}", a), "(1.5,-2.5)");
    }
}
