use std::ops::*;

use super::*;

pub trait Zero { const ZERO: Self; }
pub trait One { const ONE: Self; }
pub trait Two { const TWO: Self; }
pub trait Half { const HALF: Self; }
pub trait OneThird { const ONE_THIRD: Self; }
pub trait Inv { type Output; fn inv(self) -> Self; }
pub trait Epsilon: Copy { const EPS: Self; }

pub trait Num: Copy + PartialEq + PartialOrd
             + Zero + One + Two + Neg<Output = Self>
             + Add<Self, Output = Self> + AddAssign<Self>
             + Sub<Self, Output = Self> + SubAssign<Self>
             + Mul<Self, Output = Self> + MulAssign<Self>
             + Div<Self, Output = Self> + DivAssign<Self>
{
    #[inline(always)] fn sq(self) -> Self { self * self }
}

pub trait Float: Num + Half + OneThird + Inv + Epsilon {
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

    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn sind(self) -> Self;
    fn cosd(self) -> Self;
    fn tand(self) -> Self;

    fn discrete(a: Self, n: I) -> I;
}

#[inline] pub fn difference_of_products(a: F, b: F, c: F, d: F) -> F {
    let cd = c * d;
    a.mul_add(b, -cd) + c.mul_add(-d, cd)
}

#[inline] pub fn quad(a: F, b: F, c: F) -> Option<F2> {
    let dis = difference_of_products(b, b, 4. * a, c);
    if dis < 0. { return None }
    let disqrt = dis.sqrt();
    let q = -0.5 * b.signum().mul_add(disqrt, b);
    let t1 = q / a;
    let t2 = c / q;
    Some(if t1 <= t2 { A2(t1, t2) } else { A2(t2, t1) })
}
