use std::ops::{Add, BitAnd, BitOr, Div, Index, Mul, Sub};

use super::*;
use crate::op;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct BBox(pub A3<B>);

impl Zero for BBox { const ZERO: Self = BBox(A3::ZERO); }

impl BBox {
    #[inline(always)] pub fn center(&self) -> P { P(self.0.map(B::center)) }
    #[inline(always)] pub fn extents(&self) -> F3 { self.0.map(B::extent) }

    #[inline(always)] pub fn max_extent(&self) -> (F, Dim) {
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
    #[inline(always)] fn index(&self, dim: Dim) -> &B { &self.0[dim] }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn bbox() {
        let s = "[[-1, 1], [2, 10], [0.5, -0.5]]";
        assert_eq!(serde_json::from_str::<BBox>(s).unwrap(),
                   BBox(A3(B::b(-1., 1.), B::b(2., 10.), B::b(0.5, -0.5))));
    }
}
