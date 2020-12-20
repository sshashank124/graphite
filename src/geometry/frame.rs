use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Frame;

impl Frame {
    pub fn ct(v: F3) -> F { v[Z] }
    pub fn st(v: F3) -> F { F::sqrt(Self::s2t(v)) }
    pub fn tt(v: F3) -> F { Self::st(v) / Self::ct(v) }

    pub fn c2t(v: F3) -> F { Self::ct(v).sq() }
    pub fn s2t(v: F3) -> F { F::clamp_pos(1. - Self::c2t(v)) }
    pub fn t2t(v: F3) -> F { Self::s2t(v) / Self::c2t(v) }

    pub fn reflect(v: V) -> V { V([-v[X], -v[Y], v[Z]].into()) }

    // Frame transforms

    pub fn cart2spher(v: F3) -> F2 {
        let y = F::atan2(v[Y], v[X]);
        let y = if y < 0. { y + F::TWO_PI } else { y };
        [F::acos(v[Z]), y].into()
    }

    pub fn spher2cart(v: F2) -> F3 {
        let st = F::sin(v[0]);
        [st * F::cos(v[1]), st * F::sin(v[1]), F::cos(v[0])].into()
    }
}
