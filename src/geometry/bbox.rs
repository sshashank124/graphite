use std::ops::{Add, BitAnd, BitOr, Div, Index, Mul, Sub};

use super::*;
use crate::op;

#[derive(Clone, Copy, Debug)]
pub struct BBox(pub A3<B>);

impl Zero for BBox { const ZERO: Self = BBox(Arr::ZERO); }

impl BBox {
    #[inline] pub fn center(&self) -> P { P(map(self.0, B::center)) }
    #[inline] pub fn extents(&self) -> F3 { map(self.0, B::extent) }

    #[inline] pub fn max_extent(&self) -> (F, Dim) {
        self.extents().zip(XYZ, |a, b| (a, b))
            .reduce(|(a, b), (c, d)| if a > c { (a, b) } else { (c, d) })
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

impl Index<Dim> for BBox {
    type Output = B;
    #[inline] fn index(&self, dim: Dim) -> &B { &self.0[dim] }
}
