use super::*;

impl Zero for I { const ZERO: Self = 0; }
impl One for I { const ONE: Self = 1; }
impl Two for I { const TWO: Self = 2; }
impl Num for I { }

impl Zero for F { const ZERO: Self = 0.; }
impl One for F { const ONE: Self = 1.; }
impl Two for F { const TWO: Self = 2.; }
impl Half for F { const HALF: Self = 0.5; }
impl Num for F { }

impl Inv for F { type Output = F; #[inline(always)] fn inv(self) -> F { self.recip() } }
impl Epsilon for f32 { const EPS: Self = 1e-4; }
impl Epsilon for f64 { const EPS: Self = 1e-6; }

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

    #[inline(always)] fn ceili(self) -> I { self.ceil() as I }
    #[inline(always)] fn floori(self) -> I { self.floor() as I }

    #[inline(always)] fn exp(f: Self) -> Self { f.exp() }
    #[inline(always)] fn sqrt(self) -> Self { self.sqrt() }

    #[inline(always)] fn sin(self) -> Self { self.sin() }
    #[inline(always)] fn cos(self) -> Self { self.cos() }
    #[inline(always)] fn tan(self) -> Self { self.tan() }
    #[inline(always)] fn sind(self) -> Self { self.to_radians().sin() }
    #[inline(always)] fn cosd(self) -> Self { self.to_radians().cos() }
    #[inline(always)] fn tand(self) -> Self { self.to_radians().tan() }

    #[inline(always)] fn discrete(a: Self, n: I) -> I
    { Num::min(Self::floori(a * n as Self), n - 1) }
}
