use std::ops::{Div, Mul};

use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct R {
    pub o: P,
    pub d: V,
    pub t: F,
}

impl R {
    pub fn r(o: P, d: V, t: F) -> R { R { o, d, t } }
    pub fn unbounded(o: P, d: V) -> R { R::r(o, d.unit(), F::POS_INF) }
    pub fn unit(o: P, d: V) -> R { R::r(o, d.unit(), d.norm()) }
    pub fn p2(a: P, b: P) -> R { R::unit(a, b - a) }
    pub fn at(&self, t: F) -> P { self.o + self.d * t }
    pub fn clipped(self, t: F) -> R { R::r(self.o, self.d, t) }
    pub fn range(&self) -> B { B::b(F::EPS, self.t - F::EPS) }
}

impl Mul<R> for T {
    type Output = R;
    fn mul(self, R { o, d, t }: R) -> R { R::r(self * o, self * d, t) }
}

impl Div<R> for T {
    type Output = R;
    fn div(self, R { o, d, t }: R) -> R { R::r(self / o, self / d, t) }
}
