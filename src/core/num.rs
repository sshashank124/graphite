use std::ops::*;

use super::*;

pub trait Zero { const ZERO: Self; }
pub trait One { const ONE: Self; }
pub trait Two: Copy { const TWO: Self; }
pub trait Half: Copy { const HALF: Self; }
pub trait Inv { type Output; fn inv(self) -> Self; }
pub trait Epsilon: Copy { const EPS: Self; }

pub trait Num: Copy + PartialOrd + PartialEq
             + Zero + One + Two + Neg<Output = Self>
             + Add<Self, Output = Self> + AddAssign<Self>
             + Sub<Self, Output = Self> + SubAssign<Self>
             + Mul<Self, Output = Self> + MulAssign<Self>
             + Div<Self, Output = Self> + DivAssign<Self>
{
    #[inline(always)] fn eq(a: Self, b: Self) -> bool { a == b }

    #[inline(always)] fn sq(self) -> Self { self * self }

    #[inline(always)] fn abs(a: Self) -> Self { if a >= Self::ZERO { a } else { -a } }

    #[inline(always)] fn min(a: Self, b: Self) -> Self { if a < b { a } else { b } }
    #[inline(always)] fn max(a: Self, b: Self) -> Self { if a < b { b } else { a } }

    #[inline(always)] fn is_pos(a: Self) -> bool { a > Self::ZERO }
    #[inline(always)] fn is_nonpos(a: Self) -> bool { !Self::is_pos(a) }

    #[inline(always)] fn is_neg(a: Self) -> bool { a < Self::ZERO }
    #[inline(always)] fn is_nonneg(a: Self) -> bool { !Self::is_neg(a) }

    #[inline(always)]
    fn clamp(v: Self, a: Self, b: Self) -> Self { Num::min(Num::max(v, a), b) }
    #[inline(always)]
    fn clamp_pos(v: Self) -> Self { Num::max(v, Self::ZERO) }
    #[inline(always)]
    fn clamp_unit(v: Self) -> Self { Num::clamp(v, Self::ZERO, Self::ONE) }
    #[inline(always)]
    fn clamp_one(v: Self) -> Self { Num::clamp(v, -Self::ONE, Self::ONE) }
}

pub trait Float: Num + Half + Inv + Epsilon {
    const NEG_INF: Self;
    const POS_INF: Self;

    const PI: Self;
    const HALF_PI: Self;
    const FOURTH_PI: Self;
    const TWO_PI: Self;
    const FOUR_PI: Self;
    const INV_PI: Self;
    const INV_2PI: Self;
    const INV_4PI: Self;

    const FRAC_1_2POW32: Self;

    fn ceili(self) -> I;
    fn floori(self) -> I;

    fn exp(f: Self) -> Self;
    fn sqrt(self) -> Self;

    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn sind(self) -> Self;
    fn cosd(self) -> Self;
    fn tand(self) -> Self;

    fn discrete(a: Self, n: I) -> I;

    #[inline(always)]
    fn approx_eq(a: Self, b: Self) -> bool { Self::abs(a - b) < Self::EPS }
    #[inline(always)]
    fn approx_zero(a: Self) -> bool { Self::approx_eq(a, Self::ZERO) }
    #[inline(always)]
    fn approx_one(a: Self) -> bool { Self::approx_eq(a, Self::ONE) }
}

#[inline(always)]
pub fn difference_of_products(a: F, b: F, c: F, d: F) -> F {
    let cd = c * d;
    a.mul_add(b, -cd) + c.mul_add(-d, cd)
}

#[inline(always)]
pub fn quad(a: F, b: F, c: F) -> Option<F2> {
    let dis = difference_of_products(b, b, 4. * a, c);
    if dis < 0. { return None }
    let disqrt = dis.sqrt();
    let q = -0.5 * (b + b.signum() * disqrt);
    let t1 = q / a;
    let t2 = c / q;
    Some(Arr(if t1 <= t2 { [t1, t2] } else { [t2, t1] }))
}
