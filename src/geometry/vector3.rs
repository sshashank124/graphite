use std::ops::{Add, Div, Index, Mul, Neg, Sub};

use super::*;
use crate::op;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature="serde-derive", derive(Deserialize, Serialize))]
#[repr(C)]
pub struct V(pub(crate) F3);

impl Zero for V { const ZERO: Self = V(F3::ZERO); }

impl V {
    #[inline(always)] pub fn norm2(self) -> F { F3::dot(self.0, self.0) }
    #[inline(always)] pub fn norm(self) -> F { self.norm2().sqrt() }
    #[inline(always)] pub fn unit(self) -> V { self / self.norm() }

    #[inline(always)] fn cross(self, v: V) -> V
    { V(self.0.swizzle(1, 2, 0) * v.0.swizzle(2, 0, 1)
      - self.0.swizzle(2, 0, 1) * v.0.swizzle(1, 2, 0)) }
}

op!(Neg::neg, *V);
op!(Add::add, *V -> *V -> V);
op!(Add::add, *P -> *V -> P);
op!(Add::add, *V -> *P -> P);
op!(Sub::sub, *V -> *V -> V);
op!(Sub::sub, *P -> *P -> V);
op!(Sub::sub, *P -> *V -> P);
op!(Mul::mul, *V ->  F -> V);
op!(Div::div, *V ->  F -> V);

impl Mul for V
{ type Output = V; #[inline(always)] fn mul(self, v: V) -> V { self.cross(v) } }

impl Mul<V> for T {
    type Output = V;
    #[inline(always)] fn mul(self, v: V) -> V { V(self.rot() * v.0) }
}

impl Div<V> for T {
    type Output = V;
    #[inline(always)] fn div(self, v: V) -> V { V(self.rot() / v.0) }
}

impl Conv<V> for F3 { #[inline(always)] fn conv(self) -> V { V(self) } }
impl Conv<F3> for V { #[inline(always)] fn conv(self) -> F3 { self.0 } }

impl Index<Dim> for V {
    type Output = F;
    #[inline(always)] fn index(&self, dim: Dim) -> &F { &self.0[dim] }
}


#[cfg(feature="serde-derive")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn deser_v() {
        assert_eq!(serde_json::from_str::<V>("[-1, 1, 0.5]").unwrap(),
                   V(A3(-1., 1., 0.5)));
    }
}
