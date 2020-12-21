use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Frame;

impl Frame {
    pub fn ct<A: Into<F3>>(v: A) -> F { v.into()[Z] }
    pub fn st<A: Into<F3>>(v: A) -> F { F::sqrt(Self::s2t(v)) }
    pub fn tt<A: Copy + Into<F3>>(v: A) -> F { Self::st(v) / Self::ct(v) }

    pub fn c2t<A: Into<F3>>(v: A) -> F { Self::ct(v).sq() }
    pub fn s2t<A: Into<F3>>(v: A) -> F { F::clamp_pos(1. - Self::c2t(v)) }
    pub fn t2t<A: Copy + Into<F3>>(v: A) -> F { Self::s2t(v) / Self::c2t(v) }

    pub fn reflect<A: Into<F3>>(v: A) -> F3 {
        let v = v.into();
        Arr([-v[X], -v[Y], v[Z]])
    }

    // Frame transforms

    pub fn cart2spher<A: Into<F3>>(v: A) -> F2 {
        let v = v.into();
        let y = F::atan2(v[Y], v[X]);
        let y = if y < 0. { y + F::TWO_PI } else { y };
        Arr([F::acos(v[Z]), y])
    }

    pub fn spher2cart(v: F2) -> F3 {
        let st = F::sin(v[0]);
        Arr([st * F::cos(v[1]), st * F::sin(v[1]), F::cos(v[0])])
    }
}
