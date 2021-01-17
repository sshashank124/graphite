use std::ops::{Add, Div, Index, Mul, Neg};

use super::*;
use crate::{conv, op};

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature="serde-derive", derive(Deserialize, Serialize))]
#[repr(C)]
pub struct N(pub(crate) V);

impl Zero for N { const ZERO: Self = N(V::ZERO); }

op!(Neg::neg, *N);
op!(Add::add, *N -> *N -> N);
op!(Mul::mul, *N ->  F -> N);

impl Mul<N> for T {
    type Output = N;
    #[inline] fn mul(self, n: N) -> N { conv!(self.inv().t() * n.0 => N) }
}

impl Div<N> for T {
    type Output = N;
    #[inline] fn div(self, n: N) -> N { conv!(self.inv().t() / n.0 => N) }
}

impl Conv<N> for V { #[inline] fn conv(self) -> N { N(self.unit()) } }
impl Conv<V> for N { #[inline] fn conv(self) -> V { self.0 } }

impl Conv<N> for F3 { #[inline] fn conv(self) -> N { conv!(self => V => N) } }
impl Conv<F3> for N { #[inline] fn conv(self) -> F3 { conv!(self => V => F3) } }

impl Index<Dim> for N {
    type Output = F;
    #[inline] fn index(&self, dim: Dim) -> &F { &self.0[dim] }
}


#[cfg(feature="serde-derive")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn deser_n() {
        assert_eq!(serde_json::from_str::<N>("[-1, 1, 0.5]").unwrap(),
                   conv!(A3(-1., 1., 0.5) => V => N));
    }
}
