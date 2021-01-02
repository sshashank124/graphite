use std::ops::{Add, BitAnd, BitOr, Div, Index, Mul, Sub};

use super::*;
use crate::op;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature="serde-derive", derive(Deserialize, Serialize))]
#[repr(C)]
pub struct B(pub(crate) F2);

impl Zero for B { const ZERO: Self = B::b(F::POS_INF, F::NEG_INF); }

impl B {
    #[inline] pub const fn b(l: F, u: F) -> B { B(A2(l, u)) }
    #[inline] pub fn point(f: F) -> B { B(F2::rep(f)) }
    #[inline] pub fn ordered(a: F2) -> B
    { if a[0] > a[1] { B(a.flip()) } else { B(a) } }

    #[inline] pub fn bounds(self, t: F) -> bool
    { self.0[0] <= t && t <= self.0[1] }
    #[inline] pub fn degen(self) -> bool { self.0[0] > self.0[1] }

    #[inline] pub fn center(self) -> F { self.0.mean() }
    #[inline] pub fn extent(self) -> F { self.0[1] - self.0[0] }
}

op!(Add::add, *B -> *B -> B);
op!(Add::add, *B ->  F -> B);
op!(Sub::sub, *B ->  F -> B);

impl Mul<F> for B {
    type Output = B;
    #[inline] fn mul(self, f: F) -> B { B::ordered(self.0 * f) }
}

impl Div<F> for B {
    type Output = B;
    #[inline] fn div(self, f: F) -> B { self * f.inv() }
}

impl BitOr for B {
    type Output = B;
    #[inline] fn bitor(self, b: B) -> B {
        B::b(F::min(self.0[0], b.0[0]), F::max(self.0[1], b.0[1]))
    }
}

impl BitOr<F> for B {
    type Output = B;
    #[inline] fn bitor(self, f: F) -> B { self | B::point(f) }
}

impl BitAnd for B {
    type Output = B;
    #[inline] fn bitand(self, b: B) -> B {
        B::b(F::max(self.0[0], b.0[0]), F::min(self.0[1], b.0[1]))
    }
}

impl Index<usize> for B {
    type Output = F;
    #[inline] fn index(&self, i: usize) -> &F { &self.0[i] }
}


#[cfg(feature="serde-derive")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn b() {
        assert_eq!(serde_json::from_str::<B>("[-1, 1]").unwrap(),
                   B::b(-1., 1.));
    }
}
