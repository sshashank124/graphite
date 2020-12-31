use std::ops::{Add, Div, Index, Mul, Sub};

use super::*;
use crate::op;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature="serde-derive", derive(Deserialize, Serialize))]
pub struct P(pub(crate) F3);

impl Zero for P { const ZERO: P = P(F3::ZERO); }

op!(Add::add, *P -> *P -> P);
op!(Add::add, *P ->  F -> P);
op!(Sub::sub, *P ->  F -> P);
op!(Mul::mul, *P ->  F -> P);
op!(Mul::mul,  T -> *P -> P);
op!(Div::div,  T -> *P -> P);

impl From<F3> for P { #[inline(always)] fn from(f3: F3) -> Self { Self(f3) } }
impl From<P> for F3 { #[inline(always)] fn from(p: P) -> Self { p.0 } }

impl Index<Dim> for P {
    type Output = F;
    #[inline(always)] fn index(&self, dim: Dim) -> &F { &self.0[dim] }
}


#[cfg(feature="serde-derive")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn p() {
        assert_eq!(serde_json::from_str::<P>("[-1, 1, 0.5]").unwrap(),
                   P(A3(-1., 1., 0.5)));
    }
}
