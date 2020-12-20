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
    fn eq(a: Self, b: Self) -> bool { a == b }

    fn sq(self) -> Self { self * self }

    fn abs(a: Self) -> Self { if a >= Self::ZERO { a } else { -a } }

    fn min(a: Self, b: Self) -> Self { if a < b { a } else { b } }
    fn max(a: Self, b: Self) -> Self { if a < b { b } else { a } }

    fn is_pos(a: Self) -> bool { a > Self::ZERO }
    fn is_nonpos(a: Self) -> bool { !Self::is_pos(a) }

    fn is_neg(a: Self) -> bool { a < Self::ZERO }
    fn is_nonneg(a: Self) -> bool { !Self::is_neg(a) }

    fn clamp(v: Self, a: Self, b: Self) -> Self { Num::min(Num::max(v, a), b) }
    fn clamp_pos(v: Self) -> Self { Num::max(v, Self::ZERO) }
    fn clamp_unit(v: Self) -> Self { Num::clamp(v, Self::ZERO, Self::ONE) }
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

    fn approx_eq(a: Self, b: Self) -> bool { Self::abs(a - b) < Self::EPS }
    fn approx_zero(a: Self) -> bool { Self::approx_eq(a, Self::ZERO) }
    fn approx_one(a: Self) -> bool { Self::approx_eq(a, Self::ONE) }
}

pub fn difference_of_products(a: F, b: F, c: F, d: F) -> F {
    let cd = c * d;
    a.mul_add(b, -cd) + c.mul_add(-d, cd)
}

pub fn quad(a: F, b: F, c: F) -> Option<F2> {
    let dis = difference_of_products(b, b, 4. * a, c);
    if dis < 0. { return None }
    let disqrt = dis.sqrt();
    let q = -0.5 * (b + b.signum() * disqrt);
    let t1 = q / a;
    let t2 = c / q;
    Some(if t1 <= t2 { [t1, t2] } else { [t2, t1] }.into())
}
