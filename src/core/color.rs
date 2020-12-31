use std::iter::{Product, Sum};
use std::ops::*;

use super::*;
use crate::op;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature="serde-derive", derive(Deserialize, Serialize))]
#[repr(C)]
pub struct Color(F3);

impl Zero for Color { const ZERO: Self = Self(F3::ZERO); }
impl One for Color { const ONE: Self = Self(F3::ONE); }

impl Color {
    #[inline(always)] pub fn rgb(rgb: F3) -> Self { Self(rgb) }
    #[inline(always)] pub fn gray(g: F) -> Self { Self(F3::rep(g)) }
    #[inline(always)] pub fn max_channel(self) -> F { self.0.max() }
    #[inline(always)] pub fn to_rgb(self) -> F3 { self.0 }
}

op!(Neg::neg, *Color);
op!(Inv::inv, *Color);
op!(Add::add, *Color -> *Color -> Color);
op!(Sub::sub, *Color -> *Color -> Color);
op!(Mul::mul, *Color -> *Color -> Color);
op!(Div::div, *Color -> *Color -> Color);
op!(AddAssign::add_assign, *mut Color -> *Color -> ());
op!(SubAssign::sub_assign, *mut Color -> *Color -> ());
op!(MulAssign::mul_assign, *mut Color -> *Color -> ());
op!(DivAssign::div_assign, *mut Color -> *Color -> ());
op!(Add::add, *Color -> F -> Color);
op!(Sub::sub, *Color -> F -> Color);
op!(Mul::mul, *Color -> F -> Color);
op!(Div::div, *Color -> F -> Color);
op!(AddAssign::add_assign, *mut Color -> F -> ());
op!(SubAssign::sub_assign, *mut Color -> F -> ());
op!(MulAssign::mul_assign, *mut Color -> F -> ());
op!(DivAssign::div_assign, *mut Color -> F -> ());

impl Sum for Color {
    #[inline(always)]
    fn sum<It>(it: It) -> Self where It: Iterator<Item=Self>
    { Color(it.map(|i| i.0).sum()) }
}

impl Product for Color {
    #[inline(always)]
    fn product<It>(it: It) -> Self where It: Iterator<Item=Self>
    { Color(it.map(|i| i.0).product()) }
}


#[cfg(feature="serde-derive")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn rgb() {
        assert_eq!(serde_json::from_str::<Color>("[-1, 0.5, 1]").unwrap(),
                   Color(A3(-1., 0.5, 1.)));
    }
}
