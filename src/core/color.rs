use std::iter::{Product, Sum};
use std::ops::*;

use super::*;
use crate::op;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(F3);

impl Zero for Color { const ZERO: Self = Self(F3::ZERO); }
impl One for Color { const ONE: Self = Self(F3::ONE); }

impl Color {
    pub fn rgb(rgb: F3) -> Self { Self(rgb) }
    pub fn gray(g: F) -> Self { Self(F3::rep(g)) }
    pub fn max_channel(self) -> F { self.0.max() }
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

// TODO implement indexing + with R, G, B + spectral channels

impl Sum for Color {
    fn sum<It>(it: It) -> Self where It: Iterator<Item=Self>
    { Color(it.map(|i| i.0).sum()) }
}

impl Product for Color {
    fn product<It>(it: It) -> Self where It: Iterator<Item=Self>
    { Color(it.map(|i| i.0).product()) }
}
