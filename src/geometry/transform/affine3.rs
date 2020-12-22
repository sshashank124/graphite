use super::*;
use rotscale3::RotScale3;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Affine3 {
    r: RotScale3,
    t: F3,
}

impl One for Affine3 { const ONE: Self = Self::new(RotScale3::ONE, F3::ZERO); }

impl Affine3 {
    #[inline] pub const fn new(r: RotScale3, t: F3) -> Self { Self { r, t } }

    #[inline] pub fn translate(t: F3) -> Self { Self::new(RotScale3::ONE, t) }
    #[inline] pub fn scale(s: F3) -> Self
    { Self::new(RotScale3::scale(s), F3::ZERO) }

    #[inline] pub fn rotate(axis: F3, theta: F) -> Self
    { Self::new(RotScale3::rotate(axis, theta), F3::ZERO) }

    #[inline] pub fn from_frame(v: V) -> Self
    { Self::new(RotScale3::from_frame(v), F3::ZERO) }

    #[inline] pub fn look_at(pos: P, target: P, up: V) -> Self
    { Self::new(RotScale3::look_at(target - pos, up), pos.0) }

    #[inline] pub fn rot(&self) -> Self { Self::new(self.r, F3::ZERO) }
    #[inline] pub fn t(&self) -> Self { Self::new(self.r.t(), F3::ZERO) }
}

impl<A> Mul<A3<A>> for Affine3
    where A: Copy + Zero + Add<F, Output = A> + Add<Output = A>
           + Mul<F, Output = A>
{
    type Output = A3<A>;
    #[inline] fn mul(self, o: A3<A>) -> A3<A>
    { (self.r * o).zip(self.t, Add::add) }
}

impl Mul for Affine3 {
    type Output = Self;
    #[inline] fn mul(self, o: Self) -> Self
    { Self::new(self.r * o.r, self * o.t) }
}
