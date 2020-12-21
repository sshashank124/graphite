use std::ops::{Add, Div, Index, Mul, Neg};

use super::*;
use crate::op;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct N(pub(crate) V);

impl Zero for N { const ZERO: Self = N(V::ZERO); }

op!(Neg::neg, *N);
op!(Add::add, *N -> *N -> N);
op!(Mul::mul, *N ->  F -> N);

impl Mul<N> for T {
    type Output = N;
    fn mul(self, n: N) -> N { N(self.inv().t() * n.0) }
}

impl Div<N> for T {
    type Output = N;
    fn div(self, n: N) -> N { N(self.inv().t() / n.0) }
}

impl From<F3> for N { fn from(f3: F3) -> Self { Self(V(f3)) } }
impl From<N> for F3 { fn from(n: N) -> Self { n.0.0 } }
impl From<V> for N { fn from(v: V) -> Self { Self(v.unit()) } }
impl From<N> for V { fn from(n: N) -> Self { n.0 } }

impl Index<Dim> for N {
    type Output = F;
    fn index(&self, dim: Dim) -> &F { &self.0[dim] }
}
