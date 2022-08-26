#![cfg_attr(
    all(
        any(target_arch = "arm", target_arch = "aarch64"),
        target_feature = "neon"
    ),
    feature(stdsimd)
)]

pub mod pga3;
pub mod simd;

impl Exp for pga3::Line {
    type Output = pga3::Motor;

    fn exp(self) -> pga3::Motor {
        let det = self.g1[0] * self.g1[0] + self.g1[1] * self.g1[1] + self.g1[2] * self.g1[2];
        if det <= 0.0 {
            return pga3::Motor {
                g0: [1.0, 0.0, 0.0, 0.0].into(),
                g1: [0.0, self.g0[0], self.g0[1], self.g0[2]].into(),
            };
        }
        let a = det.sqrt();
        let c = a.cos();
        let s = a.sin() / a;
        let m = self.g0[0] * self.g1[0] + self.g0[1] * self.g1[1] + self.g0[2] * self.g1[2];
        let t = m / det * (c - s);
        let g0 = simd::Simd32x3::from(s) * self.g1;
        let g1 = simd::Simd32x3::from(s) * self.g0 + simd::Simd32x3::from(t) * self.g1;
        pga3::Motor {
            g0: [c, g0[0], g0[1], g0[2]].into(),
            g1: [s * m, g1[0], g1[1], g1[2]].into(),
        }
    }
}

impl Ln for pga3::Motor {
    type Output = pga3::Line;

    fn ln(self) -> pga3::Line {
        let det = 1.0 - self.g0[0] * self.g0[0];
        if det <= 0.0 {
            return pga3::Line {
                g0: [self.g1[1], self.g1[2], self.g1[3]].into(),
                g1: [0.0, 0.0, 0.0].into(),
            };
        }
        let a = 1.0 / det;
        let b = self.g0[0].acos() * a.sqrt();
        let c = a * self.g1[0] * (1.0 - self.g0[0] * b);
        let g0 = simd::Simd32x4::from(b) * self.g1 + simd::Simd32x4::from(c) * self.g0;
        let g1 = simd::Simd32x4::from(b) * self.g0;
        pga3::Line {
            g0: [g0[1], g0[2], g0[3]].into(),
            g1: [g1[1], g1[2], g1[3]].into(),
        }
    }
}

impl Powf for pga3::Motor {
    type Output = Self;

    fn powf(self, exponent: f32) -> Self {
        (pga3::Scalar { g0: exponent } * self.ln()).exp()
    }
}

impl Exp for pga3::Branch {
    type Output = pga3::Translator;

    fn exp(self) -> pga3::Translator {
        pga3::Translator {
            g0: [1.0, self.g0[0], self.g0[1], self.g0[2]].into(),
        }
    }
}

impl Ln for pga3::Translator {
    type Output = pga3::Branch;

    fn ln(self) -> pga3::Branch {
        pga3::Branch {
            g0: [
                self.g0[1] / self.g0[0],
                self.g0[2] / self.g0[0],
                self.g0[3] / self.g0[0],
            ]
            .into(),
        }
    }
}

impl Powf for pga3::Translator {
    type Output = Self;

    fn powf(self, exponent: f32) -> Self {
        (pga3::Scalar { g0: exponent } * self.ln()).exp()
    }
}

impl pga3::Scalar {
    pub fn new(x: f32) -> Self {
        Self { g0: x }
    }
}

impl pga3::Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            g0: [x, y, z, 1.0].into(),
        }
    }
}

/// Projects a direction onto the unit sphere.
impl std::convert::From<pga3::Dir> for pga3::Point {
    fn from(d: pga3::Dir) -> Self {
        Self {
            g0: [d.g0[0], d.g0[1], d.g0[2], 1.0].into(),
        }
    }
}

/// All elements set to `0.0`
pub trait Zero {
    fn zero() -> Self;
}

/// All elements set to `0.0`, except for the scalar, which is set to `1.0`
pub trait One {
    fn one() -> Self;
}

/// Element order reversed
pub trait Dual {
    type Output;
    fn dual(self) -> Self::Output;
}

/// Negates elements with `grade % 2 == 1`
///
/// Also called main involution
pub trait Automorphism {
    type Output;
    fn automorphism(self) -> Self::Output;
}

/// Negates elements with `grade % 4 >= 2`
///
/// Also called transpose
pub trait Reversal {
    type Output;
    fn reversal(self) -> Self::Output;
}

/// Negates elements with `(grade + 3) % 4 < 2`
pub trait Conjugation {
    type Output;
    fn conjugation(self) -> Self::Output;
}

/// General multi vector multiplication
pub trait GeometricProduct<T> {
    type Output;
    fn geometric_product(self, other: T) -> Self::Output;
}

/// Dual of the geometric product grade filtered by `t == r + s`
///
/// Also called join
pub trait RegressiveProduct<T> {
    type Output;
    fn regressive_product(self, other: T) -> Self::Output;
}

/// Geometric product grade filtered by `t == r + s`
///
/// Also called meet or exterior product
pub trait OuterProduct<T> {
    type Output;
    fn outer_product(self, other: T) -> Self::Output;
}

/// Geometric product grade filtered by `t == (r - s).abs()`
///
/// Also called fat dot product
pub trait InnerProduct<T> {
    type Output;
    fn inner_product(self, other: T) -> Self::Output;
}

/// Geometric product grade filtered by `t == s - r`
pub trait LeftContraction<T> {
    type Output;
    fn left_contraction(self, other: T) -> Self::Output;
}

/// Geometric product grade filtered by `t == r - s`
pub trait RightContraction<T> {
    type Output;
    fn right_contraction(self, other: T) -> Self::Output;
}

/// Geometric product grade filtered by `t == 0`
pub trait ScalarProduct<T> {
    type Output;
    fn scalar_product(self, other: T) -> Self::Output;
}

/// `self * other * self.reversion()`
///
/// Also called sandwich product
pub trait Transformation<T> {
    type Output;
    fn transformation(self, other: T) -> Self::Output;
}

/// Square of the magnitude
pub trait SquaredMagnitude {
    type Output;
    fn squared_magnitude(self) -> Self::Output;
}

/// Length as scalar
///
/// Also called amplitude, absolute value or norm
pub trait Magnitude {
    type Output;
    fn magnitude(self) -> Self::Output;
}

/// Direction without magnitude (set to scalar `-1.0` or `1.0`)
///
/// Also called sign or normalize
pub trait Signum {
    type Output;
    fn signum(self) -> Self::Output;
}

/// Raises a number to the scalar power of `-1.0`
pub trait Inverse {
    type Output;
    fn inverse(self) -> Self::Output;
}

/// The natural logarithm
pub trait Ln {
    type Output;
    fn ln(self) -> Self::Output;
}

/// The exponential function
pub trait Exp {
    type Output;
    fn exp(self) -> Self::Output;
}

/// Raises a number to an integer scalar power
pub trait Powi {
    type Output;
    fn powi(self, exponent: isize) -> Self::Output;
}

/// Raises a number to an floating point scalar power
pub trait Powf {
    type Output;
    fn powf(self, exponent: f32) -> Self::Output;
}

/// Extracts the square root of a number
pub fn sqrt<T: Powf>(n: T) -> T::Output {
    n.powf(0.5)
}

// pub fn unitize<T: Powf>(n: T) -> T::Output {
//     n.powf(0.5)
// }
