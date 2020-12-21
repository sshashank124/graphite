use std::ops::{Add, BitAnd, BitOr, Div, Mul, Sub};

use super::*;
use crate::op;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct B(pub(crate) F2);

impl Zero for B { const ZERO: Self = B::b(F::POS_INF, F::NEG_INF); }

impl B {
    pub const fn b(l: F, u: F) -> B { B(Arr([l, u])) }
    pub fn point(f: F) -> B { B(F2::rep(f)) }
    pub fn ordered(a: F2) -> B { if a[0] > a[1] { B(a.shl()) } else { B(a) } }

    pub fn bounds(self, t: F) -> bool { self.0[0] <= t && t <= self.0[1] }
    pub fn degen(self) -> bool { self.0[0] > self.0[1] }

    pub fn center(self) -> F { self.0.mean() }
    pub fn extent(self) -> F { self.0[1] - self.0[0] }
}

op!(Add::add, *B -> *B -> B);
op!(Add::add, *B ->  F -> B);
op!(Sub::sub, *B ->  F -> B);

impl Mul<F> for B {
    type Output = B;
    fn mul(self, f: F) -> B { B::ordered(self.0 * f) }
}

impl Div<F> for B {
    type Output = B;
    fn div(self, f: F) -> B { self * f.inv() }
}

impl BitOr for B {
    type Output = B;
    fn bitor(self, b: B) -> B {
        B::b(F::min(self.0[0], b.0[0]), F::max(self.0[1], b.0[1]))
    }
}

impl BitOr<F> for B {
    type Output = B;
    fn bitor(self, f: F) -> B { self | B::point(f) }
}

impl BitAnd for B {
    type Output = B;
    fn bitand(self, b: B) -> B {
        B::b(F::max(self.0[0], b.0[0]), F::min(self.0[1], b.0[1]))
    }
}
