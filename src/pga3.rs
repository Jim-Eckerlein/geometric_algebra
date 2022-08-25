#![allow(clippy::assign_op_pattern)]
use crate::{simd::*, *};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Scalar {
    /// 1
    pub g0: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct MultiVector {
    /// 1, e23, -e13, e12
    pub g0: Simd32x4,
    /// e0, -e023, e013, -e012
    pub g1: Simd32x4,
    /// e123, e1, e2, e3
    pub g2: Simd32x4,
    /// e0123, e01, e02, e03
    pub g3: Simd32x4,
}

#[derive(Clone, Copy, Debug)]
pub struct Rotor {
    /// 1, e23, -e13, e12
    pub g0: Simd32x4,
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    /// e123, -e023, e013, -e012
    pub g0: Simd32x4,
}

#[derive(Clone, Copy, Debug)]
pub struct Plane {
    /// e0, e1, e2, e3
    pub g0: Simd32x4,
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    /// e01, e02, e03
    pub g0: Simd32x3,
    /// e23, -e13, e12
    pub g1: Simd32x3,
}

#[derive(Clone, Copy, Debug)]
pub struct Translator {
    /// 1, e01, e02, e03
    pub g0: Simd32x4,
}

#[derive(Clone, Copy, Debug)]
pub struct Motor {
    /// 1, e23, -e13, e12
    pub g0: Simd32x4,
    /// e0123, e01, e02, e03
    pub g1: Simd32x4,
}

#[derive(Clone, Copy, Debug)]
pub struct Branch {
    /// e01, e02, e03
    pub g0: Simd32x3,
}

impl Zero for Scalar {
    fn zero() -> Self {
        Scalar { g0: 0.0 }
    }
}

impl One for Scalar {
    fn one() -> Self {
        Scalar { g0: 1.0 }
    }
}

impl Neg for Scalar {
    type Output = Scalar;

    fn neg(self) -> Scalar {
        Scalar { g0: self.g0 * -1.0 }
    }
}

impl Automorphism for Scalar {
    type Output = Scalar;

    fn automorphism(self) -> Scalar {
        Scalar { g0: self.g0 }
    }
}

impl Reversal for Scalar {
    type Output = Scalar;

    fn reversal(self) -> Scalar {
        Scalar { g0: self.g0 }
    }
}

impl Conjugation for Scalar {
    type Output = Scalar;

    fn conjugation(self) -> Scalar {
        Scalar { g0: self.g0 }
    }
}

impl Add<Scalar> for Scalar {
    type Output = Scalar;

    fn add(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g0 + other.g0 }
    }
}

impl AddAssign<Scalar> for Scalar {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Sub<Scalar> for Scalar {
    type Output = Scalar;

    fn sub(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g0 - other.g0 }
    }
}

impl SubAssign<Scalar> for Scalar {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn geometric_product(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g0 * other.g0 }
    }
}

impl OuterProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn outer_product(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g0 * other.g0 }
    }
}

impl InnerProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn inner_product(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g0 * other.g0 }
    }
}

impl LeftContraction<Scalar> for Scalar {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g0 * other.g0 }
    }
}

impl RightContraction<Scalar> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g0 * other.g0 }
    }
}

impl ScalarProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g0 * other.g0 }
    }
}

impl Add<MultiVector> for Scalar {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.g0, g1: other.g1, g2: other.g2, g3: other.g3 }
    }
}

impl Sub<MultiVector> for Scalar {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.g0, g1: Simd32x4::from(0.0) - other.g1, g2: Simd32x4::from(0.0) - other.g2, g3: Simd32x4::from(0.0) - other.g3 }
    }
}

impl GeometricProduct<MultiVector> for Scalar {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0) * other.g0, g1: Simd32x4::from(self.g0) * other.g1, g2: Simd32x4::from(self.g0) * other.g2, g3: Simd32x4::from(self.g0) * other.g3 }
    }
}

impl RegressiveProduct<MultiVector> for Scalar {
    type Output = Scalar;

    fn regressive_product(self, other: MultiVector) -> Scalar {
        Scalar { g0: self.g0 * other.g3[0] }
    }
}

impl OuterProduct<MultiVector> for Scalar {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0) * other.g0, g1: Simd32x4::from(self.g0) * other.g1, g2: Simd32x4::from(self.g0) * other.g2, g3: Simd32x4::from(self.g0) * other.g3 }
    }
}

impl InnerProduct<MultiVector> for Scalar {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0) * other.g0, g1: Simd32x4::from(self.g0) * other.g1, g2: Simd32x4::from(self.g0) * other.g2, g3: Simd32x4::from(self.g0) * other.g3 }
    }
}

impl LeftContraction<MultiVector> for Scalar {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0) * other.g0, g1: Simd32x4::from(self.g0) * other.g1, g2: Simd32x4::from(self.g0) * other.g2, g3: Simd32x4::from(self.g0) * other.g3 }
    }
}

impl RightContraction<MultiVector> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: MultiVector) -> Scalar {
        Scalar { g0: self.g0 * other.g0[0] }
    }
}

impl ScalarProduct<MultiVector> for Scalar {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { g0: self.g0 * other.g0[0] }
    }
}

impl Add<Rotor> for Scalar {
    type Output = Rotor;

    fn add(self, other: Rotor) -> Rotor {
        Rotor { g0: Simd32x4::from(self.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.g0 }
    }
}

impl Sub<Rotor> for Scalar {
    type Output = Rotor;

    fn sub(self, other: Rotor) -> Rotor {
        Rotor { g0: Simd32x4::from(self.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.g0 }
    }
}

impl GeometricProduct<Rotor> for Scalar {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { g0: Simd32x4::from(self.g0) * other.g0 }
    }
}

impl OuterProduct<Rotor> for Scalar {
    type Output = Rotor;

    fn outer_product(self, other: Rotor) -> Rotor {
        Rotor { g0: Simd32x4::from(self.g0) * other.g0 }
    }
}

impl InnerProduct<Rotor> for Scalar {
    type Output = Rotor;

    fn inner_product(self, other: Rotor) -> Rotor {
        Rotor { g0: Simd32x4::from(self.g0) * other.g0 }
    }
}

impl LeftContraction<Rotor> for Scalar {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { g0: Simd32x4::from(self.g0) * other.g0 }
    }
}

impl RightContraction<Rotor> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: Rotor) -> Scalar {
        Scalar { g0: self.g0 * other.g0[0] }
    }
}

impl ScalarProduct<Rotor> for Scalar {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { g0: self.g0 * other.g0[0] }
    }
}

impl GeometricProduct<Point> for Scalar {
    type Output = Point;

    fn geometric_product(self, other: Point) -> Point {
        Point { g0: Simd32x4::from(self.g0) * other.g0 }
    }
}

impl OuterProduct<Point> for Scalar {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { g0: Simd32x4::from(self.g0) * other.g0 }
    }
}

impl InnerProduct<Point> for Scalar {
    type Output = Point;

    fn inner_product(self, other: Point) -> Point {
        Point { g0: Simd32x4::from(self.g0) * other.g0 }
    }
}

impl LeftContraction<Point> for Scalar {
    type Output = Point;

    fn left_contraction(self, other: Point) -> Point {
        Point { g0: Simd32x4::from(self.g0) * other.g0 }
    }
}

impl GeometricProduct<Plane> for Scalar {
    type Output = Plane;

    fn geometric_product(self, other: Plane) -> Plane {
        Plane { g0: Simd32x4::from(self.g0) * other.g0 }
    }
}

impl OuterProduct<Plane> for Scalar {
    type Output = Plane;

    fn outer_product(self, other: Plane) -> Plane {
        Plane { g0: Simd32x4::from(self.g0) * other.g0 }
    }
}

impl InnerProduct<Plane> for Scalar {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { g0: Simd32x4::from(self.g0) * other.g0 }
    }
}

impl LeftContraction<Plane> for Scalar {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { g0: Simd32x4::from(self.g0) * other.g0 }
    }
}

impl GeometricProduct<Line> for Scalar {
    type Output = Line;

    fn geometric_product(self, other: Line) -> Line {
        Line { g0: Simd32x3::from(self.g0) * other.g0, g1: Simd32x3::from(self.g0) * other.g1 }
    }
}

impl OuterProduct<Line> for Scalar {
    type Output = Line;

    fn outer_product(self, other: Line) -> Line {
        Line { g0: Simd32x3::from(self.g0) * other.g0, g1: Simd32x3::from(self.g0) * other.g1 }
    }
}

impl InnerProduct<Line> for Scalar {
    type Output = Line;

    fn inner_product(self, other: Line) -> Line {
        Line { g0: Simd32x3::from(self.g0) * other.g0, g1: Simd32x3::from(self.g0) * other.g1 }
    }
}

impl LeftContraction<Line> for Scalar {
    type Output = Line;

    fn left_contraction(self, other: Line) -> Line {
        Line { g0: Simd32x3::from(self.g0) * other.g0, g1: Simd32x3::from(self.g0) * other.g1 }
    }
}

impl Add<Translator> for Scalar {
    type Output = Translator;

    fn add(self, other: Translator) -> Translator {
        Translator { g0: Simd32x4::from(self.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.g0 }
    }
}

impl Sub<Translator> for Scalar {
    type Output = Translator;

    fn sub(self, other: Translator) -> Translator {
        Translator { g0: Simd32x4::from(self.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.g0 }
    }
}

impl GeometricProduct<Translator> for Scalar {
    type Output = Translator;

    fn geometric_product(self, other: Translator) -> Translator {
        Translator { g0: Simd32x4::from(self.g0) * other.g0 }
    }
}

impl OuterProduct<Translator> for Scalar {
    type Output = Translator;

    fn outer_product(self, other: Translator) -> Translator {
        Translator { g0: Simd32x4::from(self.g0) * other.g0 }
    }
}

impl InnerProduct<Translator> for Scalar {
    type Output = Translator;

    fn inner_product(self, other: Translator) -> Translator {
        Translator { g0: Simd32x4::from(self.g0) * other.g0 }
    }
}

impl LeftContraction<Translator> for Scalar {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { g0: Simd32x4::from(self.g0) * other.g0 }
    }
}

impl RightContraction<Translator> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: Translator) -> Scalar {
        Scalar { g0: self.g0 * other.g0[0] }
    }
}

impl ScalarProduct<Translator> for Scalar {
    type Output = Scalar;

    fn scalar_product(self, other: Translator) -> Scalar {
        Scalar { g0: self.g0 * other.g0[0] }
    }
}

impl Add<Motor> for Scalar {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.g0, g1: other.g1 }
    }
}

impl Sub<Motor> for Scalar {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.g0, g1: Simd32x4::from(0.0) - other.g1 }
    }
}

impl GeometricProduct<Motor> for Scalar {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0) * other.g0, g1: Simd32x4::from(self.g0) * other.g1 }
    }
}

impl RegressiveProduct<Motor> for Scalar {
    type Output = Scalar;

    fn regressive_product(self, other: Motor) -> Scalar {
        Scalar { g0: self.g0 * other.g1[0] }
    }
}

impl OuterProduct<Motor> for Scalar {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0) * other.g0, g1: Simd32x4::from(self.g0) * other.g1 }
    }
}

impl InnerProduct<Motor> for Scalar {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0) * other.g0, g1: Simd32x4::from(self.g0) * other.g1 }
    }
}

impl LeftContraction<Motor> for Scalar {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0) * other.g0, g1: Simd32x4::from(self.g0) * other.g1 }
    }
}

impl RightContraction<Motor> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: Motor) -> Scalar {
        Scalar { g0: self.g0 * other.g0[0] }
    }
}

impl ScalarProduct<Motor> for Scalar {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { g0: self.g0 * other.g0[0] }
    }
}

impl Add<Branch> for Scalar {
    type Output = Translator;

    fn add(self, other: Branch) -> Translator {
        Translator { g0: Simd32x4::from(self.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl Sub<Branch> for Scalar {
    type Output = Translator;

    fn sub(self, other: Branch) -> Translator {
        Translator { g0: Simd32x4::from(self.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl GeometricProduct<Branch> for Scalar {
    type Output = Branch;

    fn geometric_product(self, other: Branch) -> Branch {
        Branch { g0: Simd32x3::from(self.g0) * other.g0 }
    }
}

impl OuterProduct<Branch> for Scalar {
    type Output = Branch;

    fn outer_product(self, other: Branch) -> Branch {
        Branch { g0: Simd32x3::from(self.g0) * other.g0 }
    }
}

impl InnerProduct<Branch> for Scalar {
    type Output = Branch;

    fn inner_product(self, other: Branch) -> Branch {
        Branch { g0: Simd32x3::from(self.g0) * other.g0 }
    }
}

impl LeftContraction<Branch> for Scalar {
    type Output = Branch;

    fn left_contraction(self, other: Branch) -> Branch {
        Branch { g0: Simd32x3::from(self.g0) * other.g0 }
    }
}

impl SquaredMagnitude for Scalar {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Scalar {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { g0: self.squared_magnitude().g0.sqrt() }
    }
}

impl Signum for Scalar {
    type Output = Scalar;

    fn signum(self) -> Scalar {
        self.geometric_product(Scalar { g0: 1.0 / self.magnitude().g0 })
    }
}

impl Inverse for Scalar {
    type Output = Scalar;

    fn inverse(self) -> Scalar {
        self.reversal().geometric_product(Scalar { g0: 1.0 / self.squared_magnitude().g0 })
    }
}

impl Zero for MultiVector {
    fn zero() -> Self {
        MultiVector { g0: Simd32x4::from(0.0), g1: Simd32x4::from(0.0), g2: Simd32x4::from(0.0), g3: Simd32x4::from(0.0) }
    }
}

impl One for MultiVector {
    fn one() -> Self {
        MultiVector { g0: Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x4::from(0.0), g3: Simd32x4::from(0.0) }
    }
}

impl Neg for MultiVector {
    type Output = MultiVector;

    fn neg(self) -> MultiVector {
        MultiVector { g0: self.g0 * Simd32x4::from(-1.0), g1: self.g1 * Simd32x4::from(-1.0), g2: self.g2 * Simd32x4::from(-1.0), g3: self.g3 * Simd32x4::from(-1.0) }
    }
}

impl Automorphism for MultiVector {
    type Output = MultiVector;

    fn automorphism(self) -> MultiVector {
        MultiVector { g0: self.g0, g1: self.g1 * Simd32x4::from(-1.0), g2: self.g2 * Simd32x4::from(-1.0), g3: self.g3 }
    }
}

impl Reversal for MultiVector {
    type Output = MultiVector;

    fn reversal(self) -> MultiVector {
        MultiVector { g0: self.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g1: self.g1 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g2: self.g2 * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]), g3: self.g3 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) }
    }
}

impl Conjugation for MultiVector {
    type Output = MultiVector;

    fn conjugation(self) -> MultiVector {
        MultiVector { g0: self.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g1: self.g1 * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]), g2: self.g2 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g3: self.g3 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) }
    }
}

impl Dual for MultiVector {
    type Output = MultiVector;

    fn dual(self) -> MultiVector {
        MultiVector { g0: self.g3, g1: self.g2 * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]), g2: self.g1 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g3: self.g0 }
    }
}

impl Into<Scalar> for MultiVector {
    fn into(self) -> Scalar {
        Scalar { g0: self.g0[0] }
    }
}

impl Add<Scalar> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Scalar) -> MultiVector {
        MultiVector { g0: self.g0 + Simd32x4::from(other.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.g1, g2: self.g2, g3: self.g3 }
    }
}

impl AddAssign<Scalar> for MultiVector {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Sub<Scalar> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Scalar) -> MultiVector {
        MultiVector { g0: self.g0 - Simd32x4::from(other.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.g1, g2: self.g2, g3: self.g3 }
    }
}

impl SubAssign<Scalar> for MultiVector {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Scalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Scalar) -> MultiVector {
        MultiVector { g0: self.g0 * Simd32x4::from(other.g0), g1: self.g1 * Simd32x4::from(other.g0), g2: self.g2 * Simd32x4::from(other.g0), g3: self.g3 * Simd32x4::from(other.g0) }
    }
}

impl RegressiveProduct<Scalar> for MultiVector {
    type Output = Scalar;

    fn regressive_product(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g3[0] * other.g0 }
    }
}

impl OuterProduct<Scalar> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: Scalar) -> MultiVector {
        MultiVector { g0: self.g0 * Simd32x4::from(other.g0), g1: self.g1 * Simd32x4::from(other.g0), g2: self.g2 * Simd32x4::from(other.g0), g3: self.g3 * Simd32x4::from(other.g0) }
    }
}

impl InnerProduct<Scalar> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: Scalar) -> MultiVector {
        MultiVector { g0: self.g0 * Simd32x4::from(other.g0), g1: self.g1 * Simd32x4::from(other.g0), g2: self.g2 * Simd32x4::from(other.g0), g3: self.g3 * Simd32x4::from(other.g0) }
    }
}

impl LeftContraction<Scalar> for MultiVector {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0 }
    }
}

impl RightContraction<Scalar> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Scalar) -> MultiVector {
        MultiVector { g0: self.g0 * Simd32x4::from(other.g0), g1: self.g1 * Simd32x4::from(other.g0), g2: self.g2 * Simd32x4::from(other.g0), g3: self.g3 * Simd32x4::from(other.g0) }
    }
}

impl ScalarProduct<Scalar> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0 }
    }
}

impl Add<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: self.g0 + other.g0, g1: self.g1 + other.g1, g2: self.g2 + other.g2, g3: self.g3 + other.g3 }
    }
}

impl AddAssign<MultiVector> for MultiVector {
    fn add_assign(&mut self, other: MultiVector) {
        *self = (*self).add(other);
    }
}

impl Sub<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: self.g0 - other.g0, g1: self.g1 - other.g1, g2: self.g2 - other.g2, g3: self.g3 - other.g3 }
    }
}

impl SubAssign<MultiVector> for MultiVector {
    fn sub_assign(&mut self, other: MultiVector) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.g2[0]) * other.g2 * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g2, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g2, 2, 3, 0, 1) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g2, 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[1]) * swizzle!(other.g1, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g1, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g1, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.g1[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.g2[0]) * other.g3 + Simd32x4::from(self.g2[1]) * swizzle!(other.g3, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g3, 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g3, 3, 2, 1, 0) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) - Simd32x4::from(self.g3[0]) * other.g2 + Simd32x4::from(self.g3[1]) * swizzle!(other.g2, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g3[2]) * swizzle!(other.g2, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g3[3]) * swizzle!(other.g2, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g2: Simd32x4::from(self.g0[0]) * other.g2 + Simd32x4::from(self.g0[1]) * swizzle!(other.g2, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g2, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g2, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.g2[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]), g3: Simd32x4::from(self.g0[0]) * other.g3 + Simd32x4::from(self.g0[1]) * swizzle!(other.g3, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g3, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g3, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.g1[0]) * other.g2 + Simd32x4::from(self.g1[1]) * swizzle!(other.g2, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g2, 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g2, 3, 2, 1, 0) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) - Simd32x4::from(self.g2[0]) * other.g1 + Simd32x4::from(self.g2[1]) * swizzle!(other.g1, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g1, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g1, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.g3[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g3[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g3[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g3[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) }
    }
}

impl RegressiveProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn regressive_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g3, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g3, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g3, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from(other.g2[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g2, 1, 0, 1, 1) * Simd32x4::from([-1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g2, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g2, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.g2[0]) * other.g1 * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.g2[1]) * Simd32x4::from(other.g1[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g2[2]) * Simd32x4::from(other.g1[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g2[3]) * Simd32x4::from(other.g1[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g3[0]) * other.g0 + Simd32x4::from(self.g3[1]) * Simd32x4::from(other.g0[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g3[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g3[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from(other.g3[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.g1[1]) * swizzle!(other.g3, 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g3, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g3, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g3[0]) * other.g1 + Simd32x4::from(self.g3[1]) * Simd32x4::from(other.g1[1]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g3[2]) * Simd32x4::from(other.g1[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g3[3]) * Simd32x4::from(other.g1[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from(other.g3[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.g0[2]) * swizzle!(other.g1, 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g1, 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g2[0]) * other.g3 + Simd32x4::from(self.g2[2]) * Simd32x4::from(other.g3[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g2[3]) * Simd32x4::from(other.g3[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g3[0]) * other.g2 + Simd32x4::from(self.g3[1]) * Simd32x4::from(other.g2[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g3[2]) * Simd32x4::from(other.g2[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g3[3]) * Simd32x4::from(other.g2[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.g0[0], self.g2[1], self.g0[1], self.g0[1]]) * Simd32x4::from([other.g1[0], other.g3[0], other.g1[3], other.g1[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]), g3: Simd32x4::from(self.g1[2]) * swizzle!(other.g1, 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g1, 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.g3[0]) * other.g3 + Simd32x4::from(self.g3[2]) * Simd32x4::from(other.g3[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g3[3]) * Simd32x4::from(other.g3[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.g1[0], self.g3[1], self.g1[1], self.g1[1]]) * Simd32x4::from([other.g1[0], other.g3[0], other.g1[3], other.g1[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) }
    }
}

impl OuterProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g2[1]) * swizzle!(other.g2, 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g2, 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g2, 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g1[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g1[1]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g3, 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g3, 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g3, 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.g3[1]) * swizzle!(other.g2, 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g3[2]) * swizzle!(other.g2, 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g3[3]) * swizzle!(other.g2, 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + self.g0 * Simd32x4::from(other.g1[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g2: Simd32x4::from(self.g0[0]) * other.g2 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g2[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g2[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g2[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g2, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g3: Simd32x4::from(self.g0[0]) * other.g3 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g3[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g3[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[0]) * other.g2 + Simd32x4::from(self.g1[1]) * Simd32x4::from(other.g2[1]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * Simd32x4::from(other.g2[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[3]) * Simd32x4::from(other.g2[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g2[0]) * Simd32x4::from(other.g1[0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g1, 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g1, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g1, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.g3[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g3[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g3[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g3[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g3, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl InnerProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g2[0]) * other.g2 * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g2, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g2, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g2, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 1, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g1[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g1[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g2[0]) * Simd32x4::from(other.g3[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g3, 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g3, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g3, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) - Simd32x4::from(self.g3[0]) * other.g2 + Simd32x4::from(self.g3[1]) * Simd32x4::from(other.g2[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g3[2]) * Simd32x4::from(other.g2[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g3[3]) * Simd32x4::from(other.g2[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g1, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.g0[0]) * other.g2 + Simd32x4::from(self.g0[2]) * swizzle!(other.g2, 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g2, 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.g2[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g0, 0, 0, 3, 2) * Simd32x4::from([0.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g0, 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g0, 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + swizzle!(self.g0, 0, 1, 1, 1) * swizzle!(other.g2, 0, 0, 3, 2) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]), g3: Simd32x4::from(self.g0[0]) * other.g3 + Simd32x4::from(self.g1[1]) * swizzle!(other.g2, 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g2, 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g2, 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g1, 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g1, 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g1, 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g3[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g3[1]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g3[2]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g3[3]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + self.g0 * Simd32x4::from(other.g3[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) }
    }
}

impl LeftContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g2[0]) * Simd32x4::from(other.g2[0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g2, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g2, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g2, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g1[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g1[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g2[0]) * Simd32x4::from(other.g3[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g3, 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g3, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g3, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g1, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.g0[0]) * other.g2 + Simd32x4::from(self.g2[1]) * swizzle!(other.g0, 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g0, 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g0, 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + self.g0 * Simd32x4::from(other.g2[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g3: Simd32x4::from(self.g0[0]) * other.g3 + Simd32x4::from(self.g2[1]) * swizzle!(other.g1, 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g1, 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g1, 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + self.g0 * Simd32x4::from(other.g3[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) }
    }
}

impl RightContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g2[0]) * other.g2 * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.g2[1]) * Simd32x4::from(other.g2[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g2[2]) * Simd32x4::from(other.g2[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g2[3]) * Simd32x4::from(other.g2[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) - Simd32x4::from(self.g3[0]) * other.g2 + Simd32x4::from(self.g3[1]) * Simd32x4::from(other.g2[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g3[2]) * Simd32x4::from(other.g2[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g3[3]) * Simd32x4::from(other.g2[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.g0[2]) * swizzle!(other.g2, 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g2, 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g2[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g2[2]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g2[3]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.g0[0], self.g2[1], self.g0[1], self.g0[1]]) * Simd32x4::from([other.g2[0], other.g0[0], other.g2[3], other.g2[2]]) * Simd32x4::from([0.0, 1.0, 1.0, -1.0]), g3: Simd32x4::from(self.g1[2]) * swizzle!(other.g2, 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g2, 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.g3[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g3[2]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g3[3]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.g1[0], self.g3[1], self.g1[1], self.g1[1]]) * Simd32x4::from([other.g2[0], other.g0[0], other.g2[3], other.g2[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) }
    }
}

impl ScalarProduct<MultiVector> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] - self.g0[1] * other.g0[1] - self.g0[2] * other.g0[2] - self.g0[3] * other.g0[3] - self.g2[0] * other.g2[0] + self.g2[1] * other.g2[1] + self.g2[2] * other.g2[2] + self.g2[3] * other.g2[3] }
    }
}

impl Into<Rotor> for MultiVector {
    fn into(self) -> Rotor {
        Rotor { g0: self.g0 }
    }
}

impl Add<Rotor> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Rotor) -> MultiVector {
        MultiVector { g0: self.g0 + other.g0, g1: self.g1, g2: self.g2, g3: self.g3 }
    }
}

impl AddAssign<Rotor> for MultiVector {
    fn add_assign(&mut self, other: Rotor) {
        *self = (*self).add(other);
    }
}

impl Sub<Rotor> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Rotor) -> MultiVector {
        MultiVector { g0: self.g0 - other.g0, g1: self.g1, g2: self.g2, g3: self.g3 }
    }
}

impl SubAssign<Rotor> for MultiVector {
    fn sub_assign(&mut self, other: Rotor) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Rotor> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Rotor) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.g1[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]), g2: Simd32x4::from(self.g2[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]), g3: Simd32x4::from(self.g3[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g3[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g3[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g3[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) }
    }
}

impl OuterProduct<Rotor> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: Rotor) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.g1[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.g1 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g2: Simd32x4::from(self.g2[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g2[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g3: Simd32x4::from(self.g3[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g3[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g3[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g3[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl InnerProduct<Rotor> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: Rotor) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 1, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.g2[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g0, 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g0, 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + swizzle!(self.g2, 0, 1, 1, 1) * swizzle!(other.g0, 0, 0, 3, 2) * Simd32x4::from([0.0, 1.0, 1.0, -1.0]), g3: Simd32x4::from(self.g3[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.g3 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl RightContraction<Rotor> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Rotor) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.g2[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.g2 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g3: Simd32x4::from(self.g3[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.g3 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl ScalarProduct<Rotor> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] - self.g0[1] * other.g0[1] - self.g0[2] * other.g0[2] - self.g0[3] * other.g0[3] }
    }
}

impl Into<Point> for MultiVector {
    fn into(self) -> Point {
        Point { g0: Simd32x4::from([self.g2[0], self.g1[1], self.g1[2], self.g1[3]]) }
    }
}

impl Add<Point> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Point) -> MultiVector {
        MultiVector { g0: self.g0, g1: self.g1 + other.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g2: self.g2 + Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g3: self.g3 }
    }
}

impl AddAssign<Point> for MultiVector {
    fn add_assign(&mut self, other: Point) {
        *self = (*self).add(other);
    }
}

impl Sub<Point> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Point) -> MultiVector {
        MultiVector { g0: self.g0, g1: self.g1 - other.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g2: self.g2 - Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g3: self.g3 }
    }
}

impl SubAssign<Point> for MultiVector {
    fn sub_assign(&mut self, other: Point) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Point> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Point) -> MultiVector {
        MultiVector { g0: self.g2 * Simd32x4::from(other.g0[0]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 2, 1) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 3) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g3[1]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.g3[2]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.g3[3]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.g3[0], self.g0[0], self.g0[0], self.g0[0]]) * other.g0 * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]), g2: self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g3: Simd32x4::from(self.g2[0]) * swizzle!(other.g0, 1, 1, 2, 3) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g0, 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g0, 2, 3, 2, 1) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g0, 3, 2, 1, 3) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + self.g1 * Simd32x4::from(other.g0[0]) }
    }
}

impl ScalarProduct<Point> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Point) -> Scalar {
        Scalar { g0: 0.0 - self.g2[0] * other.g0[0] }
    }
}

impl Into<Plane> for MultiVector {
    fn into(self) -> Plane {
        Plane { g0: Simd32x4::from([self.g1[0], self.g2[1], self.g2[2], self.g2[3]]) }
    }
}

impl Add<Plane> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Plane) -> MultiVector {
        MultiVector { g0: self.g0, g1: self.g1 + Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: self.g2 + other.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g3: self.g3 }
    }
}

impl AddAssign<Plane> for MultiVector {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Sub<Plane> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Plane) -> MultiVector {
        MultiVector { g0: self.g0, g1: self.g1 - Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: self.g2 - other.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g3: self.g3 }
    }
}

impl SubAssign<Plane> for MultiVector {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Plane> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Plane) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g2[1]) * swizzle!(other.g0, 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g0, 2, 3, 2, 1) * Simd32x4::from([1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g0, 3, 2, 1, 3) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.g2[0]) * other.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.g3[0]) * swizzle!(other.g0, 1, 1, 2, 3) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g3[1]) * swizzle!(other.g0, 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g3[2]) * swizzle!(other.g0, 2, 3, 2, 1) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g3[3]) * swizzle!(other.g0, 3, 2, 1, 3) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g2: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 2, 1) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 3) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g0[0]) * other.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g3: Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 1, 1, 3, 2) * Simd32x4::from([-1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 3, 2, 1) * Simd32x4::from([-1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 3, 2, 1, 3) * Simd32x4::from([-1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.g2[1]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.g2[2]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.g2[3]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.g2[0], self.g1[0], self.g1[0], self.g1[0]]) * other.g0 * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) }
    }
}

impl ScalarProduct<Plane> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Plane) -> Scalar {
        Scalar { g0: self.g2[1] * other.g0[1] + self.g2[2] * other.g0[2] + self.g2[3] * other.g0[3] }
    }
}

impl Into<Line> for MultiVector {
    fn into(self) -> Line {
        Line { g0: Simd32x3::from([self.g3[1], self.g3[2], self.g3[3]]), g1: Simd32x3::from([self.g0[1], self.g0[2], self.g0[3]]) }
    }
}

impl Add<Line> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Line) -> MultiVector {
        MultiVector { g0: self.g0 + Simd32x4::from([other.g0[0], other.g1[0], other.g1[1], other.g1[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: self.g1, g2: self.g2, g3: self.g3 + Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl AddAssign<Line> for MultiVector {
    fn add_assign(&mut self, other: Line) {
        *self = (*self).add(other);
    }
}

impl Sub<Line> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Line) -> MultiVector {
        MultiVector { g0: self.g0 - Simd32x4::from([other.g0[0], other.g1[0], other.g1[1], other.g1[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: self.g1, g2: self.g2, g3: self.g3 - Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl SubAssign<Line> for MultiVector {
    fn sub_assign(&mut self, other: Line) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Line> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Line) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[1]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[2], other.g1[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * Simd32x4::from([other.g1[1], other.g1[2], other.g1[1], other.g1[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from([other.g1[2], other.g1[1], other.g1[0], other.g1[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[1], other.g1[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.g1[1]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[2], other.g1[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g1[2]) * Simd32x4::from([other.g1[1], other.g1[2], other.g1[1], other.g1[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g1[3]) * Simd32x4::from([other.g1[2], other.g1[1], other.g1[0], other.g1[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g2[0]) * Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.g2[1]) * Simd32x4::from([other.g0[0], other.g0[0], other.g0[2], other.g0[1]]) * Simd32x4::from([-1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.g2[2]) * Simd32x4::from([other.g0[1], other.g0[2], other.g0[1], other.g0[0]]) * Simd32x4::from([-1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.g2[3]) * Simd32x4::from([other.g0[2], other.g0[1], other.g0[0], other.g0[2]]) * Simd32x4::from([-1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[1], other.g1[2]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g2: Simd32x4::from(self.g2[1]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[2], other.g1[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g2[2]) * Simd32x4::from([other.g1[1], other.g1[2], other.g1[1], other.g1[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g2[3]) * Simd32x4::from([other.g1[2], other.g1[1], other.g1[0], other.g1[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g2[0]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[1], other.g1[2]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g3: Simd32x4::from(self.g0[1]) * Simd32x4::from([other.g0[0], other.g0[0], other.g0[2], other.g0[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * Simd32x4::from([other.g0[1], other.g0[2], other.g0[1], other.g0[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from([other.g0[2], other.g0[1], other.g0[0], other.g0[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g3[0]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[1], other.g1[2]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g3[1]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[2], other.g1[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g3[2]) * Simd32x4::from([other.g1[1], other.g1[2], other.g1[1], other.g1[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g3[3]) * Simd32x4::from([other.g1[2], other.g1[1], other.g1[0], other.g1[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl ScalarProduct<Line> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Line) -> Scalar {
        Scalar { g0: 0.0 - self.g0[1] * other.g1[0] - self.g0[2] * other.g1[1] - self.g0[3] * other.g1[2] }
    }
}

impl Into<Translator> for MultiVector {
    fn into(self) -> Translator {
        Translator { g0: Simd32x4::from([self.g0[0], self.g3[1], self.g3[2], self.g3[3]]) }
    }
}

impl Add<Translator> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Translator) -> MultiVector {
        MultiVector { g0: self.g0 + Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.g1, g2: self.g2, g3: self.g3 + other.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl AddAssign<Translator> for MultiVector {
    fn add_assign(&mut self, other: Translator) {
        *self = (*self).add(other);
    }
}

impl Sub<Translator> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Translator) -> MultiVector {
        MultiVector { g0: self.g0 - Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.g1, g2: self.g2, g3: self.g3 - other.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl SubAssign<Translator> for MultiVector {
    fn sub_assign(&mut self, other: Translator) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Translator> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Translator) -> MultiVector {
        MultiVector { g0: self.g0 * Simd32x4::from(other.g0[0]), g1: Simd32x4::from(self.g2[0]) * swizzle!(other.g0, 1, 1, 2, 3) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g0, 1, 1, 3, 2) * Simd32x4::from([-1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g0, 2, 3, 2, 1) * Simd32x4::from([-1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g0, 3, 2, 1, 3) * Simd32x4::from([-1.0, -1.0, 1.0, 0.0]) + self.g1 * Simd32x4::from(other.g0[0]), g2: self.g2 * Simd32x4::from(other.g0[0]), g3: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 2, 1) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 3) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g3[1]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g3[2]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g3[3]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.g3[0], self.g0[0], self.g0[0], self.g0[0]]) * other.g0 }
    }
}

impl OuterProduct<Translator> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: Translator) -> MultiVector {
        MultiVector { g0: self.g0 * Simd32x4::from(other.g0[0]), g1: Simd32x4::from(self.g2[1]) * swizzle!(other.g0, 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g0, 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g0, 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + self.g1 * Simd32x4::from(other.g0[0]), g2: self.g2 * Simd32x4::from(other.g0[0]), g3: Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g3[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g3[1]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g3[2]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g3[3]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g0, 1, 1, 2, 3) }
    }
}

impl InnerProduct<Translator> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: Translator) -> MultiVector {
        MultiVector { g0: self.g0 * Simd32x4::from(other.g0[0]), g1: Simd32x4::from(self.g2[1]) * Simd32x4::from(other.g0[1]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g2[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g2[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + self.g1 * Simd32x4::from(other.g0[0]), g2: self.g2 * Simd32x4::from(other.g0[0]), g3: Simd32x4::from(self.g3[1]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g3[2]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g3[3]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.g3[0], self.g0[0], self.g0[0], self.g0[0]]) * other.g0 }
    }
}

impl RightContraction<Translator> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Translator) -> MultiVector {
        MultiVector { g0: self.g0 * Simd32x4::from(other.g0[0]), g1: self.g1 * Simd32x4::from(other.g0[0]), g2: self.g2 * Simd32x4::from(other.g0[0]), g3: self.g3 * Simd32x4::from(other.g0[0]) }
    }
}

impl ScalarProduct<Translator> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Translator) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] }
    }
}

impl Into<Motor> for MultiVector {
    fn into(self) -> Motor {
        Motor { g0: self.g0, g1: self.g3 }
    }
}

impl Add<Motor> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Motor) -> MultiVector {
        MultiVector { g0: self.g0 + other.g0, g1: self.g1, g2: self.g2, g3: self.g3 + other.g1 }
    }
}

impl AddAssign<Motor> for MultiVector {
    fn add_assign(&mut self, other: Motor) {
        *self = (*self).add(other);
    }
}

impl Sub<Motor> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Motor) -> MultiVector {
        MultiVector { g0: self.g0 - other.g0, g1: self.g1, g2: self.g2, g3: self.g3 - other.g1 }
    }
}

impl SubAssign<Motor> for MultiVector {
    fn sub_assign(&mut self, other: Motor) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Motor> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Motor) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.g1[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.g2[0]) * other.g1 + Simd32x4::from(self.g2[1]) * swizzle!(other.g1, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g1, 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g1, 3, 2, 1, 0) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]), g2: Simd32x4::from(self.g2[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]), g3: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[1]) * swizzle!(other.g1, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g1, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g1, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.g3[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g3[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g3[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g3[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) }
    }
}

impl RegressiveProduct<Motor> for MultiVector {
    type Output = MultiVector;

    fn regressive_product(self, other: Motor) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g1, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g1, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g1, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g3[0]) * other.g0 + Simd32x4::from(self.g3[1]) * Simd32x4::from(other.g0[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g3[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g3[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from(other.g1[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.g1[1]) * swizzle!(other.g1, 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g1, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g1, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from(other.g1[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g2[0]) * other.g1 + Simd32x4::from(self.g2[2]) * Simd32x4::from(other.g1[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g2[3]) * Simd32x4::from(other.g1[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.g1[0], self.g2[1], self.g1[1], self.g1[1]]) * Simd32x4::from([other.g0[0], other.g1[0], other.g0[3], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, -1.0]), g3: Simd32x4::from(self.g3[0]) * other.g1 + self.g3 * Simd32x4::from(other.g1[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl OuterProduct<Motor> for MultiVector {
    type Output = MultiVector;

    fn outer_product(self, other: Motor) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.g1[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g1, 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g1, 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g1, 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + self.g1 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g2: Simd32x4::from(self.g2[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g2[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g3: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g1[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g1[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g3[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g3[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g3[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g3[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g1, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl InnerProduct<Motor> for MultiVector {
    type Output = MultiVector;

    fn inner_product(self, other: Motor) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 1, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g2[0]) * Simd32x4::from(other.g1[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g2[1]) * swizzle!(other.g1, 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g1, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g1, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.g2[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g2[2]) * swizzle!(other.g0, 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g2[3]) * swizzle!(other.g0, 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + swizzle!(self.g2, 0, 1, 1, 1) * swizzle!(other.g0, 0, 0, 3, 2) * Simd32x4::from([0.0, 1.0, 1.0, -1.0]), g3: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g3[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g3[1]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g3[2]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g3[3]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + self.g0 * Simd32x4::from(other.g1[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) }
    }
}

impl RightContraction<Motor> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Motor) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.g2[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.g2 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g3: Simd32x4::from(self.g3[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.g3 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl ScalarProduct<Motor> for MultiVector {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] - self.g0[1] * other.g0[1] - self.g0[2] * other.g0[2] - self.g0[3] * other.g0[3] }
    }
}

impl Into<Branch> for MultiVector {
    fn into(self) -> Branch {
        Branch { g0: Simd32x3::from([self.g3[1], self.g3[2], self.g3[3]]) }
    }
}

impl Add<Branch> for MultiVector {
    type Output = MultiVector;

    fn add(self, other: Branch) -> MultiVector {
        MultiVector { g0: self.g0, g1: self.g1, g2: self.g2, g3: self.g3 + Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl AddAssign<Branch> for MultiVector {
    fn add_assign(&mut self, other: Branch) {
        *self = (*self).add(other);
    }
}

impl Sub<Branch> for MultiVector {
    type Output = MultiVector;

    fn sub(self, other: Branch) -> MultiVector {
        MultiVector { g0: self.g0, g1: self.g1, g2: self.g2, g3: self.g3 - Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl SubAssign<Branch> for MultiVector {
    fn sub_assign(&mut self, other: Branch) {
        *self = (*self).sub(other);
    }
}

impl SquaredMagnitude for MultiVector {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for MultiVector {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { g0: self.squared_magnitude().g0.sqrt() }
    }
}

impl Signum for MultiVector {
    type Output = MultiVector;

    fn signum(self) -> MultiVector {
        self.geometric_product(Scalar { g0: 1.0 / self.magnitude().g0 })
    }
}

impl Inverse for MultiVector {
    type Output = MultiVector;

    fn inverse(self) -> MultiVector {
        self.reversal().geometric_product(Scalar { g0: 1.0 / self.squared_magnitude().g0 })
    }
}

impl Zero for Rotor {
    fn zero() -> Self {
        Rotor { g0: Simd32x4::from(0.0) }
    }
}

impl One for Rotor {
    fn one() -> Self {
        Rotor { g0: Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl Neg for Rotor {
    type Output = Rotor;

    fn neg(self) -> Rotor {
        Rotor { g0: self.g0 * Simd32x4::from(-1.0) }
    }
}

impl Automorphism for Rotor {
    type Output = Rotor;

    fn automorphism(self) -> Rotor {
        Rotor { g0: self.g0 }
    }
}

impl Reversal for Rotor {
    type Output = Rotor;

    fn reversal(self) -> Rotor {
        Rotor { g0: self.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) }
    }
}

impl Conjugation for Rotor {
    type Output = Rotor;

    fn conjugation(self) -> Rotor {
        Rotor { g0: self.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) }
    }
}

impl Into<Scalar> for Rotor {
    fn into(self) -> Scalar {
        Scalar { g0: self.g0[0] }
    }
}

impl Add<Scalar> for Rotor {
    type Output = Rotor;

    fn add(self, other: Scalar) -> Rotor {
        Rotor { g0: self.g0 + Simd32x4::from(other.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl AddAssign<Scalar> for Rotor {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Sub<Scalar> for Rotor {
    type Output = Rotor;

    fn sub(self, other: Scalar) -> Rotor {
        Rotor { g0: self.g0 - Simd32x4::from(other.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl SubAssign<Scalar> for Rotor {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Scalar> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: Scalar) -> Rotor {
        Rotor { g0: self.g0 * Simd32x4::from(other.g0) }
    }
}

impl OuterProduct<Scalar> for Rotor {
    type Output = Rotor;

    fn outer_product(self, other: Scalar) -> Rotor {
        Rotor { g0: self.g0 * Simd32x4::from(other.g0) }
    }
}

impl InnerProduct<Scalar> for Rotor {
    type Output = Rotor;

    fn inner_product(self, other: Scalar) -> Rotor {
        Rotor { g0: self.g0 * Simd32x4::from(other.g0) }
    }
}

impl LeftContraction<Scalar> for Rotor {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0 }
    }
}

impl RightContraction<Scalar> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Scalar) -> Rotor {
        Rotor { g0: self.g0 * Simd32x4::from(other.g0) }
    }
}

impl ScalarProduct<Scalar> for Rotor {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0 }
    }
}

impl Add<MultiVector> for Rotor {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: self.g0 + other.g0, g1: other.g1, g2: other.g2, g3: other.g3 }
    }
}

impl Sub<MultiVector> for Rotor {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: self.g0 - other.g0, g1: Simd32x4::from(0.0) - other.g1, g2: Simd32x4::from(0.0) - other.g2, g3: Simd32x4::from(0.0) - other.g3 }
    }
}

impl GeometricProduct<MultiVector> for Rotor {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[1]) * swizzle!(other.g1, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g1, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g1, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g2: Simd32x4::from(self.g0[0]) * other.g2 + Simd32x4::from(self.g0[1]) * swizzle!(other.g2, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g2, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g2, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g3: Simd32x4::from(self.g0[0]) * other.g3 + Simd32x4::from(self.g0[1]) * swizzle!(other.g3, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g3, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g3, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) }
    }
}

impl OuterProduct<MultiVector> for Rotor {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + self.g0 * Simd32x4::from(other.g1[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g2: Simd32x4::from(self.g0[0]) * other.g2 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g2[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g2[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g2, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g3: Simd32x4::from(self.g0[0]) * other.g3 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g3[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g3[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g3, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl InnerProduct<MultiVector> for Rotor {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 1, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g1[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g1[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g1, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.g0[0]) * other.g2 + Simd32x4::from(self.g0[2]) * swizzle!(other.g2, 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g2, 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + swizzle!(self.g0, 0, 1, 1, 1) * swizzle!(other.g2, 0, 0, 3, 2) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]), g3: Simd32x4::from(self.g0[0]) * other.g3 + self.g0 * Simd32x4::from(other.g3[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) }
    }
}

impl LeftContraction<MultiVector> for Rotor {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g1[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g1[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g1, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.g0[0]) * other.g2 + self.g0 * Simd32x4::from(other.g2[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g3: Simd32x4::from(self.g0[0]) * other.g3 + self.g0 * Simd32x4::from(other.g3[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) }
    }
}

impl ScalarProduct<MultiVector> for Rotor {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] - self.g0[1] * other.g0[1] - self.g0[2] * other.g0[2] - self.g0[3] * other.g0[3] }
    }
}

impl Add<Rotor> for Rotor {
    type Output = Rotor;

    fn add(self, other: Rotor) -> Rotor {
        Rotor { g0: self.g0 + other.g0 }
    }
}

impl AddAssign<Rotor> for Rotor {
    fn add_assign(&mut self, other: Rotor) {
        *self = (*self).add(other);
    }
}

impl Sub<Rotor> for Rotor {
    type Output = Rotor;

    fn sub(self, other: Rotor) -> Rotor {
        Rotor { g0: self.g0 - other.g0 }
    }
}

impl SubAssign<Rotor> for Rotor {
    fn sub_assign(&mut self, other: Rotor) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Rotor> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) }
    }
}

impl OuterProduct<Rotor> for Rotor {
    type Output = Rotor;

    fn outer_product(self, other: Rotor) -> Rotor {
        Rotor { g0: Simd32x4::from(self.g0[0]) * other.g0 + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl InnerProduct<Rotor> for Rotor {
    type Output = Rotor;

    fn inner_product(self, other: Rotor) -> Rotor {
        Rotor { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 1, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) }
    }
}

impl LeftContraction<Rotor> for Rotor {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) }
    }
}

impl RightContraction<Rotor> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Rotor) -> Rotor {
        Rotor { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl ScalarProduct<Rotor> for Rotor {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] - self.g0[1] * other.g0[1] - self.g0[2] * other.g0[2] - self.g0[3] * other.g0[3] }
    }
}

impl OuterProduct<Point> for Rotor {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { g0: Simd32x4::from(self.g0[0]) * other.g0 }
    }
}

impl InnerProduct<Plane> for Rotor {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + swizzle!(self.g0, 0, 0, 1, 1) * swizzle!(other.g0, 0, 0, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) }
    }
}

impl LeftContraction<Plane> for Rotor {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[0]) * other.g0 }
    }
}

impl GeometricProduct<Line> for Rotor {
    type Output = Motor;

    fn geometric_product(self, other: Line) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[1]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[2], other.g1[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * Simd32x4::from([other.g1[1], other.g1[2], other.g1[1], other.g1[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from([other.g1[2], other.g1[1], other.g1[0], other.g1[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[1], other.g1[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.g0[1]) * Simd32x4::from([other.g0[0], other.g0[0], other.g0[2], other.g0[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * Simd32x4::from([other.g0[1], other.g0[2], other.g0[1], other.g0[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from([other.g0[2], other.g0[1], other.g0[0], other.g0[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl RegressiveProduct<Line> for Rotor {
    type Output = Scalar;

    fn regressive_product(self, other: Line) -> Scalar {
        Scalar { g0: self.g0[1] * other.g0[0] + self.g0[2] * other.g0[1] + self.g0[3] * other.g0[2] }
    }
}

impl RightContraction<Line> for Rotor {
    type Output = Scalar;

    fn right_contraction(self, other: Line) -> Scalar {
        Scalar { g0: 0.0 - self.g0[1] * other.g1[0] - self.g0[2] * other.g1[1] - self.g0[3] * other.g1[2] }
    }
}

impl ScalarProduct<Line> for Rotor {
    type Output = Scalar;

    fn scalar_product(self, other: Line) -> Scalar {
        Scalar { g0: 0.0 - self.g0[1] * other.g1[0] - self.g0[2] * other.g1[1] - self.g0[3] * other.g1[2] }
    }
}

impl GeometricProduct<Translator> for Rotor {
    type Output = Motor;

    fn geometric_product(self, other: Translator) -> Motor {
        Motor { g0: self.g0 * Simd32x4::from(other.g0[0]), g1: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 2, 1) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 3) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g0[0]) * other.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl RegressiveProduct<Translator> for Rotor {
    type Output = Scalar;

    fn regressive_product(self, other: Translator) -> Scalar {
        Scalar { g0: self.g0[1] * other.g0[1] + self.g0[2] * other.g0[2] + self.g0[3] * other.g0[3] }
    }
}

impl OuterProduct<Translator> for Rotor {
    type Output = Motor;

    fn outer_product(self, other: Translator) -> Motor {
        Motor { g0: self.g0 * Simd32x4::from(other.g0[0]), g1: Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g0, 1, 1, 2, 3) }
    }
}

impl LeftContraction<Translator> for Rotor {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { g0: Simd32x4::from(self.g0[0]) * other.g0 }
    }
}

impl RightContraction<Translator> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Translator) -> Rotor {
        Rotor { g0: self.g0 * Simd32x4::from(other.g0[0]) }
    }
}

impl ScalarProduct<Translator> for Rotor {
    type Output = Scalar;

    fn scalar_product(self, other: Translator) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] }
    }
}

impl Add<Motor> for Rotor {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { g0: self.g0 + other.g0, g1: other.g1 }
    }
}

impl Sub<Motor> for Rotor {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { g0: self.g0 - other.g0, g1: Simd32x4::from(0.0) - other.g1 }
    }
}

impl GeometricProduct<Motor> for Rotor {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[1]) * swizzle!(other.g1, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g1, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g1, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) }
    }
}

impl RegressiveProduct<Motor> for Rotor {
    type Output = Rotor;

    fn regressive_product(self, other: Motor) -> Rotor {
        Rotor { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g1, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g1, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g1, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from(other.g1[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl OuterProduct<Motor> for Rotor {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0 + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g1[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g1[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g1, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl InnerProduct<Motor> for Rotor {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 1, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + self.g0 * Simd32x4::from(other.g1[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) }
    }
}

impl LeftContraction<Motor> for Rotor {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + self.g0 * Simd32x4::from(other.g1[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) }
    }
}

impl RightContraction<Motor> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Motor) -> Rotor {
        Rotor { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl ScalarProduct<Motor> for Rotor {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] - self.g0[1] * other.g0[1] - self.g0[2] * other.g0[2] - self.g0[3] * other.g0[3] }
    }
}

impl RegressiveProduct<Branch> for Rotor {
    type Output = Scalar;

    fn regressive_product(self, other: Branch) -> Scalar {
        Scalar { g0: self.g0[1] * other.g0[0] + self.g0[2] * other.g0[1] + self.g0[3] * other.g0[2] }
    }
}

impl InnerProduct<Branch> for Rotor {
    type Output = Branch;

    fn inner_product(self, other: Branch) -> Branch {
        Branch { g0: Simd32x3::from(self.g0[0]) * other.g0 }
    }
}

impl LeftContraction<Branch> for Rotor {
    type Output = Branch;

    fn left_contraction(self, other: Branch) -> Branch {
        Branch { g0: Simd32x3::from(self.g0[0]) * other.g0 }
    }
}

impl SquaredMagnitude for Rotor {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Rotor {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { g0: self.squared_magnitude().g0.sqrt() }
    }
}

impl Signum for Rotor {
    type Output = Rotor;

    fn signum(self) -> Rotor {
        self.geometric_product(Scalar { g0: 1.0 / self.magnitude().g0 })
    }
}

impl Inverse for Rotor {
    type Output = Rotor;

    fn inverse(self) -> Rotor {
        self.reversal().geometric_product(Scalar { g0: 1.0 / self.squared_magnitude().g0 })
    }
}

impl Zero for Point {
    fn zero() -> Self {
        Point { g0: Simd32x4::from(0.0) }
    }
}

impl One for Point {
    fn one() -> Self {
        Point { g0: Simd32x4::from(0.0) }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point { g0: self.g0 * Simd32x4::from(-1.0) }
    }
}

impl Automorphism for Point {
    type Output = Point;

    fn automorphism(self) -> Point {
        Point { g0: self.g0 * Simd32x4::from(-1.0) }
    }
}

impl Reversal for Point {
    type Output = Point;

    fn reversal(self) -> Point {
        Point { g0: self.g0 * Simd32x4::from(-1.0) }
    }
}

impl Conjugation for Point {
    type Output = Point;

    fn conjugation(self) -> Point {
        Point { g0: self.g0 }
    }
}

impl Dual for Point {
    type Output = Plane;

    fn dual(self) -> Plane {
        Plane { g0: self.g0 * Simd32x4::from(-1.0) }
    }
}

impl GeometricProduct<Scalar> for Point {
    type Output = Point;

    fn geometric_product(self, other: Scalar) -> Point {
        Point { g0: self.g0 * Simd32x4::from(other.g0) }
    }
}

impl OuterProduct<Scalar> for Point {
    type Output = Point;

    fn outer_product(self, other: Scalar) -> Point {
        Point { g0: self.g0 * Simd32x4::from(other.g0) }
    }
}

impl InnerProduct<Scalar> for Point {
    type Output = Point;

    fn inner_product(self, other: Scalar) -> Point {
        Point { g0: self.g0 * Simd32x4::from(other.g0) }
    }
}

impl RightContraction<Scalar> for Point {
    type Output = Point;

    fn right_contraction(self, other: Scalar) -> Point {
        Point { g0: self.g0 * Simd32x4::from(other.g0) }
    }
}

impl Add<MultiVector> for Point {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: other.g0, g1: self.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.g1, g2: Simd32x4::from(self.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.g2, g3: other.g3 }
    }
}

impl Sub<MultiVector> for Point {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(0.0) - other.g0, g1: self.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.g1, g2: Simd32x4::from(self.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.g2, g3: Simd32x4::from(0.0) - other.g3 }
    }
}

impl GeometricProduct<MultiVector> for Point {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g2 * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.g0[0]) * other.g3 + Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]), g2: Simd32x4::from(self.g0[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g3: Simd32x4::from(0.0) - Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[1]) * swizzle!(other.g2, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g2, 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g2, 3, 2, 1, 0) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) }
    }
}

impl ScalarProduct<MultiVector> for Point {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { g0: 0.0 - self.g0[0] * other.g2[0] }
    }
}

impl OuterProduct<Rotor> for Point {
    type Output = Point;

    fn outer_product(self, other: Rotor) -> Point {
        Point { g0: self.g0 * Simd32x4::from(other.g0[0]) }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { g0: self.g0 + other.g0 }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point) {
        *self = (*self).add(other);
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point { g0: self.g0 - other.g0 }
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, other: Point) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Point> for Point {
    type Output = Translator;

    fn geometric_product(self, other: Point) -> Translator {
        Translator { g0: Simd32x4::from(0.0) - Simd32x4::from(self.g0[0]) * other.g0 + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl RegressiveProduct<Point> for Point {
    type Output = Line;

    fn regressive_product(self, other: Point) -> Line {
        Line { g0: Simd32x3::from(self.g0[2]) * Simd32x3::from([other.g0[3], other.g0[3], other.g0[1]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.g0[3]) * Simd32x3::from([other.g0[2], other.g0[1], other.g0[2]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from([self.g0[0], self.g0[1], self.g0[1]]) * Simd32x3::from([other.g0[0], other.g0[3], other.g0[2]]) * Simd32x3::from([0.0, -1.0, 1.0]), g1: Simd32x3::from(self.g0[0]) * Simd32x3::from([other.g0[1], other.g0[2], other.g0[3]]) + Simd32x3::from([self.g0[1], self.g0[2], self.g0[3]]) * Simd32x3::from(other.g0[0]) * Simd32x3::from(-1.0) }
    }
}

impl InnerProduct<Point> for Point {
    type Output = Scalar;

    fn inner_product(self, other: Point) -> Scalar {
        Scalar { g0: 0.0 - self.g0[0] * other.g0[0] }
    }
}

impl LeftContraction<Point> for Point {
    type Output = Scalar;

    fn left_contraction(self, other: Point) -> Scalar {
        Scalar { g0: 0.0 - self.g0[0] * other.g0[0] }
    }
}

impl RightContraction<Point> for Point {
    type Output = Scalar;

    fn right_contraction(self, other: Point) -> Scalar {
        Scalar { g0: 0.0 - self.g0[0] * other.g0[0] }
    }
}

impl ScalarProduct<Point> for Point {
    type Output = Scalar;

    fn scalar_product(self, other: Point) -> Scalar {
        Scalar { g0: 0.0 - self.g0[0] * other.g0[0] }
    }
}

impl RegressiveProduct<Plane> for Point {
    type Output = Scalar;

    fn regressive_product(self, other: Plane) -> Scalar {
        Scalar { g0: 0.0 - self.g0[0] * other.g0[0] - self.g0[1] * other.g0[1] - self.g0[2] * other.g0[2] - self.g0[3] * other.g0[3] }
    }
}

impl InnerProduct<Plane> for Point {
    type Output = Line;

    fn inner_product(self, other: Plane) -> Line {
        Line { g0: Simd32x3::from(self.g0[2]) * Simd32x3::from([other.g0[3], other.g0[3], other.g0[1]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.g0[3]) * Simd32x3::from([other.g0[2], other.g0[1], other.g0[2]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from([self.g0[0], self.g0[1], self.g0[1]]) * Simd32x3::from([other.g0[0], other.g0[3], other.g0[2]]) * Simd32x3::from([0.0, -1.0, 1.0]), g1: Simd32x3::from(self.g0[0]) * Simd32x3::from([other.g0[1], other.g0[2], other.g0[3]]) }
    }
}

impl RightContraction<Plane> for Point {
    type Output = Line;

    fn right_contraction(self, other: Plane) -> Line {
        Line { g0: Simd32x3::from(self.g0[2]) * Simd32x3::from([other.g0[3], other.g0[3], other.g0[1]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.g0[3]) * Simd32x3::from([other.g0[2], other.g0[1], other.g0[2]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from([self.g0[0], self.g0[1], self.g0[1]]) * Simd32x3::from([other.g0[0], other.g0[3], other.g0[2]]) * Simd32x3::from([0.0, -1.0, 1.0]), g1: Simd32x3::from(self.g0[0]) * Simd32x3::from([other.g0[1], other.g0[2], other.g0[3]]) }
    }
}

impl RegressiveProduct<Line> for Point {
    type Output = Plane;

    fn regressive_product(self, other: Line) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[1]) * Simd32x4::from([other.g0[0], other.g0[0], other.g1[2], other.g1[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * Simd32x4::from([other.g0[1], other.g1[2], other.g0[1], other.g1[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from([other.g0[2], other.g1[1], other.g1[0], other.g0[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl InnerProduct<Line> for Point {
    type Output = Plane;

    fn inner_product(self, other: Line) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g1[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g1[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[1], other.g1[2]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) }
    }
}

impl RightContraction<Line> for Point {
    type Output = Plane;

    fn right_contraction(self, other: Line) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g1[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g1[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[1], other.g1[2]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) }
    }
}

impl GeometricProduct<Translator> for Point {
    type Output = Point;

    fn geometric_product(self, other: Translator) -> Point {
        Point { g0: Simd32x4::from(self.g0[0]) * other.g0 + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl RegressiveProduct<Translator> for Point {
    type Output = Plane;

    fn regressive_product(self, other: Translator) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g0, 1, 1, 2, 3) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) }
    }
}

impl OuterProduct<Translator> for Point {
    type Output = Point;

    fn outer_product(self, other: Translator) -> Point {
        Point { g0: self.g0 * Simd32x4::from(other.g0[0]) }
    }
}

impl InnerProduct<Translator> for Point {
    type Output = Point;

    fn inner_product(self, other: Translator) -> Point {
        Point { g0: self.g0 * Simd32x4::from(other.g0[0]) }
    }
}

impl RightContraction<Translator> for Point {
    type Output = Point;

    fn right_contraction(self, other: Translator) -> Point {
        Point { g0: self.g0 * Simd32x4::from(other.g0[0]) }
    }
}

impl OuterProduct<Motor> for Point {
    type Output = Point;

    fn outer_product(self, other: Motor) -> Point {
        Point { g0: self.g0 * Simd32x4::from(other.g0[0]) }
    }
}

impl RegressiveProduct<Branch> for Point {
    type Output = Plane;

    fn regressive_product(self, other: Branch) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[1]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) }
    }
}

impl SquaredMagnitude for Point {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Point {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { g0: self.squared_magnitude().g0.sqrt() }
    }
}

impl Signum for Point {
    type Output = Point;

    fn signum(self) -> Point {
        self.geometric_product(Scalar { g0: 1.0 / self.magnitude().g0 })
    }
}

impl Inverse for Point {
    type Output = Point;

    fn inverse(self) -> Point {
        self.reversal().geometric_product(Scalar { g0: 1.0 / self.squared_magnitude().g0 })
    }
}

impl Zero for Plane {
    fn zero() -> Self {
        Plane { g0: Simd32x4::from(0.0) }
    }
}

impl One for Plane {
    fn one() -> Self {
        Plane { g0: Simd32x4::from(0.0) }
    }
}

impl Neg for Plane {
    type Output = Plane;

    fn neg(self) -> Plane {
        Plane { g0: self.g0 * Simd32x4::from(-1.0) }
    }
}

impl Automorphism for Plane {
    type Output = Plane;

    fn automorphism(self) -> Plane {
        Plane { g0: self.g0 * Simd32x4::from(-1.0) }
    }
}

impl Reversal for Plane {
    type Output = Plane;

    fn reversal(self) -> Plane {
        Plane { g0: self.g0 }
    }
}

impl Conjugation for Plane {
    type Output = Plane;

    fn conjugation(self) -> Plane {
        Plane { g0: self.g0 * Simd32x4::from(-1.0) }
    }
}

impl Dual for Plane {
    type Output = Point;

    fn dual(self) -> Point {
        Point { g0: self.g0 }
    }
}

impl GeometricProduct<Scalar> for Plane {
    type Output = Plane;

    fn geometric_product(self, other: Scalar) -> Plane {
        Plane { g0: self.g0 * Simd32x4::from(other.g0) }
    }
}

impl OuterProduct<Scalar> for Plane {
    type Output = Plane;

    fn outer_product(self, other: Scalar) -> Plane {
        Plane { g0: self.g0 * Simd32x4::from(other.g0) }
    }
}

impl InnerProduct<Scalar> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Scalar) -> Plane {
        Plane { g0: self.g0 * Simd32x4::from(other.g0) }
    }
}

impl RightContraction<Scalar> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Scalar) -> Plane {
        Plane { g0: self.g0 * Simd32x4::from(other.g0) }
    }
}

impl Add<MultiVector> for Plane {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: other.g0, g1: Simd32x4::from(self.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.g1, g2: self.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.g2, g3: other.g3 }
    }
}

impl Sub<MultiVector> for Plane {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(0.0) - other.g0, g1: Simd32x4::from(self.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.g1, g2: self.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.g2, g3: Simd32x4::from(0.0) - other.g3 }
    }
}

impl GeometricProduct<MultiVector> for Plane {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g2, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g2, 2, 3, 0, 1) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g2, 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]), g1: Simd32x4::from(self.g0[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g0[1]) * swizzle!(other.g3, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g3, 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g3, 3, 2, 1, 0) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]), g2: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]), g3: Simd32x4::from(self.g0[0]) * other.g2 + Simd32x4::from(self.g0[1]) * swizzle!(other.g1, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g1, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g1, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) }
    }
}

impl ScalarProduct<MultiVector> for Plane {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { g0: self.g0[1] * other.g2[1] + self.g0[2] * other.g2[2] + self.g0[3] * other.g2[3] }
    }
}

impl InnerProduct<Rotor> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Rotor) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) + swizzle!(self.g0, 0, 1, 1, 1) * swizzle!(other.g0, 0, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) }
    }
}

impl RightContraction<Rotor> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Rotor) -> Plane {
        Plane { g0: self.g0 * Simd32x4::from(other.g0[0]) }
    }
}

impl RegressiveProduct<Point> for Plane {
    type Output = Scalar;

    fn regressive_product(self, other: Point) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] + self.g0[1] * other.g0[1] + self.g0[2] * other.g0[2] + self.g0[3] * other.g0[3] }
    }
}

impl InnerProduct<Point> for Plane {
    type Output = Line;

    fn inner_product(self, other: Point) -> Line {
        Line { g0: Simd32x3::from(self.g0[2]) * Simd32x3::from([other.g0[3], other.g0[3], other.g0[1]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.g0[3]) * Simd32x3::from([other.g0[2], other.g0[1], other.g0[2]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from([self.g0[0], self.g0[1], self.g0[1]]) * Simd32x3::from([other.g0[0], other.g0[3], other.g0[2]]) * Simd32x3::from([0.0, 1.0, -1.0]), g1: Simd32x3::from([self.g0[1], self.g0[2], self.g0[3]]) * Simd32x3::from(other.g0[0]) }
    }
}

impl LeftContraction<Point> for Plane {
    type Output = Line;

    fn left_contraction(self, other: Point) -> Line {
        Line { g0: Simd32x3::from(self.g0[2]) * Simd32x3::from([other.g0[3], other.g0[3], other.g0[1]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.g0[3]) * Simd32x3::from([other.g0[2], other.g0[1], other.g0[2]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from([self.g0[0], self.g0[1], self.g0[1]]) * Simd32x3::from([other.g0[0], other.g0[3], other.g0[2]]) * Simd32x3::from([0.0, 1.0, -1.0]), g1: Simd32x3::from([self.g0[1], self.g0[2], self.g0[3]]) * Simd32x3::from(other.g0[0]) }
    }
}

impl Add<Plane> for Plane {
    type Output = Plane;

    fn add(self, other: Plane) -> Plane {
        Plane { g0: self.g0 + other.g0 }
    }
}

impl AddAssign<Plane> for Plane {
    fn add_assign(&mut self, other: Plane) {
        *self = (*self).add(other);
    }
}

impl Sub<Plane> for Plane {
    type Output = Plane;

    fn sub(self, other: Plane) -> Plane {
        Plane { g0: self.g0 - other.g0 }
    }
}

impl SubAssign<Plane> for Plane {
    fn sub_assign(&mut self, other: Plane) {
        *self = (*self).sub(other);
    }
}

impl OuterProduct<Plane> for Plane {
    type Output = Line;

    fn outer_product(self, other: Plane) -> Line {
        Line { g0: Simd32x3::from(self.g0[0]) * Simd32x3::from([other.g0[1], other.g0[2], other.g0[3]]) + Simd32x3::from([self.g0[1], self.g0[2], self.g0[3]]) * Simd32x3::from(other.g0[0]) * Simd32x3::from(-1.0), g1: Simd32x3::from(self.g0[2]) * Simd32x3::from([other.g0[3], other.g0[3], other.g0[1]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.g0[3]) * Simd32x3::from([other.g0[2], other.g0[1], other.g0[2]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from([self.g0[0], self.g0[1], self.g0[1]]) * Simd32x3::from([other.g0[0], other.g0[3], other.g0[2]]) * Simd32x3::from([0.0, -1.0, 1.0]) }
    }
}

impl InnerProduct<Plane> for Plane {
    type Output = Scalar;

    fn inner_product(self, other: Plane) -> Scalar {
        Scalar { g0: self.g0[1] * other.g0[1] + self.g0[2] * other.g0[2] + self.g0[3] * other.g0[3] }
    }
}

impl LeftContraction<Plane> for Plane {
    type Output = Scalar;

    fn left_contraction(self, other: Plane) -> Scalar {
        Scalar { g0: self.g0[1] * other.g0[1] + self.g0[2] * other.g0[2] + self.g0[3] * other.g0[3] }
    }
}

impl RightContraction<Plane> for Plane {
    type Output = Scalar;

    fn right_contraction(self, other: Plane) -> Scalar {
        Scalar { g0: self.g0[1] * other.g0[1] + self.g0[2] * other.g0[2] + self.g0[3] * other.g0[3] }
    }
}

impl ScalarProduct<Plane> for Plane {
    type Output = Scalar;

    fn scalar_product(self, other: Plane) -> Scalar {
        Scalar { g0: self.g0[1] * other.g0[1] + self.g0[2] * other.g0[2] + self.g0[3] * other.g0[3] }
    }
}

impl OuterProduct<Line> for Plane {
    type Output = Point;

    fn outer_product(self, other: Line) -> Point {
        Point { g0: Simd32x4::from(self.g0[1]) * Simd32x4::from([other.g1[0], other.g1[0], other.g0[2], other.g0[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.g0[2]) * Simd32x4::from([other.g1[1], other.g0[2], other.g1[1], other.g0[0]]) * Simd32x4::from([1.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from([other.g1[2], other.g0[1], other.g0[0], other.g1[2]]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[1], other.g1[2]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) }
    }
}

impl InnerProduct<Line> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Line) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[2]) * Simd32x4::from([other.g0[1], other.g1[2], other.g0[1], other.g1[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from([other.g0[2], other.g1[1], other.g1[0], other.g0[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + swizzle!(self.g0, 1, 0, 1, 1) * Simd32x4::from([other.g0[0], other.g0[0], other.g1[2], other.g1[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) }
    }
}

impl LeftContraction<Line> for Plane {
    type Output = Plane;

    fn left_contraction(self, other: Line) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[2]) * Simd32x4::from([other.g0[1], other.g1[2], other.g0[1], other.g1[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from([other.g0[2], other.g1[1], other.g1[0], other.g0[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + swizzle!(self.g0, 1, 0, 1, 1) * Simd32x4::from([other.g0[0], other.g0[0], other.g1[2], other.g1[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) }
    }
}

impl InnerProduct<Translator> for Plane {
    type Output = Plane;

    fn inner_product(self, other: Translator) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl RightContraction<Translator> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Translator) -> Plane {
        Plane { g0: self.g0 * Simd32x4::from(other.g0[0]) }
    }
}

impl RegressiveProduct<Motor> for Plane {
    type Output = Plane;

    fn regressive_product(self, other: Motor) -> Plane {
        Plane { g0: self.g0 * Simd32x4::from(other.g1[0]) }
    }
}

impl RightContraction<Motor> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Motor) -> Plane {
        Plane { g0: self.g0 * Simd32x4::from(other.g0[0]) }
    }
}

impl SquaredMagnitude for Plane {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Plane {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { g0: self.squared_magnitude().g0.sqrt() }
    }
}

impl Signum for Plane {
    type Output = Plane;

    fn signum(self) -> Plane {
        self.geometric_product(Scalar { g0: 1.0 / self.magnitude().g0 })
    }
}

impl Inverse for Plane {
    type Output = Plane;

    fn inverse(self) -> Plane {
        self.reversal().geometric_product(Scalar { g0: 1.0 / self.squared_magnitude().g0 })
    }
}

impl Zero for Line {
    fn zero() -> Self {
        Line { g0: Simd32x3::from(0.0), g1: Simd32x3::from(0.0) }
    }
}

impl One for Line {
    fn one() -> Self {
        Line { g0: Simd32x3::from(0.0), g1: Simd32x3::from(0.0) }
    }
}

impl Neg for Line {
    type Output = Line;

    fn neg(self) -> Line {
        Line { g0: self.g0 * Simd32x3::from(-1.0), g1: self.g1 * Simd32x3::from(-1.0) }
    }
}

impl Automorphism for Line {
    type Output = Line;

    fn automorphism(self) -> Line {
        Line { g0: self.g0, g1: self.g1 }
    }
}

impl Reversal for Line {
    type Output = Line;

    fn reversal(self) -> Line {
        Line { g0: self.g0 * Simd32x3::from(-1.0), g1: self.g1 * Simd32x3::from(-1.0) }
    }
}

impl Conjugation for Line {
    type Output = Line;

    fn conjugation(self) -> Line {
        Line { g0: self.g0 * Simd32x3::from(-1.0), g1: self.g1 * Simd32x3::from(-1.0) }
    }
}

impl Dual for Line {
    type Output = Line;

    fn dual(self) -> Line {
        Line { g0: self.g1, g1: self.g0 }
    }
}

impl GeometricProduct<Scalar> for Line {
    type Output = Line;

    fn geometric_product(self, other: Scalar) -> Line {
        Line { g0: self.g0 * Simd32x3::from(other.g0), g1: self.g1 * Simd32x3::from(other.g0) }
    }
}

impl OuterProduct<Scalar> for Line {
    type Output = Line;

    fn outer_product(self, other: Scalar) -> Line {
        Line { g0: self.g0 * Simd32x3::from(other.g0), g1: self.g1 * Simd32x3::from(other.g0) }
    }
}

impl InnerProduct<Scalar> for Line {
    type Output = Line;

    fn inner_product(self, other: Scalar) -> Line {
        Line { g0: self.g0 * Simd32x3::from(other.g0), g1: self.g1 * Simd32x3::from(other.g0) }
    }
}

impl RightContraction<Scalar> for Line {
    type Output = Line;

    fn right_contraction(self, other: Scalar) -> Line {
        Line { g0: self.g0 * Simd32x3::from(other.g0), g1: self.g1 * Simd32x3::from(other.g0) }
    }
}

impl Add<MultiVector> for Line {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from([self.g0[0], self.g1[0], self.g1[1], self.g1[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.g0, g1: other.g1, g2: other.g2, g3: Simd32x4::from([self.g0[0], self.g0[0], self.g0[1], self.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.g3 }
    }
}

impl Sub<MultiVector> for Line {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from([self.g0[0], self.g1[0], self.g1[1], self.g1[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.g0, g1: Simd32x4::from(0.0) - other.g1, g2: Simd32x4::from(0.0) - other.g2, g3: Simd32x4::from([self.g0[0], self.g0[0], self.g0[1], self.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.g3 }
    }
}

impl GeometricProduct<MultiVector> for Line {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g1[0]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.g0[0]) * swizzle!(other.g2, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[1]) * swizzle!(other.g2, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g2, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.g1[0]) * swizzle!(other.g1, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g1, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g1, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g2: Simd32x4::from(self.g1[0]) * swizzle!(other.g2, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g2, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g2, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g3: Simd32x4::from(self.g0[0]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.g1[0]) * swizzle!(other.g3, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g3, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g3, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) }
    }
}

impl ScalarProduct<MultiVector> for Line {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { g0: 0.0 - self.g1[0] * other.g0[1] - self.g1[1] * other.g0[2] - self.g1[2] * other.g0[3] }
    }
}

impl GeometricProduct<Rotor> for Line {
    type Output = Motor;

    fn geometric_product(self, other: Rotor) -> Motor {
        Motor { g0: Simd32x4::from(self.g1[0]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.g0[0]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) }
    }
}

impl RegressiveProduct<Rotor> for Line {
    type Output = Scalar;

    fn regressive_product(self, other: Rotor) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[1] + self.g0[1] * other.g0[2] + self.g0[2] * other.g0[3] }
    }
}

impl LeftContraction<Rotor> for Line {
    type Output = Scalar;

    fn left_contraction(self, other: Rotor) -> Scalar {
        Scalar { g0: 0.0 - self.g1[0] * other.g0[1] - self.g1[1] * other.g0[2] - self.g1[2] * other.g0[3] }
    }
}

impl ScalarProduct<Rotor> for Line {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { g0: 0.0 - self.g1[0] * other.g0[1] - self.g1[1] * other.g0[2] - self.g1[2] * other.g0[3] }
    }
}

impl RegressiveProduct<Point> for Line {
    type Output = Plane;

    fn regressive_product(self, other: Point) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from([self.g0[0], self.g0[0], self.g1[0], self.g1[0]]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]) }
    }
}

impl InnerProduct<Point> for Line {
    type Output = Plane;

    fn inner_product(self, other: Point) -> Plane {
        Plane { g0: Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.g1[0]) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) }
    }
}

impl LeftContraction<Point> for Line {
    type Output = Plane;

    fn left_contraction(self, other: Point) -> Plane {
        Plane { g0: Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.g1[0]) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) }
    }
}

impl OuterProduct<Plane> for Line {
    type Output = Point;

    fn outer_product(self, other: Plane) -> Point {
        Point { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.g1[0], self.g1[0], self.g0[0], self.g0[0]]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) }
    }
}

impl InnerProduct<Plane> for Line {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[1]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from([self.g0[0], self.g0[0], self.g1[0], self.g1[0]]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) }
    }
}

impl RightContraction<Plane> for Line {
    type Output = Plane;

    fn right_contraction(self, other: Plane) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[1]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from([self.g0[0], self.g0[0], self.g1[0], self.g1[0]]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) }
    }
}

impl Add<Line> for Line {
    type Output = Line;

    fn add(self, other: Line) -> Line {
        Line { g0: self.g0 + other.g0, g1: self.g1 + other.g1 }
    }
}

impl AddAssign<Line> for Line {
    fn add_assign(&mut self, other: Line) {
        *self = (*self).add(other);
    }
}

impl Sub<Line> for Line {
    type Output = Line;

    fn sub(self, other: Line) -> Line {
        Line { g0: self.g0 - other.g0, g1: self.g1 - other.g1 }
    }
}

impl SubAssign<Line> for Line {
    fn sub_assign(&mut self, other: Line) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Line> for Line {
    type Output = Motor;

    fn geometric_product(self, other: Line) -> Motor {
        Motor { g0: Simd32x4::from(self.g1[1]) * Simd32x4::from([other.g1[1], other.g1[2], other.g1[1], other.g1[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g1[2]) * Simd32x4::from([other.g1[2], other.g1[1], other.g1[0], other.g1[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[2], other.g1[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]), g1: Simd32x4::from(self.g0[1]) * Simd32x4::from([other.g1[1], other.g1[2], other.g1[1], other.g1[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[2]) * Simd32x4::from([other.g1[2], other.g1[1], other.g1[0], other.g1[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from([other.g0[0], other.g0[0], other.g0[2], other.g0[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g1[1]) * Simd32x4::from([other.g0[1], other.g0[2], other.g0[1], other.g0[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g1[2]) * Simd32x4::from([other.g0[2], other.g0[1], other.g0[0], other.g0[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[2], other.g1[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) }
    }
}

impl RegressiveProduct<Line> for Line {
    type Output = Scalar;

    fn regressive_product(self, other: Line) -> Scalar {
        Scalar { g0: self.g0[0] * other.g1[0] + self.g0[1] * other.g1[1] + self.g0[2] * other.g1[2] + self.g1[0] * other.g0[0] + self.g1[1] * other.g0[1] + self.g1[2] * other.g0[2] }
    }
}

impl InnerProduct<Line> for Line {
    type Output = Scalar;

    fn inner_product(self, other: Line) -> Scalar {
        Scalar { g0: 0.0 - self.g1[0] * other.g1[0] - self.g1[1] * other.g1[1] - self.g1[2] * other.g1[2] }
    }
}

impl LeftContraction<Line> for Line {
    type Output = Scalar;

    fn left_contraction(self, other: Line) -> Scalar {
        Scalar { g0: 0.0 - self.g1[0] * other.g1[0] - self.g1[1] * other.g1[1] - self.g1[2] * other.g1[2] }
    }
}

impl RightContraction<Line> for Line {
    type Output = Scalar;

    fn right_contraction(self, other: Line) -> Scalar {
        Scalar { g0: 0.0 - self.g1[0] * other.g1[0] - self.g1[1] * other.g1[1] - self.g1[2] * other.g1[2] }
    }
}

impl ScalarProduct<Line> for Line {
    type Output = Scalar;

    fn scalar_product(self, other: Line) -> Scalar {
        Scalar { g0: 0.0 - self.g1[0] * other.g1[0] - self.g1[1] * other.g1[1] - self.g1[2] * other.g1[2] }
    }
}

impl RegressiveProduct<Translator> for Line {
    type Output = Scalar;

    fn regressive_product(self, other: Translator) -> Scalar {
        Scalar { g0: self.g1[0] * other.g0[1] + self.g1[1] * other.g0[2] + self.g1[2] * other.g0[3] }
    }
}

impl InnerProduct<Translator> for Line {
    type Output = Line;

    fn inner_product(self, other: Translator) -> Line {
        Line { g0: self.g0 * Simd32x3::from(other.g0[0]), g1: self.g1 * Simd32x3::from(other.g0[0]) }
    }
}

impl RightContraction<Translator> for Line {
    type Output = Line;

    fn right_contraction(self, other: Translator) -> Line {
        Line { g0: self.g0 * Simd32x3::from(other.g0[0]), g1: self.g1 * Simd32x3::from(other.g0[0]) }
    }
}

impl Add<Motor> for Line {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from([self.g0[0], self.g1[0], self.g1[1], self.g1[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.g0, g1: Simd32x4::from([self.g0[0], self.g0[0], self.g0[1], self.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.g1 }
    }
}

impl Sub<Motor> for Line {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from([self.g0[0], self.g1[0], self.g1[1], self.g1[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.g0, g1: Simd32x4::from([self.g0[0], self.g0[0], self.g0[1], self.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.g1 }
    }
}

impl GeometricProduct<Motor> for Line {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g1[0]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.g0[0]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) + Simd32x4::from(self.g1[0]) * swizzle!(other.g1, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g1, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g1, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) }
    }
}

impl LeftContraction<Motor> for Line {
    type Output = Translator;

    fn left_contraction(self, other: Motor) -> Translator {
        Translator { g0: Simd32x4::from(self.g1[1]) * Simd32x4::from([other.g0[2], other.g0[2], other.g1[0], other.g0[2]]) * Simd32x4::from([-1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.g1[2]) * Simd32x4::from([other.g0[3], other.g0[3], other.g0[3], other.g1[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from([other.g0[1], other.g1[0], other.g0[0], other.g0[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 0.0]) }
    }
}

impl ScalarProduct<Motor> for Line {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { g0: 0.0 - self.g1[0] * other.g0[1] - self.g1[1] * other.g0[2] - self.g1[2] * other.g0[3] }
    }
}

impl Into<Branch> for Line {
    fn into(self) -> Branch {
        Branch { g0: self.g0 }
    }
}

impl Add<Branch> for Line {
    type Output = Line;

    fn add(self, other: Branch) -> Line {
        Line { g0: self.g0 + other.g0, g1: self.g1 }
    }
}

impl AddAssign<Branch> for Line {
    fn add_assign(&mut self, other: Branch) {
        *self = (*self).add(other);
    }
}

impl Sub<Branch> for Line {
    type Output = Line;

    fn sub(self, other: Branch) -> Line {
        Line { g0: self.g0 - other.g0, g1: self.g1 }
    }
}

impl SubAssign<Branch> for Line {
    fn sub_assign(&mut self, other: Branch) {
        *self = (*self).sub(other);
    }
}

impl RegressiveProduct<Branch> for Line {
    type Output = Scalar;

    fn regressive_product(self, other: Branch) -> Scalar {
        Scalar { g0: self.g1[0] * other.g0[0] + self.g1[1] * other.g0[1] + self.g1[2] * other.g0[2] }
    }
}

impl SquaredMagnitude for Line {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Line {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { g0: self.squared_magnitude().g0.sqrt() }
    }
}

impl Signum for Line {
    type Output = Line;

    fn signum(self) -> Line {
        self.geometric_product(Scalar { g0: 1.0 / self.magnitude().g0 })
    }
}

impl Inverse for Line {
    type Output = Line;

    fn inverse(self) -> Line {
        self.reversal().geometric_product(Scalar { g0: 1.0 / self.squared_magnitude().g0 })
    }
}

impl Zero for Translator {
    fn zero() -> Self {
        Translator { g0: Simd32x4::from(0.0) }
    }
}

impl One for Translator {
    fn one() -> Self {
        Translator { g0: Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl Neg for Translator {
    type Output = Translator;

    fn neg(self) -> Translator {
        Translator { g0: self.g0 * Simd32x4::from(-1.0) }
    }
}

impl Automorphism for Translator {
    type Output = Translator;

    fn automorphism(self) -> Translator {
        Translator { g0: self.g0 }
    }
}

impl Reversal for Translator {
    type Output = Translator;

    fn reversal(self) -> Translator {
        Translator { g0: self.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) }
    }
}

impl Conjugation for Translator {
    type Output = Translator;

    fn conjugation(self) -> Translator {
        Translator { g0: self.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) }
    }
}

impl Into<Scalar> for Translator {
    fn into(self) -> Scalar {
        Scalar { g0: self.g0[0] }
    }
}

impl Add<Scalar> for Translator {
    type Output = Translator;

    fn add(self, other: Scalar) -> Translator {
        Translator { g0: self.g0 + Simd32x4::from(other.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl AddAssign<Scalar> for Translator {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Sub<Scalar> for Translator {
    type Output = Translator;

    fn sub(self, other: Scalar) -> Translator {
        Translator { g0: self.g0 - Simd32x4::from(other.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl SubAssign<Scalar> for Translator {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Scalar> for Translator {
    type Output = Translator;

    fn geometric_product(self, other: Scalar) -> Translator {
        Translator { g0: self.g0 * Simd32x4::from(other.g0) }
    }
}

impl OuterProduct<Scalar> for Translator {
    type Output = Translator;

    fn outer_product(self, other: Scalar) -> Translator {
        Translator { g0: self.g0 * Simd32x4::from(other.g0) }
    }
}

impl InnerProduct<Scalar> for Translator {
    type Output = Translator;

    fn inner_product(self, other: Scalar) -> Translator {
        Translator { g0: self.g0 * Simd32x4::from(other.g0) }
    }
}

impl LeftContraction<Scalar> for Translator {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0 }
    }
}

impl RightContraction<Scalar> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Scalar) -> Translator {
        Translator { g0: self.g0 * Simd32x4::from(other.g0) }
    }
}

impl ScalarProduct<Scalar> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0 }
    }
}

impl Add<MultiVector> for Translator {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.g0, g1: other.g1, g2: other.g2, g3: self.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.g3 }
    }
}

impl Sub<MultiVector> for Translator {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.g0, g1: Simd32x4::from(0.0) - other.g1, g2: Simd32x4::from(0.0) - other.g2, g3: self.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.g3 }
    }
}

impl GeometricProduct<MultiVector> for Translator {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0, g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[1]) * swizzle!(other.g2, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g2, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g2, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g2: Simd32x4::from(self.g0[0]) * other.g2, g3: Simd32x4::from(self.g0[0]) * other.g3 + Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) }
    }
}

impl OuterProduct<MultiVector> for Translator {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0, g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[2]) * swizzle!(other.g2, 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g2, 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + swizzle!(self.g0, 0, 0, 1, 1) * swizzle!(other.g2, 0, 0, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]), g2: Simd32x4::from(self.g0[0]) * other.g2, g3: Simd32x4::from(self.g0[0]) * other.g3 + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 1, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) }
    }
}

impl InnerProduct<MultiVector> for Translator {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0, g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g2[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g2[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g2, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.g0[0]) * other.g2, g3: Simd32x4::from(self.g0[0]) * other.g3 + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl LeftContraction<MultiVector> for Translator {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0, g1: Simd32x4::from(self.g0[0]) * other.g1, g2: Simd32x4::from(self.g0[0]) * other.g2, g3: Simd32x4::from(self.g0[0]) * other.g3 }
    }
}

impl ScalarProduct<MultiVector> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] }
    }
}

impl GeometricProduct<Rotor> for Translator {
    type Output = Motor;

    fn geometric_product(self, other: Rotor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0, g1: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) }
    }
}

impl RegressiveProduct<Rotor> for Translator {
    type Output = Scalar;

    fn regressive_product(self, other: Rotor) -> Scalar {
        Scalar { g0: self.g0[1] * other.g0[1] + self.g0[2] * other.g0[2] + self.g0[3] * other.g0[3] }
    }
}

impl OuterProduct<Rotor> for Translator {
    type Output = Motor;

    fn outer_product(self, other: Rotor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0, g1: Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 1, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) }
    }
}

impl LeftContraction<Rotor> for Translator {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { g0: Simd32x4::from(self.g0[0]) * other.g0 }
    }
}

impl RightContraction<Rotor> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Rotor) -> Translator {
        Translator { g0: self.g0 * Simd32x4::from(other.g0[0]) }
    }
}

impl ScalarProduct<Rotor> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] }
    }
}

impl GeometricProduct<Point> for Translator {
    type Output = Point;

    fn geometric_product(self, other: Point) -> Point {
        Point { g0: Simd32x4::from(self.g0[0]) * other.g0 + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) }
    }
}

impl RegressiveProduct<Point> for Translator {
    type Output = Plane;

    fn regressive_product(self, other: Point) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 1, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) }
    }
}

impl OuterProduct<Point> for Translator {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { g0: Simd32x4::from(self.g0[0]) * other.g0 }
    }
}

impl InnerProduct<Point> for Translator {
    type Output = Point;

    fn inner_product(self, other: Point) -> Point {
        Point { g0: Simd32x4::from(self.g0[0]) * other.g0 }
    }
}

impl LeftContraction<Point> for Translator {
    type Output = Point;

    fn left_contraction(self, other: Point) -> Point {
        Point { g0: Simd32x4::from(self.g0[0]) * other.g0 }
    }
}

impl InnerProduct<Plane> for Translator {
    type Output = Plane;

    fn inner_product(self, other: Plane) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl LeftContraction<Plane> for Translator {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[0]) * other.g0 }
    }
}

impl RegressiveProduct<Line> for Translator {
    type Output = Scalar;

    fn regressive_product(self, other: Line) -> Scalar {
        Scalar { g0: self.g0[1] * other.g1[0] + self.g0[2] * other.g1[1] + self.g0[3] * other.g1[2] }
    }
}

impl InnerProduct<Line> for Translator {
    type Output = Line;

    fn inner_product(self, other: Line) -> Line {
        Line { g0: Simd32x3::from(self.g0[0]) * other.g0, g1: Simd32x3::from(self.g0[0]) * other.g1 }
    }
}

impl LeftContraction<Line> for Translator {
    type Output = Line;

    fn left_contraction(self, other: Line) -> Line {
        Line { g0: Simd32x3::from(self.g0[0]) * other.g0, g1: Simd32x3::from(self.g0[0]) * other.g1 }
    }
}

impl Add<Translator> for Translator {
    type Output = Translator;

    fn add(self, other: Translator) -> Translator {
        Translator { g0: self.g0 + other.g0 }
    }
}

impl AddAssign<Translator> for Translator {
    fn add_assign(&mut self, other: Translator) {
        *self = (*self).add(other);
    }
}

impl Sub<Translator> for Translator {
    type Output = Translator;

    fn sub(self, other: Translator) -> Translator {
        Translator { g0: self.g0 - other.g0 }
    }
}

impl SubAssign<Translator> for Translator {
    fn sub_assign(&mut self, other: Translator) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Translator> for Translator {
    type Output = Translator;

    fn geometric_product(self, other: Translator) -> Translator {
        Translator { g0: Simd32x4::from(self.g0[0]) * other.g0 + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl OuterProduct<Translator> for Translator {
    type Output = Translator;

    fn outer_product(self, other: Translator) -> Translator {
        Translator { g0: Simd32x4::from(self.g0[0]) * other.g0 + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl InnerProduct<Translator> for Translator {
    type Output = Translator;

    fn inner_product(self, other: Translator) -> Translator {
        Translator { g0: Simd32x4::from(self.g0[0]) * other.g0 + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl LeftContraction<Translator> for Translator {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { g0: Simd32x4::from(self.g0[0]) * other.g0 }
    }
}

impl RightContraction<Translator> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Translator) -> Translator {
        Translator { g0: self.g0 * Simd32x4::from(other.g0[0]) }
    }
}

impl ScalarProduct<Translator> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: Translator) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] }
    }
}

impl Add<Motor> for Translator {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + other.g0, g1: self.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.g1 }
    }
}

impl Sub<Motor> for Translator {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - other.g0, g1: self.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.g1 }
    }
}

impl GeometricProduct<Motor> for Translator {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0, g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) }
    }
}

impl RegressiveProduct<Motor> for Translator {
    type Output = Translator;

    fn regressive_product(self, other: Motor) -> Translator {
        Translator { g0: Simd32x4::from(self.g0[1]) * Simd32x4::from([other.g0[1], other.g1[0], other.g0[1], other.g0[1]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g0[2]) * Simd32x4::from([other.g0[2], other.g0[2], other.g1[0], other.g0[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from([other.g0[3], other.g0[3], other.g0[3], other.g1[0]]) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from(other.g1[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl OuterProduct<Motor> for Translator {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0, g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 1, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) }
    }
}

impl InnerProduct<Motor> for Translator {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0, g1: Simd32x4::from(self.g0[0]) * other.g1 + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl LeftContraction<Motor> for Translator {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0, g1: Simd32x4::from(self.g0[0]) * other.g1 }
    }
}

impl RightContraction<Motor> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Motor) -> Translator {
        Translator { g0: self.g0 * Simd32x4::from(other.g0[0]) }
    }
}

impl ScalarProduct<Motor> for Translator {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] }
    }
}

impl Into<Branch> for Translator {
    fn into(self) -> Branch {
        Branch { g0: Simd32x3::from([self.g0[1], self.g0[2], self.g0[3]]) }
    }
}

impl Add<Branch> for Translator {
    type Output = Translator;

    fn add(self, other: Branch) -> Translator {
        Translator { g0: self.g0 + Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl AddAssign<Branch> for Translator {
    fn add_assign(&mut self, other: Branch) {
        *self = (*self).add(other);
    }
}

impl Sub<Branch> for Translator {
    type Output = Translator;

    fn sub(self, other: Branch) -> Translator {
        Translator { g0: self.g0 - Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl SubAssign<Branch> for Translator {
    fn sub_assign(&mut self, other: Branch) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Branch> for Translator {
    type Output = Branch;

    fn geometric_product(self, other: Branch) -> Branch {
        Branch { g0: Simd32x3::from(self.g0[0]) * other.g0 }
    }
}

impl OuterProduct<Branch> for Translator {
    type Output = Branch;

    fn outer_product(self, other: Branch) -> Branch {
        Branch { g0: Simd32x3::from(self.g0[0]) * other.g0 }
    }
}

impl InnerProduct<Branch> for Translator {
    type Output = Branch;

    fn inner_product(self, other: Branch) -> Branch {
        Branch { g0: Simd32x3::from(self.g0[0]) * other.g0 }
    }
}

impl LeftContraction<Branch> for Translator {
    type Output = Branch;

    fn left_contraction(self, other: Branch) -> Branch {
        Branch { g0: Simd32x3::from(self.g0[0]) * other.g0 }
    }
}

impl SquaredMagnitude for Translator {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Translator {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { g0: self.squared_magnitude().g0.sqrt() }
    }
}

impl Signum for Translator {
    type Output = Translator;

    fn signum(self) -> Translator {
        self.geometric_product(Scalar { g0: 1.0 / self.magnitude().g0 })
    }
}

impl Inverse for Translator {
    type Output = Translator;

    fn inverse(self) -> Translator {
        self.reversal().geometric_product(Scalar { g0: 1.0 / self.squared_magnitude().g0 })
    }
}

impl Zero for Motor {
    fn zero() -> Self {
        Motor { g0: Simd32x4::from(0.0), g1: Simd32x4::from(0.0) }
    }
}

impl One for Motor {
    fn one() -> Self {
        Motor { g0: Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(0.0) }
    }
}

impl Neg for Motor {
    type Output = Motor;

    fn neg(self) -> Motor {
        Motor { g0: self.g0 * Simd32x4::from(-1.0), g1: self.g1 * Simd32x4::from(-1.0) }
    }
}

impl Automorphism for Motor {
    type Output = Motor;

    fn automorphism(self) -> Motor {
        Motor { g0: self.g0, g1: self.g1 }
    }
}

impl Reversal for Motor {
    type Output = Motor;

    fn reversal(self) -> Motor {
        Motor { g0: self.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g1: self.g1 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) }
    }
}

impl Conjugation for Motor {
    type Output = Motor;

    fn conjugation(self) -> Motor {
        Motor { g0: self.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]), g1: self.g1 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) }
    }
}

impl Dual for Motor {
    type Output = Motor;

    fn dual(self) -> Motor {
        Motor { g0: self.g1, g1: self.g0 }
    }
}

impl Into<Scalar> for Motor {
    fn into(self) -> Scalar {
        Scalar { g0: self.g0[0] }
    }
}

impl Add<Scalar> for Motor {
    type Output = Motor;

    fn add(self, other: Scalar) -> Motor {
        Motor { g0: self.g0 + Simd32x4::from(other.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.g1 }
    }
}

impl AddAssign<Scalar> for Motor {
    fn add_assign(&mut self, other: Scalar) {
        *self = (*self).add(other);
    }
}

impl Sub<Scalar> for Motor {
    type Output = Motor;

    fn sub(self, other: Scalar) -> Motor {
        Motor { g0: self.g0 - Simd32x4::from(other.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.g1 }
    }
}

impl SubAssign<Scalar> for Motor {
    fn sub_assign(&mut self, other: Scalar) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Scalar> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Scalar) -> Motor {
        Motor { g0: self.g0 * Simd32x4::from(other.g0), g1: self.g1 * Simd32x4::from(other.g0) }
    }
}

impl RegressiveProduct<Scalar> for Motor {
    type Output = Scalar;

    fn regressive_product(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g1[0] * other.g0 }
    }
}

impl OuterProduct<Scalar> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Scalar) -> Motor {
        Motor { g0: self.g0 * Simd32x4::from(other.g0), g1: self.g1 * Simd32x4::from(other.g0) }
    }
}

impl InnerProduct<Scalar> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Scalar) -> Motor {
        Motor { g0: self.g0 * Simd32x4::from(other.g0), g1: self.g1 * Simd32x4::from(other.g0) }
    }
}

impl LeftContraction<Scalar> for Motor {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0 }
    }
}

impl RightContraction<Scalar> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Scalar) -> Motor {
        Motor { g0: self.g0 * Simd32x4::from(other.g0), g1: self.g1 * Simd32x4::from(other.g0) }
    }
}

impl ScalarProduct<Scalar> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: Scalar) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0 }
    }
}

impl Add<MultiVector> for Motor {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: self.g0 + other.g0, g1: other.g1, g2: other.g2, g3: self.g1 + other.g3 }
    }
}

impl Sub<MultiVector> for Motor {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: self.g0 - other.g0, g1: Simd32x4::from(0.0) - other.g1, g2: Simd32x4::from(0.0) - other.g2, g3: self.g1 - other.g3 }
    }
}

impl GeometricProduct<MultiVector> for Motor {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[1]) * swizzle!(other.g1, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g1, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g1, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) - Simd32x4::from(self.g1[0]) * other.g2 + Simd32x4::from(self.g1[1]) * swizzle!(other.g2, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g2, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g2, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g2: Simd32x4::from(self.g0[0]) * other.g2 + Simd32x4::from(self.g0[1]) * swizzle!(other.g2, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g2, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g2, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]), g3: Simd32x4::from(self.g0[0]) * other.g3 + Simd32x4::from(self.g0[1]) * swizzle!(other.g3, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g3, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g3, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.g1[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) }
    }
}

impl RegressiveProduct<MultiVector> for Motor {
    type Output = MultiVector;

    fn regressive_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g3, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g3, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g3, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g1[0]) * other.g0 + Simd32x4::from(self.g1[1]) * Simd32x4::from(other.g0[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from(other.g3[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.g1[0]) * other.g1 + Simd32x4::from(self.g1[2]) * Simd32x4::from(other.g1[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[3]) * Simd32x4::from(other.g1[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g1, 1, 0, 0, 0) * swizzle!(other.g1, 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.g0[2]) * swizzle!(other.g1, 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g1, 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.g1[0]) * other.g2 + Simd32x4::from(self.g1[2]) * Simd32x4::from(other.g2[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * Simd32x4::from(other.g2[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.g0[0], self.g1[1], self.g0[1], self.g0[1]]) * Simd32x4::from([other.g1[0], other.g2[0], other.g1[3], other.g1[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]), g3: Simd32x4::from(self.g1[0]) * other.g3 + self.g1 * Simd32x4::from(other.g3[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl OuterProduct<MultiVector> for Motor {
    type Output = MultiVector;

    fn outer_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g1[1]) * swizzle!(other.g2, 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g2, 3, 3, 3, 1) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g2, 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + self.g0 * Simd32x4::from(other.g1[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g2: Simd32x4::from(self.g0[0]) * other.g2 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g2[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g2[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g2, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g3: Simd32x4::from(self.g0[0]) * other.g3 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g3[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g3[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g3, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl InnerProduct<MultiVector> for Motor {
    type Output = MultiVector;

    fn inner_product(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 1, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g1[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g1[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) - Simd32x4::from(self.g1[0]) * other.g2 + Simd32x4::from(self.g1[1]) * Simd32x4::from(other.g2[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * Simd32x4::from(other.g2[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[3]) * Simd32x4::from(other.g2[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g1, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.g0[0]) * other.g2 + Simd32x4::from(self.g0[2]) * swizzle!(other.g2, 3, 3, 0, 1) * Simd32x4::from([0.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g2, 2, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) + swizzle!(self.g0, 0, 1, 1, 1) * swizzle!(other.g2, 0, 0, 3, 2) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]), g3: Simd32x4::from(self.g0[0]) * other.g3 + Simd32x4::from(self.g1[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g1[1]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + self.g0 * Simd32x4::from(other.g3[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) }
    }
}

impl LeftContraction<MultiVector> for Motor {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g1[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g1[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g1, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g2: Simd32x4::from(self.g0[0]) * other.g2 + self.g0 * Simd32x4::from(other.g2[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]), g3: Simd32x4::from(self.g0[0]) * other.g3 + self.g0 * Simd32x4::from(other.g3[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) }
    }
}

impl ScalarProduct<MultiVector> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: MultiVector) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] - self.g0[1] * other.g0[1] - self.g0[2] * other.g0[2] - self.g0[3] * other.g0[3] }
    }
}

impl Into<Rotor> for Motor {
    fn into(self) -> Rotor {
        Rotor { g0: self.g0 }
    }
}

impl Add<Rotor> for Motor {
    type Output = Motor;

    fn add(self, other: Rotor) -> Motor {
        Motor { g0: self.g0 + other.g0, g1: self.g1 }
    }
}

impl AddAssign<Rotor> for Motor {
    fn add_assign(&mut self, other: Rotor) {
        *self = (*self).add(other);
    }
}

impl Sub<Rotor> for Motor {
    type Output = Motor;

    fn sub(self, other: Rotor) -> Motor {
        Motor { g0: self.g0 - other.g0, g1: self.g1 }
    }
}

impl SubAssign<Rotor> for Motor {
    fn sub_assign(&mut self, other: Rotor) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Rotor> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Rotor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.g1[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) }
    }
}

impl RegressiveProduct<Rotor> for Motor {
    type Output = Rotor;

    fn regressive_product(self, other: Rotor) -> Rotor {
        Rotor { g0: Simd32x4::from(self.g1[0]) * other.g0 + Simd32x4::from(self.g1[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g1, 1, 0, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl OuterProduct<Rotor> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Rotor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0 + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl InnerProduct<Rotor> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Rotor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 1, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.g1[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.g1 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl LeftContraction<Rotor> for Motor {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) }
    }
}

impl RightContraction<Rotor> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Rotor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.g1[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.g1 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl ScalarProduct<Rotor> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: Rotor) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] - self.g0[1] * other.g0[1] - self.g0[2] * other.g0[2] - self.g0[3] * other.g0[3] }
    }
}

impl OuterProduct<Point> for Motor {
    type Output = Point;

    fn outer_product(self, other: Point) -> Point {
        Point { g0: Simd32x4::from(self.g0[0]) * other.g0 }
    }
}

impl RegressiveProduct<Plane> for Motor {
    type Output = Plane;

    fn regressive_product(self, other: Plane) -> Plane {
        Plane { g0: Simd32x4::from(self.g1[0]) * other.g0 }
    }
}

impl LeftContraction<Plane> for Motor {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[0]) * other.g0 }
    }
}

impl Into<Line> for Motor {
    fn into(self) -> Line {
        Line { g0: Simd32x3::from([self.g1[1], self.g1[2], self.g1[3]]), g1: Simd32x3::from([self.g0[1], self.g0[2], self.g0[3]]) }
    }
}

impl Add<Line> for Motor {
    type Output = Motor;

    fn add(self, other: Line) -> Motor {
        Motor { g0: self.g0 + Simd32x4::from([other.g0[0], other.g1[0], other.g1[1], other.g1[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: self.g1 + Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl AddAssign<Line> for Motor {
    fn add_assign(&mut self, other: Line) {
        *self = (*self).add(other);
    }
}

impl Sub<Line> for Motor {
    type Output = Motor;

    fn sub(self, other: Line) -> Motor {
        Motor { g0: self.g0 - Simd32x4::from([other.g0[0], other.g1[0], other.g1[1], other.g1[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: self.g1 - Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl SubAssign<Line> for Motor {
    fn sub_assign(&mut self, other: Line) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Line> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Line) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[1]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[2], other.g1[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * Simd32x4::from([other.g1[1], other.g1[2], other.g1[1], other.g1[0]]) * Simd32x4::from([-1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from([other.g1[2], other.g1[1], other.g1[0], other.g1[2]]) * Simd32x4::from([-1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[1], other.g1[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.g0[1]) * Simd32x4::from([other.g0[0], other.g0[0], other.g0[2], other.g0[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * Simd32x4::from([other.g0[1], other.g0[2], other.g0[1], other.g0[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from([other.g0[2], other.g0[1], other.g0[0], other.g0[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[1], other.g1[2]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g1[1]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[2], other.g1[1]]) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g1[2]) * Simd32x4::from([other.g1[1], other.g1[2], other.g1[1], other.g1[0]]) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g1[3]) * Simd32x4::from([other.g1[2], other.g1[1], other.g1[0], other.g1[2]]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl RightContraction<Line> for Motor {
    type Output = Translator;

    fn right_contraction(self, other: Line) -> Translator {
        Translator { g0: Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g1[1]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g1[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from([self.g0[1], self.g1[0], self.g1[0], self.g1[0]]) * Simd32x4::from([other.g1[0], other.g1[0], other.g1[1], other.g1[2]]) * Simd32x4::from(-1.0) }
    }
}

impl ScalarProduct<Line> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: Line) -> Scalar {
        Scalar { g0: 0.0 - self.g0[1] * other.g1[0] - self.g0[2] * other.g1[1] - self.g0[3] * other.g1[2] }
    }
}

impl Into<Translator> for Motor {
    fn into(self) -> Translator {
        Translator { g0: Simd32x4::from([self.g0[0], self.g1[1], self.g1[2], self.g1[3]]) }
    }
}

impl Add<Translator> for Motor {
    type Output = Motor;

    fn add(self, other: Translator) -> Motor {
        Motor { g0: self.g0 + Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.g1 + other.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl AddAssign<Translator> for Motor {
    fn add_assign(&mut self, other: Translator) {
        *self = (*self).add(other);
    }
}

impl Sub<Translator> for Motor {
    type Output = Motor;

    fn sub(self, other: Translator) -> Motor {
        Motor { g0: self.g0 - Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: self.g1 - other.g0 * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl SubAssign<Translator> for Motor {
    fn sub_assign(&mut self, other: Translator) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Translator> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Translator) -> Motor {
        Motor { g0: self.g0 * Simd32x4::from(other.g0[0]), g1: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 1, 3, 2) * Simd32x4::from([1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 2, 1) * Simd32x4::from([1.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 3) * Simd32x4::from([1.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.g1[1]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.g1[0], self.g0[0], self.g0[0], self.g0[0]]) * other.g0 }
    }
}

impl RegressiveProduct<Translator> for Motor {
    type Output = Translator;

    fn regressive_product(self, other: Translator) -> Translator {
        Translator { g0: Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[0]) * other.g0 + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl OuterProduct<Translator> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Translator) -> Motor {
        Motor { g0: self.g0 * Simd32x4::from(other.g0[0]), g1: Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[1]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g0, 1, 1, 2, 3) }
    }
}

impl InnerProduct<Translator> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Translator) -> Motor {
        Motor { g0: self.g0 * Simd32x4::from(other.g0[0]), g1: Simd32x4::from(self.g1[1]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.g1[0], self.g0[0], self.g0[0], self.g0[0]]) * other.g0 }
    }
}

impl LeftContraction<Translator> for Motor {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { g0: Simd32x4::from(self.g0[0]) * other.g0 }
    }
}

impl RightContraction<Translator> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Translator) -> Motor {
        Motor { g0: self.g0 * Simd32x4::from(other.g0[0]), g1: self.g1 * Simd32x4::from(other.g0[0]) }
    }
}

impl ScalarProduct<Translator> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: Translator) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] }
    }
}

impl Add<Motor> for Motor {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { g0: self.g0 + other.g0, g1: self.g1 + other.g1 }
    }
}

impl AddAssign<Motor> for Motor {
    fn add_assign(&mut self, other: Motor) {
        *self = (*self).add(other);
    }
}

impl Sub<Motor> for Motor {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { g0: self.g0 - other.g0, g1: self.g1 - other.g1 }
    }
}

impl SubAssign<Motor> for Motor {
    fn sub_assign(&mut self, other: Motor) {
        *self = (*self).sub(other);
    }
}

impl GeometricProduct<Motor> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([-1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[1]) * swizzle!(other.g1, 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g1, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g1, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.g1[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 1, 0, 3, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 3, 0, 1) * Simd32x4::from([1.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, 1.0]) }
    }
}

impl RegressiveProduct<Motor> for Motor {
    type Output = Motor;

    fn regressive_product(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g1, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g1, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g1, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g1[0]) * other.g0 + Simd32x4::from(self.g1[1]) * Simd32x4::from(other.g0[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from(other.g1[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.g1[0]) * other.g1 + self.g1 * Simd32x4::from(other.g1[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl OuterProduct<Motor> for Motor {
    type Output = Motor;

    fn outer_product(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0 + self.g0 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g1[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g1[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g1[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g1, 1, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl InnerProduct<Motor> for Motor {
    type Output = Motor;

    fn inner_product(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + swizzle!(self.g0, 1, 1, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + Simd32x4::from(self.g1[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + Simd32x4::from(self.g1[1]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g1[2]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g1[3]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + self.g0 * Simd32x4::from(other.g1[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) }
    }
}

impl LeftContraction<Motor> for Motor {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[0]) * other.g0 + Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g0[3]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]) + swizzle!(self.g0, 1, 0, 0, 0) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.g0[0]) * other.g1 + self.g0 * Simd32x4::from(other.g1[0]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]) }
    }
}

impl RightContraction<Motor> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[3]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from(other.g0[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]), g1: Simd32x4::from(self.g1[0]) * other.g0 * Simd32x4::from([1.0, -1.0, -1.0, -1.0]) + self.g1 * Simd32x4::from(other.g0[0]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl ScalarProduct<Motor> for Motor {
    type Output = Scalar;

    fn scalar_product(self, other: Motor) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[0] - self.g0[1] * other.g0[1] - self.g0[2] * other.g0[2] - self.g0[3] * other.g0[3] }
    }
}

impl Into<Branch> for Motor {
    fn into(self) -> Branch {
        Branch { g0: Simd32x3::from([self.g1[1], self.g1[2], self.g1[3]]) }
    }
}

impl Add<Branch> for Motor {
    type Output = Motor;

    fn add(self, other: Branch) -> Motor {
        Motor { g0: self.g0, g1: self.g1 + Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl AddAssign<Branch> for Motor {
    fn add_assign(&mut self, other: Branch) {
        *self = (*self).add(other);
    }
}

impl Sub<Branch> for Motor {
    type Output = Motor;

    fn sub(self, other: Branch) -> Motor {
        Motor { g0: self.g0, g1: self.g1 - Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) }
    }
}

impl SubAssign<Branch> for Motor {
    fn sub_assign(&mut self, other: Branch) {
        *self = (*self).sub(other);
    }
}

impl RegressiveProduct<Branch> for Motor {
    type Output = Translator;

    fn regressive_product(self, other: Branch) -> Translator {
        Translator { g0: Simd32x4::from(self.g0[2]) * Simd32x4::from(other.g0[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.g0[3]) * Simd32x4::from(other.g0[2]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from([self.g0[1], self.g1[0], self.g1[0], self.g1[0]]) * Simd32x4::from([other.g0[0], other.g0[0], other.g0[1], other.g0[2]]) }
    }
}

impl InnerProduct<Branch> for Motor {
    type Output = Branch;

    fn inner_product(self, other: Branch) -> Branch {
        Branch { g0: Simd32x3::from(self.g0[0]) * other.g0 }
    }
}

impl LeftContraction<Branch> for Motor {
    type Output = Branch;

    fn left_contraction(self, other: Branch) -> Branch {
        Branch { g0: Simd32x3::from(self.g0[0]) * other.g0 }
    }
}

impl SquaredMagnitude for Motor {
    type Output = Scalar;

    fn squared_magnitude(self) -> Scalar {
        self.scalar_product(self.reversal())
    }
}

impl Magnitude for Motor {
    type Output = Scalar;

    fn magnitude(self) -> Scalar {
        Scalar { g0: self.squared_magnitude().g0.sqrt() }
    }
}

impl Signum for Motor {
    type Output = Motor;

    fn signum(self) -> Motor {
        self.geometric_product(Scalar { g0: 1.0 / self.magnitude().g0 })
    }
}

impl Inverse for Motor {
    type Output = Motor;

    fn inverse(self) -> Motor {
        self.reversal().geometric_product(Scalar { g0: 1.0 / self.squared_magnitude().g0 })
    }
}

impl Zero for Branch {
    fn zero() -> Self {
        Branch { g0: Simd32x3::from(0.0) }
    }
}

impl One for Branch {
    fn one() -> Self {
        Branch { g0: Simd32x3::from(0.0) }
    }
}

impl Neg for Branch {
    type Output = Branch;

    fn neg(self) -> Branch {
        Branch { g0: self.g0 * Simd32x3::from(-1.0) }
    }
}

impl Automorphism for Branch {
    type Output = Branch;

    fn automorphism(self) -> Branch {
        Branch { g0: self.g0 }
    }
}

impl Reversal for Branch {
    type Output = Branch;

    fn reversal(self) -> Branch {
        Branch { g0: self.g0 * Simd32x3::from(-1.0) }
    }
}

impl Conjugation for Branch {
    type Output = Branch;

    fn conjugation(self) -> Branch {
        Branch { g0: self.g0 * Simd32x3::from(-1.0) }
    }
}

impl Add<Scalar> for Branch {
    type Output = Translator;

    fn add(self, other: Scalar) -> Translator {
        Translator { g0: Simd32x4::from([self.g0[0], self.g0[0], self.g0[1], self.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + Simd32x4::from(other.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl Sub<Scalar> for Branch {
    type Output = Translator;

    fn sub(self, other: Scalar) -> Translator {
        Translator { g0: Simd32x4::from([self.g0[0], self.g0[0], self.g0[1], self.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - Simd32x4::from(other.g0) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) }
    }
}

impl GeometricProduct<Scalar> for Branch {
    type Output = Branch;

    fn geometric_product(self, other: Scalar) -> Branch {
        Branch { g0: self.g0 * Simd32x3::from(other.g0) }
    }
}

impl OuterProduct<Scalar> for Branch {
    type Output = Branch;

    fn outer_product(self, other: Scalar) -> Branch {
        Branch { g0: self.g0 * Simd32x3::from(other.g0) }
    }
}

impl InnerProduct<Scalar> for Branch {
    type Output = Branch;

    fn inner_product(self, other: Scalar) -> Branch {
        Branch { g0: self.g0 * Simd32x3::from(other.g0) }
    }
}

impl RightContraction<Scalar> for Branch {
    type Output = Branch;

    fn right_contraction(self, other: Scalar) -> Branch {
        Branch { g0: self.g0 * Simd32x3::from(other.g0) }
    }
}

impl Add<MultiVector> for Branch {
    type Output = MultiVector;

    fn add(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: other.g0, g1: other.g1, g2: other.g2, g3: Simd32x4::from([self.g0[0], self.g0[0], self.g0[1], self.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.g3 }
    }
}

impl Sub<MultiVector> for Branch {
    type Output = MultiVector;

    fn sub(self, other: MultiVector) -> MultiVector {
        MultiVector { g0: Simd32x4::from(0.0) - other.g0, g1: Simd32x4::from(0.0) - other.g1, g2: Simd32x4::from(0.0) - other.g2, g3: Simd32x4::from([self.g0[0], self.g0[0], self.g0[1], self.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.g3 }
    }
}

impl RegressiveProduct<Rotor> for Branch {
    type Output = Scalar;

    fn regressive_product(self, other: Rotor) -> Scalar {
        Scalar { g0: self.g0[0] * other.g0[1] + self.g0[1] * other.g0[2] + self.g0[2] * other.g0[3] }
    }
}

impl InnerProduct<Rotor> for Branch {
    type Output = Branch;

    fn inner_product(self, other: Rotor) -> Branch {
        Branch { g0: self.g0 * Simd32x3::from(other.g0[0]) }
    }
}

impl RightContraction<Rotor> for Branch {
    type Output = Branch;

    fn right_contraction(self, other: Rotor) -> Branch {
        Branch { g0: self.g0 * Simd32x3::from(other.g0[0]) }
    }
}

impl RegressiveProduct<Point> for Branch {
    type Output = Plane;

    fn regressive_product(self, other: Point) -> Plane {
        Plane { g0: Simd32x4::from(self.g0[1]) * swizzle!(other.g0, 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[2]) * swizzle!(other.g0, 3, 3, 3, 0) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g0[0]) * swizzle!(other.g0, 1, 0, 0, 0) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) }
    }
}

impl Add<Line> for Branch {
    type Output = Line;

    fn add(self, other: Line) -> Line {
        Line { g0: self.g0 + other.g0, g1: other.g1 }
    }
}

impl Sub<Line> for Branch {
    type Output = Line;

    fn sub(self, other: Line) -> Line {
        Line { g0: self.g0 - other.g0, g1: Simd32x3::from(0.0) - other.g1 }
    }
}

impl RegressiveProduct<Line> for Branch {
    type Output = Scalar;

    fn regressive_product(self, other: Line) -> Scalar {
        Scalar { g0: self.g0[0] * other.g1[0] + self.g0[1] * other.g1[1] + self.g0[2] * other.g1[2] }
    }
}

impl Add<Translator> for Branch {
    type Output = Translator;

    fn add(self, other: Translator) -> Translator {
        Translator { g0: Simd32x4::from([self.g0[0], self.g0[0], self.g0[1], self.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.g0 }
    }
}

impl Sub<Translator> for Branch {
    type Output = Translator;

    fn sub(self, other: Translator) -> Translator {
        Translator { g0: Simd32x4::from([self.g0[0], self.g0[0], self.g0[1], self.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.g0 }
    }
}

impl GeometricProduct<Translator> for Branch {
    type Output = Branch;

    fn geometric_product(self, other: Translator) -> Branch {
        Branch { g0: self.g0 * Simd32x3::from(other.g0[0]) }
    }
}

impl OuterProduct<Translator> for Branch {
    type Output = Branch;

    fn outer_product(self, other: Translator) -> Branch {
        Branch { g0: self.g0 * Simd32x3::from(other.g0[0]) }
    }
}

impl InnerProduct<Translator> for Branch {
    type Output = Branch;

    fn inner_product(self, other: Translator) -> Branch {
        Branch { g0: self.g0 * Simd32x3::from(other.g0[0]) }
    }
}

impl RightContraction<Translator> for Branch {
    type Output = Branch;

    fn right_contraction(self, other: Translator) -> Branch {
        Branch { g0: self.g0 * Simd32x3::from(other.g0[0]) }
    }
}

impl Add<Motor> for Branch {
    type Output = Motor;

    fn add(self, other: Motor) -> Motor {
        Motor { g0: other.g0, g1: Simd32x4::from([self.g0[0], self.g0[0], self.g0[1], self.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) + other.g1 }
    }
}

impl Sub<Motor> for Branch {
    type Output = Motor;

    fn sub(self, other: Motor) -> Motor {
        Motor { g0: Simd32x4::from(0.0) - other.g0, g1: Simd32x4::from([self.g0[0], self.g0[0], self.g0[1], self.g0[2]]) * Simd32x4::from([0.0, 1.0, 1.0, 1.0]) - other.g1 }
    }
}

impl RegressiveProduct<Motor> for Branch {
    type Output = Translator;

    fn regressive_product(self, other: Motor) -> Translator {
        Translator { g0: Simd32x4::from(self.g0[1]) * Simd32x4::from([other.g0[2], other.g0[2], other.g1[0], other.g0[2]]) * Simd32x4::from([1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.g0[2]) * Simd32x4::from([other.g0[3], other.g0[3], other.g0[3], other.g1[0]]) * Simd32x4::from([1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.g0[0]) * Simd32x4::from([other.g0[1], other.g1[0], other.g0[0], other.g0[0]]) * Simd32x4::from([1.0, 1.0, 0.0, 0.0]) }
    }
}

impl InnerProduct<Motor> for Branch {
    type Output = Branch;

    fn inner_product(self, other: Motor) -> Branch {
        Branch { g0: self.g0 * Simd32x3::from(other.g0[0]) }
    }
}

impl RightContraction<Motor> for Branch {
    type Output = Branch;

    fn right_contraction(self, other: Motor) -> Branch {
        Branch { g0: self.g0 * Simd32x3::from(other.g0[0]) }
    }
}

impl Add<Branch> for Branch {
    type Output = Branch;

    fn add(self, other: Branch) -> Branch {
        Branch { g0: self.g0 + other.g0 }
    }
}

impl AddAssign<Branch> for Branch {
    fn add_assign(&mut self, other: Branch) {
        *self = (*self).add(other);
    }
}

impl Sub<Branch> for Branch {
    type Output = Branch;

    fn sub(self, other: Branch) -> Branch {
        Branch { g0: self.g0 - other.g0 }
    }
}

impl SubAssign<Branch> for Branch {
    fn sub_assign(&mut self, other: Branch) {
        *self = (*self).sub(other);
    }
}

impl Mul<Scalar> for Branch {
    type Output = Branch;

    fn mul(self, other: Scalar) -> Branch {
        self.geometric_product(other)
    }
}

impl MulAssign<Scalar> for Branch {
    fn mul_assign(&mut self, other: Scalar) {
        *self = (*self).mul(other);
    }
}

impl Div<Scalar> for Branch {
    type Output = Branch;

    fn div(self, other: Scalar) -> Branch {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Scalar> for Branch {
    fn div_assign(&mut self, other: Scalar) {
        *self = (*self).div(other);
    }
}

impl Mul<Translator> for Branch {
    type Output = Branch;

    fn mul(self, other: Translator) -> Branch {
        self.geometric_product(other)
    }
}

impl MulAssign<Translator> for Branch {
    fn mul_assign(&mut self, other: Translator) {
        *self = (*self).mul(other);
    }
}

impl Div<Translator> for Branch {
    type Output = Branch;

    fn div(self, other: Translator) -> Branch {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Translator> for Branch {
    fn div_assign(&mut self, other: Translator) {
        *self = (*self).div(other);
    }
}

impl Mul<Line> for Line {
    type Output = Motor;

    fn mul(self, other: Line) -> Motor {
        self.geometric_product(other)
    }
}

impl Div<Line> for Line {
    type Output = Motor;

    fn div(self, other: Line) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Line> for Line {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Motor> for Line {
    type Output = Motor;

    fn mul(self, other: Motor) -> Motor {
        self.geometric_product(other)
    }
}

impl Div<Motor> for Line {
    type Output = Motor;

    fn div(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Line {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<MultiVector> for Line {
    type Output = MultiVector;

    fn mul(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other)
    }
}

impl Div<MultiVector> for Line {
    type Output = MultiVector;

    fn div(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Line {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Rotor> for Line {
    type Output = Motor;

    fn mul(self, other: Rotor) -> Motor {
        self.geometric_product(other)
    }
}

impl Div<Rotor> for Line {
    type Output = Motor;

    fn div(self, other: Rotor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for Line {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Scalar> for Line {
    type Output = Line;

    fn mul(self, other: Scalar) -> Line {
        self.geometric_product(other)
    }
}

impl MulAssign<Scalar> for Line {
    fn mul_assign(&mut self, other: Scalar) {
        *self = (*self).mul(other);
    }
}

impl Div<Scalar> for Line {
    type Output = Line;

    fn div(self, other: Scalar) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Scalar> for Line {
    fn div_assign(&mut self, other: Scalar) {
        *self = (*self).div(other);
    }
}

impl Transformation<Scalar> for Line {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Line> for Motor {
    type Output = Motor;

    fn mul(self, other: Line) -> Motor {
        self.geometric_product(other)
    }
}

impl MulAssign<Line> for Motor {
    fn mul_assign(&mut self, other: Line) {
        *self = (*self).mul(other);
    }
}

impl Div<Line> for Motor {
    type Output = Motor;

    fn div(self, other: Line) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Line> for Motor {
    fn div_assign(&mut self, other: Line) {
        *self = (*self).div(other);
    }
}

impl Transformation<Line> for Motor {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Motor> for Motor {
    type Output = Motor;

    fn mul(self, other: Motor) -> Motor {
        self.geometric_product(other)
    }
}

impl MulAssign<Motor> for Motor {
    fn mul_assign(&mut self, other: Motor) {
        *self = (*self).mul(other);
    }
}

impl Powi for Motor {
    type Output = Motor;

    fn powi(self, exponent: isize) -> Motor {
        if exponent == 0 {
            return Motor::one();
        }
        let mut x: Motor = if exponent < 0 { self.inverse() } else { self };
        let mut y: Motor = Motor::one();
        let mut n: isize = exponent.abs();
        while 1 < n {
            if n & 1 == 1 {
                y = x.geometric_product(y);
            }
            x = x.geometric_product(x);
            n = n >> 1;
        }
        x.geometric_product(y)
    }
}

impl Div<Motor> for Motor {
    type Output = Motor;

    fn div(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Motor> for Motor {
    fn div_assign(&mut self, other: Motor) {
        *self = (*self).div(other);
    }
}

impl Transformation<Motor> for Motor {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<MultiVector> for Motor {
    type Output = MultiVector;

    fn mul(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other)
    }
}

impl Div<MultiVector> for Motor {
    type Output = MultiVector;

    fn div(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Motor {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Rotor> for Motor {
    type Output = Motor;

    fn mul(self, other: Rotor) -> Motor {
        self.geometric_product(other)
    }
}

impl MulAssign<Rotor> for Motor {
    fn mul_assign(&mut self, other: Rotor) {
        *self = (*self).mul(other);
    }
}

impl Div<Rotor> for Motor {
    type Output = Motor;

    fn div(self, other: Rotor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Rotor> for Motor {
    fn div_assign(&mut self, other: Rotor) {
        *self = (*self).div(other);
    }
}

impl Transformation<Rotor> for Motor {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Scalar> for Motor {
    type Output = Motor;

    fn mul(self, other: Scalar) -> Motor {
        self.geometric_product(other)
    }
}

impl MulAssign<Scalar> for Motor {
    fn mul_assign(&mut self, other: Scalar) {
        *self = (*self).mul(other);
    }
}

impl Div<Scalar> for Motor {
    type Output = Motor;

    fn div(self, other: Scalar) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Scalar> for Motor {
    fn div_assign(&mut self, other: Scalar) {
        *self = (*self).div(other);
    }
}

impl Transformation<Scalar> for Motor {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Translator> for Motor {
    type Output = Motor;

    fn mul(self, other: Translator) -> Motor {
        self.geometric_product(other)
    }
}

impl MulAssign<Translator> for Motor {
    fn mul_assign(&mut self, other: Translator) {
        *self = (*self).mul(other);
    }
}

impl Div<Translator> for Motor {
    type Output = Motor;

    fn div(self, other: Translator) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Translator> for Motor {
    fn div_assign(&mut self, other: Translator) {
        *self = (*self).div(other);
    }
}

impl Transformation<Translator> for Motor {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Line> for MultiVector {
    type Output = MultiVector;

    fn mul(self, other: Line) -> MultiVector {
        self.geometric_product(other)
    }
}

impl MulAssign<Line> for MultiVector {
    fn mul_assign(&mut self, other: Line) {
        *self = (*self).mul(other);
    }
}

impl Div<Line> for MultiVector {
    type Output = MultiVector;

    fn div(self, other: Line) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Line> for MultiVector {
    fn div_assign(&mut self, other: Line) {
        *self = (*self).div(other);
    }
}

impl Transformation<Line> for MultiVector {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Motor> for MultiVector {
    type Output = MultiVector;

    fn mul(self, other: Motor) -> MultiVector {
        self.geometric_product(other)
    }
}

impl MulAssign<Motor> for MultiVector {
    fn mul_assign(&mut self, other: Motor) {
        *self = (*self).mul(other);
    }
}

impl Div<Motor> for MultiVector {
    type Output = MultiVector;

    fn div(self, other: Motor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Motor> for MultiVector {
    fn div_assign(&mut self, other: Motor) {
        *self = (*self).div(other);
    }
}

impl Transformation<Motor> for MultiVector {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn mul(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other)
    }
}

impl MulAssign<MultiVector> for MultiVector {
    fn mul_assign(&mut self, other: MultiVector) {
        *self = (*self).mul(other);
    }
}

impl Powi for MultiVector {
    type Output = MultiVector;

    fn powi(self, exponent: isize) -> MultiVector {
        if exponent == 0 {
            return MultiVector::one();
        }
        let mut x: MultiVector = if exponent < 0 { self.inverse() } else { self };
        let mut y: MultiVector = MultiVector::one();
        let mut n: isize = exponent.abs();
        while 1 < n {
            if n & 1 == 1 {
                y = x.geometric_product(y);
            }
            x = x.geometric_product(x);
            n = n >> 1;
        }
        x.geometric_product(y)
    }
}

impl Div<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn div(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<MultiVector> for MultiVector {
    fn div_assign(&mut self, other: MultiVector) {
        *self = (*self).div(other);
    }
}

impl Transformation<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Plane> for MultiVector {
    type Output = MultiVector;

    fn mul(self, other: Plane) -> MultiVector {
        self.geometric_product(other)
    }
}

impl MulAssign<Plane> for MultiVector {
    fn mul_assign(&mut self, other: Plane) {
        *self = (*self).mul(other);
    }
}

impl Div<Plane> for MultiVector {
    type Output = MultiVector;

    fn div(self, other: Plane) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Plane> for MultiVector {
    fn div_assign(&mut self, other: Plane) {
        *self = (*self).div(other);
    }
}

impl Transformation<Plane> for MultiVector {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Point> for MultiVector {
    type Output = MultiVector;

    fn mul(self, other: Point) -> MultiVector {
        self.geometric_product(other)
    }
}

impl MulAssign<Point> for MultiVector {
    fn mul_assign(&mut self, other: Point) {
        *self = (*self).mul(other);
    }
}

impl Div<Point> for MultiVector {
    type Output = MultiVector;

    fn div(self, other: Point) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Point> for MultiVector {
    fn div_assign(&mut self, other: Point) {
        *self = (*self).div(other);
    }
}

impl Transformation<Point> for MultiVector {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Rotor> for MultiVector {
    type Output = MultiVector;

    fn mul(self, other: Rotor) -> MultiVector {
        self.geometric_product(other)
    }
}

impl MulAssign<Rotor> for MultiVector {
    fn mul_assign(&mut self, other: Rotor) {
        *self = (*self).mul(other);
    }
}

impl Div<Rotor> for MultiVector {
    type Output = MultiVector;

    fn div(self, other: Rotor) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Rotor> for MultiVector {
    fn div_assign(&mut self, other: Rotor) {
        *self = (*self).div(other);
    }
}

impl Transformation<Rotor> for MultiVector {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Scalar> for MultiVector {
    type Output = MultiVector;

    fn mul(self, other: Scalar) -> MultiVector {
        self.geometric_product(other)
    }
}

impl MulAssign<Scalar> for MultiVector {
    fn mul_assign(&mut self, other: Scalar) {
        *self = (*self).mul(other);
    }
}

impl Div<Scalar> for MultiVector {
    type Output = MultiVector;

    fn div(self, other: Scalar) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Scalar> for MultiVector {
    fn div_assign(&mut self, other: Scalar) {
        *self = (*self).div(other);
    }
}

impl Transformation<Scalar> for MultiVector {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Translator> for MultiVector {
    type Output = MultiVector;

    fn mul(self, other: Translator) -> MultiVector {
        self.geometric_product(other)
    }
}

impl MulAssign<Translator> for MultiVector {
    fn mul_assign(&mut self, other: Translator) {
        *self = (*self).mul(other);
    }
}

impl Div<Translator> for MultiVector {
    type Output = MultiVector;

    fn div(self, other: Translator) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Translator> for MultiVector {
    fn div_assign(&mut self, other: Translator) {
        *self = (*self).div(other);
    }
}

impl Transformation<Translator> for MultiVector {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<MultiVector> for Plane {
    type Output = MultiVector;

    fn mul(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other)
    }
}

impl Div<MultiVector> for Plane {
    type Output = MultiVector;

    fn div(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Plane {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Scalar> for Plane {
    type Output = Plane;

    fn mul(self, other: Scalar) -> Plane {
        self.geometric_product(other)
    }
}

impl MulAssign<Scalar> for Plane {
    fn mul_assign(&mut self, other: Scalar) {
        *self = (*self).mul(other);
    }
}

impl Div<Scalar> for Plane {
    type Output = Plane;

    fn div(self, other: Scalar) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Scalar> for Plane {
    fn div_assign(&mut self, other: Scalar) {
        *self = (*self).div(other);
    }
}

impl Mul<MultiVector> for Point {
    type Output = MultiVector;

    fn mul(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other)
    }
}

impl Div<MultiVector> for Point {
    type Output = MultiVector;

    fn div(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Point {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Point> for Point {
    type Output = Translator;

    fn mul(self, other: Point) -> Translator {
        self.geometric_product(other)
    }
}

impl Div<Point> for Point {
    type Output = Translator;

    fn div(self, other: Point) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Point {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Scalar> for Point {
    type Output = Point;

    fn mul(self, other: Scalar) -> Point {
        self.geometric_product(other)
    }
}

impl MulAssign<Scalar> for Point {
    fn mul_assign(&mut self, other: Scalar) {
        *self = (*self).mul(other);
    }
}

impl Div<Scalar> for Point {
    type Output = Point;

    fn div(self, other: Scalar) -> Point {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Scalar> for Point {
    fn div_assign(&mut self, other: Scalar) {
        *self = (*self).div(other);
    }
}

impl Transformation<Scalar> for Point {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Translator> for Point {
    type Output = Point;

    fn mul(self, other: Translator) -> Point {
        self.geometric_product(other)
    }
}

impl MulAssign<Translator> for Point {
    fn mul_assign(&mut self, other: Translator) {
        *self = (*self).mul(other);
    }
}

impl Div<Translator> for Point {
    type Output = Point;

    fn div(self, other: Translator) -> Point {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Translator> for Point {
    fn div_assign(&mut self, other: Translator) {
        *self = (*self).div(other);
    }
}

impl Transformation<Translator> for Point {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Line> for Rotor {
    type Output = Motor;

    fn mul(self, other: Line) -> Motor {
        self.geometric_product(other)
    }
}

impl Div<Line> for Rotor {
    type Output = Motor;

    fn div(self, other: Line) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Line> for Rotor {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Motor> for Rotor {
    type Output = Motor;

    fn mul(self, other: Motor) -> Motor {
        self.geometric_product(other)
    }
}

impl Div<Motor> for Rotor {
    type Output = Motor;

    fn div(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Rotor {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<MultiVector> for Rotor {
    type Output = MultiVector;

    fn mul(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other)
    }
}

impl Div<MultiVector> for Rotor {
    type Output = MultiVector;

    fn div(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Rotor {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Rotor> for Rotor {
    type Output = Rotor;

    fn mul(self, other: Rotor) -> Rotor {
        self.geometric_product(other)
    }
}

impl MulAssign<Rotor> for Rotor {
    fn mul_assign(&mut self, other: Rotor) {
        *self = (*self).mul(other);
    }
}

impl Powi for Rotor {
    type Output = Rotor;

    fn powi(self, exponent: isize) -> Rotor {
        if exponent == 0 {
            return Rotor::one();
        }
        let mut x: Rotor = if exponent < 0 { self.inverse() } else { self };
        let mut y: Rotor = Rotor::one();
        let mut n: isize = exponent.abs();
        while 1 < n {
            if n & 1 == 1 {
                y = x.geometric_product(y);
            }
            x = x.geometric_product(x);
            n = n >> 1;
        }
        x.geometric_product(y)
    }
}

impl Div<Rotor> for Rotor {
    type Output = Rotor;

    fn div(self, other: Rotor) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Rotor> for Rotor {
    fn div_assign(&mut self, other: Rotor) {
        *self = (*self).div(other);
    }
}

impl Transformation<Rotor> for Rotor {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Scalar> for Rotor {
    type Output = Rotor;

    fn mul(self, other: Scalar) -> Rotor {
        self.geometric_product(other)
    }
}

impl MulAssign<Scalar> for Rotor {
    fn mul_assign(&mut self, other: Scalar) {
        *self = (*self).mul(other);
    }
}

impl Div<Scalar> for Rotor {
    type Output = Rotor;

    fn div(self, other: Scalar) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Scalar> for Rotor {
    fn div_assign(&mut self, other: Scalar) {
        *self = (*self).div(other);
    }
}

impl Transformation<Scalar> for Rotor {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Translator> for Rotor {
    type Output = Motor;

    fn mul(self, other: Translator) -> Motor {
        self.geometric_product(other)
    }
}

impl Div<Translator> for Rotor {
    type Output = Motor;

    fn div(self, other: Translator) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for Rotor {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Branch> for Scalar {
    type Output = Branch;

    fn mul(self, other: Branch) -> Branch {
        self.geometric_product(other)
    }
}

impl Transformation<Branch> for Scalar {
    type Output = Branch;

    fn transformation(self, other: Branch) -> Branch {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Line> for Scalar {
    type Output = Line;

    fn mul(self, other: Line) -> Line {
        self.geometric_product(other)
    }
}

impl Div<Line> for Scalar {
    type Output = Line;

    fn div(self, other: Line) -> Line {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Line> for Scalar {
    type Output = Line;

    fn transformation(self, other: Line) -> Line {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Motor> for Scalar {
    type Output = Motor;

    fn mul(self, other: Motor) -> Motor {
        self.geometric_product(other)
    }
}

impl Div<Motor> for Scalar {
    type Output = Motor;

    fn div(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Scalar {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<MultiVector> for Scalar {
    type Output = MultiVector;

    fn mul(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other)
    }
}

impl Div<MultiVector> for Scalar {
    type Output = MultiVector;

    fn div(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Scalar {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Plane> for Scalar {
    type Output = Plane;

    fn mul(self, other: Plane) -> Plane {
        self.geometric_product(other)
    }
}

impl Div<Plane> for Scalar {
    type Output = Plane;

    fn div(self, other: Plane) -> Plane {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Plane> for Scalar {
    type Output = Plane;

    fn transformation(self, other: Plane) -> Plane {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Point> for Scalar {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        self.geometric_product(other)
    }
}

impl Div<Point> for Scalar {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Scalar {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Rotor> for Scalar {
    type Output = Rotor;

    fn mul(self, other: Rotor) -> Rotor {
        self.geometric_product(other)
    }
}

impl Div<Rotor> for Scalar {
    type Output = Rotor;

    fn div(self, other: Rotor) -> Rotor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for Scalar {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Scalar> for Scalar {
    type Output = Scalar;

    fn mul(self, other: Scalar) -> Scalar {
        self.geometric_product(other)
    }
}

impl MulAssign<Scalar> for Scalar {
    fn mul_assign(&mut self, other: Scalar) {
        *self = (*self).mul(other);
    }
}

impl Powi for Scalar {
    type Output = Scalar;

    fn powi(self, exponent: isize) -> Scalar {
        if exponent == 0 {
            return Scalar::one();
        }
        let mut x: Scalar = if exponent < 0 { self.inverse() } else { self };
        let mut y: Scalar = Scalar::one();
        let mut n: isize = exponent.abs();
        while 1 < n {
            if n & 1 == 1 {
                y = x.geometric_product(y);
            }
            x = x.geometric_product(x);
            n = n >> 1;
        }
        x.geometric_product(y)
    }
}

impl Div<Scalar> for Scalar {
    type Output = Scalar;

    fn div(self, other: Scalar) -> Scalar {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Scalar> for Scalar {
    fn div_assign(&mut self, other: Scalar) {
        *self = (*self).div(other);
    }
}

impl Transformation<Scalar> for Scalar {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Translator> for Scalar {
    type Output = Translator;

    fn mul(self, other: Translator) -> Translator {
        self.geometric_product(other)
    }
}

impl Div<Translator> for Scalar {
    type Output = Translator;

    fn div(self, other: Translator) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Translator> for Scalar {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Branch> for Translator {
    type Output = Branch;

    fn mul(self, other: Branch) -> Branch {
        self.geometric_product(other)
    }
}

impl Transformation<Branch> for Translator {
    type Output = Branch;

    fn transformation(self, other: Branch) -> Branch {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Motor> for Translator {
    type Output = Motor;

    fn mul(self, other: Motor) -> Motor {
        self.geometric_product(other)
    }
}

impl Div<Motor> for Translator {
    type Output = Motor;

    fn div(self, other: Motor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Motor> for Translator {
    type Output = Motor;

    fn transformation(self, other: Motor) -> Motor {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<MultiVector> for Translator {
    type Output = MultiVector;

    fn mul(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other)
    }
}

impl Div<MultiVector> for Translator {
    type Output = MultiVector;

    fn div(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<MultiVector> for Translator {
    type Output = MultiVector;

    fn transformation(self, other: MultiVector) -> MultiVector {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Point> for Translator {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        self.geometric_product(other)
    }
}

impl Div<Point> for Translator {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Point> for Translator {
    type Output = Point;

    fn transformation(self, other: Point) -> Point {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

impl Mul<Rotor> for Translator {
    type Output = Motor;

    fn mul(self, other: Rotor) -> Motor {
        self.geometric_product(other)
    }
}

impl Div<Rotor> for Translator {
    type Output = Motor;

    fn div(self, other: Rotor) -> Motor {
        self.geometric_product(other.inverse())
    }
}

impl Transformation<Rotor> for Translator {
    type Output = Rotor;

    fn transformation(self, other: Rotor) -> Rotor {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Scalar> for Translator {
    type Output = Translator;

    fn mul(self, other: Scalar) -> Translator {
        self.geometric_product(other)
    }
}

impl MulAssign<Scalar> for Translator {
    fn mul_assign(&mut self, other: Scalar) {
        *self = (*self).mul(other);
    }
}

impl Div<Scalar> for Translator {
    type Output = Translator;

    fn div(self, other: Scalar) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Scalar> for Translator {
    fn div_assign(&mut self, other: Scalar) {
        *self = (*self).div(other);
    }
}

impl Transformation<Scalar> for Translator {
    type Output = Scalar;

    fn transformation(self, other: Scalar) -> Scalar {
        self.geometric_product(other).geometric_product(self.reversal()).into()
    }
}

impl Mul<Translator> for Translator {
    type Output = Translator;

    fn mul(self, other: Translator) -> Translator {
        self.geometric_product(other)
    }
}

impl MulAssign<Translator> for Translator {
    fn mul_assign(&mut self, other: Translator) {
        *self = (*self).mul(other);
    }
}

impl Powi for Translator {
    type Output = Translator;

    fn powi(self, exponent: isize) -> Translator {
        if exponent == 0 {
            return Translator::one();
        }
        let mut x: Translator = if exponent < 0 { self.inverse() } else { self };
        let mut y: Translator = Translator::one();
        let mut n: isize = exponent.abs();
        while 1 < n {
            if n & 1 == 1 {
                y = x.geometric_product(y);
            }
            x = x.geometric_product(x);
            n = n >> 1;
        }
        x.geometric_product(y)
    }
}

impl Div<Translator> for Translator {
    type Output = Translator;

    fn div(self, other: Translator) -> Translator {
        self.geometric_product(other.inverse())
    }
}

impl DivAssign<Translator> for Translator {
    fn div_assign(&mut self, other: Translator) {
        *self = (*self).div(other);
    }
}

impl Transformation<Translator> for Translator {
    type Output = Translator;

    fn transformation(self, other: Translator) -> Translator {
        self.geometric_product(other).geometric_product(self.reversal())
    }
}

