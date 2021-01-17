use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature="serde-derive", derive(Deserialize, Serialize))]
pub struct Frame;

impl Frame {
    #[inline] pub fn ct<A: Conv<F3>>(v: A) -> F { v.conv()[Z] }
    #[inline] pub fn st<A: Conv<F3>>(v: A) -> F
    { F::sqrt(Self::s2t(v)) }
    #[inline] pub fn tt<A: Copy + Conv<F3>>(v: A) -> F
    { Self::st(v) / Self::ct(v) }

    #[inline] pub fn c2t<A: Conv<F3>>(v: A) -> F { Self::ct(v).sq() }
    #[inline] pub fn s2t<A: Conv<F3>>(v: A) -> F
    { F::clamp_pos(1. - Self::c2t(v)) }
    #[inline] pub fn t2t<A: Copy + Conv<F3>>(v: A) -> F
    { Self::s2t(v) / Self::c2t(v) }

    #[inline] pub fn reflect<A: Conv<F3>>(v: A) -> F3 {
        let v = v.conv();
        A3(-v[X], -v[Y], v[Z])
    }

    #[inline] pub fn same_hemisphere<A, B>(v1: A, v2: B) -> bool
        where A: Conv<F3>, B: Conv<F3>
    { F3::dot(v1.conv(), v2.conv()) >= 0. }

    // Frame transforms
    #[inline] pub fn cart2spher<A: Conv<F3>>(v: A) -> F2 {
        let v = v.conv();
        let y = F::atan2(v[Y], v[X]);
        let y = if y < 0. { y + F::TWO_PI } else { y };
        A2(F::acos(v[Z]), y)
    }

    #[inline] pub fn spher2cart(v: F2) -> F3 {
        let st = F::sin(v[0]);
        A3(st * F::cos(v[1]), st * F::sin(v[1]), F::cos(v[0]))
    }
}
