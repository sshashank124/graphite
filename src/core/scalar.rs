use super::*;

impl Zero for I { const ZERO: I = 0; }
impl One for I { const ONE: I = 1; }
impl Two for I { const TWO: I = 2; }
impl Num for I { }

impl Zero for F { const ZERO: F = 0.; }
impl One for F { const ONE: F = 1.; }
impl Two for F { const TWO: F = 2.; }
impl Half for F { const HALF: F = 0.5; }
impl OneThird for f32 { const ONE_THIRD: f32 = 0.333_333_34; }
impl OneThird for f64 { const ONE_THIRD: f64 = 0.333_333_333_333_333_3; }
impl Num for F { }

impl Inv for F
{ type Output = F; #[inline(always)] fn inv(self) -> F { self.recip() } }
impl Epsilon for f32 { const EPS: f32 = 1e-4; }
impl Epsilon for f64 { const EPS: f64 = 1e-6; }

impl Float for F {
    const NEG_INF: F = fmod::NEG_INFINITY;
    const POS_INF: F = fmod::INFINITY;

    const PI: F = fmod::consts::PI;
    const HALF_PI: F = fmod::consts::FRAC_PI_2;
    const FOURTH_PI: F = fmod::consts::FRAC_PI_4;
    const TWO_PI: F = F::TWO * F::PI;
    const FOUR_PI: F = F::TWO * F::TWO_PI;
    const INV_PI: F = fmod::consts::FRAC_1_PI;
    const INV_2PI: F = F::HALF * F::INV_PI;
    const INV_4PI: F = F::HALF * F::INV_2PI;

    const FRAC_1_2POW32: F = 2.328_306_4e-10;

    #[inline(always)] fn ceili(self) -> I { self.ceil().conv() }
    #[inline(always)] fn floori(self) -> I { self.floor().conv() }

    #[inline(always)] fn sin(self) -> F { self.sin() }
    #[inline(always)] fn cos(self) -> F { self.cos() }
    #[inline(always)] fn tan(self) -> F { self.tan() }
    #[inline(always)] fn sind(self) -> F { self.to_radians().sin() }
    #[inline(always)] fn cosd(self) -> F { self.to_radians().cos() }
    #[inline(always)] fn tand(self) -> F { self.to_radians().tan() }

    #[inline(always)] fn discrete(a: F, n: I) -> I { I::min(F::floori(a * F::of(n)), n - 1) }
}

macro_rules! conv_primitive {
    ($a:ident => $b:ident) => {
        impl Conv<$b> for $a { #[inline] fn conv(self) -> $b { self as $b } }
    };
    ($a:ident => $b:ident, $($bb:ident),+) => {
        conv_primitive!{$a => $b}
        conv_primitive!{$a => $($bb),+}
    };
    ($a:ident, $($aa:ident),+ => $($bb:ident),+) => {
        conv_primitive!{$a => $($bb),+}
        conv_primitive!{$($aa),+ => $($bb),+}
    };
}

conv_primitive!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64
             => u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);

conv_primitive!(bool, char => u8, u16, u32, u64, usize,
                              i8, i16, i32, i64, isize);
conv_primitive!(u8 => char);
