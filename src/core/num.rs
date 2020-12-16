use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use super::*;

pub trait Zero {
    const ZERO: Self;
}

pub trait One {
    const ONE: Self;
}

pub trait Num:
    Copy
    + PartialOrd
    + PartialEq
    + Zero
    + One
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
{
    #[inline(always)]
    fn eq(a: Self, b: Self) -> bool { a == b }
    #[inline(always)]
    fn sq(self) -> Self { self * self }

    #[inline(always)]
    fn abs(a: Self) -> Self {
        if a >= Self::ZERO {
            a
        } else {
            -a
        }
    }

    #[inline(always)]
    fn min(a: Self, b: Self) -> Self {
        if a < b {
            a
        } else {
            b
        }
    }

    #[inline(always)]
    fn max(a: Self, b: Self) -> Self {
        if a < b {
            b
        } else {
            a
        }
    }

    #[inline(always)]
    fn is_pos(a: Self) -> bool { a > Self::ZERO }
    #[inline(always)]
    fn is_nonpos(a: Self) -> bool { !Self::is_pos(a) }

    #[inline(always)]
    fn is_neg(a: Self) -> bool { a < Self::ZERO }
    #[inline(always)]
    fn is_nonneg(a: Self) -> bool { !Self::is_neg(a) }

    #[inline(always)]
    fn clamp(v: Self, a: Self, b: Self) -> Self { Num::min(Num::max(v, a), b) }

    #[inline(always)]
    fn clamp_pos(v: Self) -> Self { Num::max(v, Self::ZERO) }

    #[inline(always)]
    fn clamp_unit(v: Self) -> Self { Num::clamp(v, Self::ZERO, Self::ONE) }

    #[inline(always)]
    fn clamp_one(v: Self) -> Self { Num::clamp(v, -Self::ONE, Self::ONE) }
}

impl Zero for I {
    const ZERO: Self = 0;
}
impl One for I {
    const ONE: Self = 1;
}
impl Num for I {}

impl Zero for F {
    const ZERO: Self = 0.;
}
impl One for F {
    const ONE: Self = 1.;
}
impl Num for F {}

pub trait Integer: Num {
    fn mod2(i: Self) -> Self;
}

impl Integer for I {
    #[inline(always)]
    fn mod2(i: Self) -> Self {
        let r = i % 2;
        if r < 0 {
            r + 2
        } else {
            r
        }
    }
}

pub trait Half: Copy {
    const HALF: Self;
}

pub trait Two: Copy {
    const TWO: Self;
}

pub trait Inv {
    type Output;
    fn inv(self) -> Self;
}

pub trait Epsilon: Copy {
    const EPSILON: Self;
}

pub trait Float: Num + Half + Two + Inv + Epsilon {
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
    fn approx_eq(a: Self, b: Self) -> bool { Self::abs(a - b) < Self::EPSILON }

    #[inline(always)]
    fn approx_zero(a: Self) -> bool { Self::approx_eq(a, Self::ZERO) }

    #[inline(always)]
    fn approx_one(a: Self) -> bool { Self::approx_eq(a, Self::ONE) }
}

impl Half for F {
    const HALF: Self = 0.5;
}
impl Two for F {
    const TWO: Self = 2.;
}
impl Inv for F {
    type Output = F;
    #[inline(always)]
    fn inv(self) -> F { self.recip() }
}
impl Epsilon for f32 {
    const EPSILON: Self = 1e-3;
}
impl Epsilon for f64 {
    const EPSILON: Self = 1e-5;
}

impl Float for F {
    const NEG_INF: Self = fmod::NEG_INFINITY;
    const POS_INF: Self = fmod::INFINITY;

    const PI: Self = fmod::consts::PI;
    const HALF_PI: Self = fmod::consts::FRAC_PI_2;
    const FOURTH_PI: Self = fmod::consts::FRAC_PI_4;
    const TWO_PI: Self = Self::TWO * Self::PI;
    const FOUR_PI: Self = Self::TWO * Self::TWO_PI;
    const INV_PI: Self = fmod::consts::FRAC_1_PI;
    const INV_2PI: Self = Self::HALF * Self::INV_PI;
    const INV_4PI: Self = Self::HALF * Self::INV_2PI;

    const FRAC_1_2POW32: Self = 2.328_306_4e-10;

    #[inline(always)]
    fn ceili(self) -> I { self.ceil() as I }
    #[inline(always)]
    fn floori(self) -> I { self.floor() as I }

    #[inline(always)]
    fn exp(f: Self) -> Self { f.exp() }
    #[inline(always)]
    fn sqrt(self) -> Self { self.sqrt() }

    #[inline(always)]
    fn sin(self) -> Self { self.sin() }
    #[inline(always)]
    fn cos(self) -> Self { self.cos() }
    #[inline(always)]
    fn tan(self) -> Self { self.tan() }
    #[inline(always)]
    fn sind(self) -> Self { self.to_radians().sin() }
    #[inline(always)]
    fn cosd(self) -> Self { self.to_radians().cos() }
    #[inline(always)]
    fn tand(self) -> Self { self.to_radians().tan() }

    #[inline(always)]
    fn discrete(a: Self, n: I) -> I {
        Num::min(Self::floori(a * n as Self), n - 1)
    }
}

#[inline(always)]
pub fn difference_of_products(a: F, b: F, c: F, d: F) -> F {
    let cd = c * d;
    let err = (-c).mul_add(d, cd);
    let dop = a.mul_add(b, -cd);
    dop + err
}

#[inline(always)]
pub fn quad(a: F, b: F, c: F) -> Option<F2> {
    let dis = difference_of_products(b, b, 4. * a, c);
    if dis < 0. {
        return None
    }
    let disqrt = dis.sqrt();
    let q = -0.5 * (b + b.signum() * disqrt);
    let t1 = q / a;
    let t2 = c / q;
    if t1 <= t2 {
        Some(A2(t1, t2))
    } else {
        Some(A2(t2, t1))
    }
}
