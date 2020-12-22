use super::*;
use rotscale3::RotScale3;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Affine3 {
    r: RotScale3,
    t: Option<F3>,
}

impl One for Affine3 { const ONE: Self = Self::new(RotScale3::ONE, None); }

impl Affine3 {
    #[inline(always)] pub const fn new(r: RotScale3, t: Option<F3>) -> Self { Self { r, t } }

    #[inline(always)] pub fn translate(t: F3) -> Self { Self::new(RotScale3::ONE, Some(t)) }
    #[inline(always)] pub fn scale(s: F3) -> Self
    { Self::new(RotScale3::scale(s), None) }

    #[inline(always)] pub fn rotate(axis: F3, theta: F) -> Self
    { Self::new(RotScale3::rotate(axis, theta), None) }

    #[inline(always)] pub fn from_frame(v: V) -> Self
    { Self::new(RotScale3::from_frame(v), None) }

    #[inline(always)] pub fn look_at(pos: P, target: P, up: V) -> Self
    { Self::new(RotScale3::look_at(target - pos, up), Some(pos.0)) }

    #[inline(always)] pub fn rot(&self) -> Self { Self::new(self.r, None) }
    #[inline(always)] pub fn t(&self) -> Self { Self::new(self.r.t(), None) }
}

impl<A> Mul<A3<A>> for Affine3
    where A: Copy + Zero + Add<F, Output = A> + Add<Output = A>
           + Mul<F, Output = A>
{
    type Output = A3<A>;
    #[inline(always)] fn mul(self, o: A3<A>) -> A3<A> {
        let r = self.r * o;
        self.t.map(|t| r.zip(t, Add::add)).unwrap_or_else(|| r)
    }
}

impl Mul for Affine3 {
    type Output = Self;
    #[inline(always)] fn mul(self, o: Self) -> Self {
        let r = self.r * o.r;
        let t = o.t.map(|ot| Some(self * ot)).unwrap_or_else(|| self.t);
        Self::new(r, t)
    }
}
