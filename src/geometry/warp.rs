use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniformTriangle;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniformDisk;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CosineHemisphere;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniformCylinder;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniformSphere;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniformHemisphere;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BeckmannHemisphere;

impl UniformTriangle {
    pub fn warp(s: F2) -> F2 {
        let t = s * 0.5;
        let o = t[1] - t[0];
        Arr(if F::is_pos(o) { [t[0], t[1] + o] }
            else { [t[0] - o, t[1]] })
    }

    pub fn pdf() -> F { 2. }
}

impl UniformDisk {
    pub fn warp(s: F2) -> F2 {
        let u = s * 2. - 1.;
        if u == F2::ZERO { F2::ZERO } else {
            let (r, t) = if F::abs(u[X]) > F::abs(u[Y]) {
                (u[X], F::FOURTH_PI * u[Y] / u[X])
            } else { (u[Y], F::HALF_PI - F::FOURTH_PI * u[X] / u[Y]) };
            Arr([F::cos(t), F::sin(t)]) * r
        }
    }

    pub fn pdf() -> F { F::INV_PI }
}

impl CosineHemisphere {
    pub fn warp(s: F2) -> F3 {
        let p = UniformDisk::warp(s);
        F3::a2a(p, F::sqrt(1. - dot(p, p)))
    }

    pub fn pdf<A: Into<F3>>(s: A) -> F { Frame::ct(s) * F::INV_PI }
}

impl UniformCylinder {
    pub fn warp(s: F2) -> F3 {
        let t = F::TWO_PI * s[Y];
        Arr([F::cos(t), F::sin(t), 2. * s[X] - 1.])
    }

    pub fn pdf() -> F { F::INV_4PI }
}

impl UniformSphere {
    pub fn warp(s: F2) -> F3 {
        let v = UniformCylinder::warp(s);
        let r = Frame::st(v);
        Arr([r * v[X], r * v[Y], v[Z]])
    }

    pub fn pdf() -> F { F::INV_4PI }
}

impl UniformHemisphere {
    pub fn warp(s: F2) -> F3 {
        let v = UniformSphere::warp(s);
        Arr([v[X], v[Y], v[Z].abs()])
    }

    pub fn pdf() -> F { F::INV_2PI }
}

impl BeckmannHemisphere {
    pub fn warp(s: F2, alpha: F) -> F3 {
        let c2t = (1. - alpha.sq() * F::ln(F::ONE - s[0])).inv();
        let phi = F::TWO_PI * s[1];
        let r = F::sqrt(1. - c2t);
        Arr([r * F::cos(phi), r * F::sin(phi), F::sqrt(c2t)])
    }

    pub fn pdf<A: Into<F3>>(s: A, alpha: F) -> F {
        let a2_inv = alpha.sq().inv();
        let ct = Frame::ct(s);
        (F::INV_PI * a2_inv * F::exp(-a2_inv * (ct.sq().inv() - 1.)))
        / (ct * ct.sq())
    }
}
