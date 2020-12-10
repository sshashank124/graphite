use std::ops::{Add, BitAnd, BitOr, Deref, Div, Mul, Sub};

use super::*;
use crate::op;

#[derive(Clone, Copy)]
pub struct BBox(pub A3<B>);

impl Zero for BBox {
    const ZERO: Self = BBox(A3(B::ZERO, B::ZERO, B::ZERO));
}

impl BBox {
    pub fn center(&self) -> P { P(self.map(B::center)) }
    pub fn extents(&self) -> F3 { self.map(B::extent) }

    pub fn max_extent(&self) -> (F, Dim) {
        self.extents().zip(A3(X, Y, Z), |a, b| (a, b))
                      .fold((F::NEG_INF, X),
                            |(a, b), (c, d)| if a > c { (a, b) }
                                             else { (c, d) })
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

impl Deref for BBox {
    type Target = A3<B>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.0 }
}
