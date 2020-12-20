use std::ops::{Add, BitAnd, BitOr, Div, Mul, Sub};

use super::*;
use crate::op;

#[derive(Clone, Copy, Debug)]
pub struct BBox(A3<B>);

impl Zero for BBox { const ZERO: Self = BBox(Arr::ZERO); }

impl BBox {
    pub fn center(&self) -> P { P(self.0.map(B::center)) }
    pub fn extents(&self) -> F3 { self.0.map(B::extent) }

    pub fn max_extent(&self) -> (F, Dim) {
        self.extents().zip(XYZ, |a, b| (a, b))
                      .fold((F::NEG_INF, X), |(a, b), (c, d)|
                          if a > c { (a, b) } else { (c, d) })
    }
}

op!(Add::add, *BBox -> *P -> BBox);
op!(Sub::sub, *BBox -> *P -> BBox);
op!(Mul::mul, *BBox -> *V -> BBox);
op!(Div::div, *BBox -> *V -> BBox);

op!(BitAnd::bitand, *BBox -> *BBox -> BBox);
op!(BitOr::bitor, *BBox -> *BBox -> BBox);
op!(BitOr::bitor, *BBox ->    *P -> BBox);

op!(Mul::mul, T -> *BBox -> BBox);
op!(Div::div, T -> *BBox -> BBox);
