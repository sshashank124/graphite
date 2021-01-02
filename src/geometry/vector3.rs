use std::ops::{Add, Div, Index, Mul, Neg, Sub};

use super::*;
use crate::op;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature="serde-derive", derive(Deserialize, Serialize))]
#[repr(C)]
pub struct V(pub(crate) F3);

impl Zero for V { const ZERO: Self = V(F3::ZERO); }

impl V {
    #[inline] pub fn norm2(self) -> F { F3::dot(self, self) }
    #[inline] pub fn norm(self) -> F { self.norm2().sqrt() }
    #[inline] pub fn unit(self) -> V { self / self.norm() }

    #[inline] fn cross(self, v: V) -> V
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
{ type Output = V; #[inline] fn mul(self, v: V) -> V { self.cross(v) } }

impl Mul<V> for T {
    type Output = V;
    #[inline] fn mul(self, v: V) -> V { V(self.rot() * v.0) }
}

impl Div<V> for T {
    type Output = V;
    #[inline] fn div(self, v: V) -> V { V(self.rot() / v.0) }
}

impl From<F3> for V { #[inline] fn from(f3: F3) -> Self { Self(f3) } }
impl From<V> for F3 { #[inline] fn from(v: V) -> Self { v.0 } }

impl Index<Dim> for V {
    type Output = F;
    #[inline] fn index(&self, dim: Dim) -> &F { &self.0[dim] }
}


#[cfg(feature="serde-derive")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn v() {
        assert_eq!(serde_json::from_str::<V>("[-1, 1, 0.5]").unwrap(),
                   V(A3(-1., 1., 0.5)));
    }
}
