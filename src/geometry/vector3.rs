use std::ops::{Add, Div, Index, Mul, Neg, Sub};

use super::*;
use crate::op;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V(pub(crate) F3);

impl Zero for V { const ZERO: Self = V(F3::ZERO); }

impl V {
    #[inline(always)] pub fn norm2(self) -> F { dot(self, self) }
    #[inline(always)] pub fn norm(self) -> F { self.norm2().sqrt() }
    #[inline(always)] pub fn unit(self) -> V { self / self.norm() }

    #[inline(always)] fn cross(self, v: V) -> V
    { V(self.0.shl() * v.0.shr() - self.0.shr() * v.0.shl()) }
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

impl From<[F; 3]> for V
{ #[inline(always)] fn from(f3: [F; 3]) -> Self { Self(Arr(f3)) } }
impl From<F3> for V { #[inline(always)] fn from(f3: F3) -> Self { Self(f3) } }
impl From<V> for F3 { #[inline(always)] fn from(v: V) -> Self { v.0 } }

impl Index<Dim> for V {
    type Output = F;
    #[inline(always)] fn index(&self, dim: Dim) -> &F { &self.0[dim] }
}
