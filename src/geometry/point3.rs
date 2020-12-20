use std::ops::{Add, Div, Index, Mul, Sub};

use super::*;
use crate::op;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct P(pub F3);

impl Zero for P { const ZERO: P = P(F3::ZERO); }

op!(Add::add, *P -> *P -> P);
op!(Add::add, *P -> *V -> P);
op!(Add::add, *V -> *P -> P);
op!(Add::add, *P ->  F -> P);
op!(Sub::sub, *P -> *V -> P);
op!(Sub::sub, *P ->  F -> P);
op!(Mul::mul, *P ->  F -> P);
op!(Mul::mul,  T -> *P -> P);
op!(Div::div,  T -> *P -> P);

impl From<F3> for P { fn from(f3: F3) -> Self { Self(f3) } }
impl From<P> for F3 { fn from(p: P) -> Self { p.0 } }

impl Index<Dim> for P {
    type Output = F;
    fn index(&self, dim: Dim) -> &F { &self.0[dim] }
}
