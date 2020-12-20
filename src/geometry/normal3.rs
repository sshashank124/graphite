use std::ops::{Add, Div, Index, Mul, Neg};

use super::*;
use crate::op;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct N(V);

impl Zero for N { const ZERO: Self = N(V::ZERO); }

impl N {
    pub fn v(v: V) -> N { N(v.unit()) }
}

op!(Neg::neg, *N);
op!(Add::add, *N -> *N -> N);
op!(Mul::mul, *N ->  F -> N);

impl Mul<N> for T {
    type Output = N;
    fn mul(self, n: N) -> N { N::v(self.inv().t() * n.0) }
}

impl Div<N> for T {
    type Output = N;
    fn div(self, n: N) -> N { N::v(self.inv().t() / n.0) }
}

impl From<F3> for N { fn from(f3: F3) -> Self { Self(V::from(f3)) } }
impl From<N> for F3 { fn from(n: N) -> Self { F3::from(n.0) } }
impl From<V> for N { fn from(v: V) -> Self { Self(v) } }

impl Index<Dim> for N {
    type Output = F;
    fn index(&self, dim: Dim) -> &F { &self.0[dim] }
}
