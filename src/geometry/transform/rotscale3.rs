use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RotScale3(A3<F3>);

impl One for RotScale3 {
    const ONE: Self = Self(Arr([Arr([F::ONE, F::ZERO, F::ZERO]),
                                Arr([F::ZERO, F::ONE, F::ZERO]),
                                Arr([F::ZERO, F::ZERO, F::ONE])]));
}
impl Default for RotScale3 { fn default() -> Self { Self::ONE } }

impl RotScale3 {
    fn from_rows(r1: F3, r2: F3, r3: F3) -> Self
    { Self(Arr([r1, r2, r3])) }

    pub fn from_cols(c1: F3, c2: F3, c3: F3) -> Self
    { Self::from_rows(c1, c2, c3).t() }

    pub fn scale(s: F3) -> Self
    { Self(Arr::from_iter((0..=2).map(F3::unit_dim)) * s) }

    pub fn rotate(axis: F3, theta: F) -> Self {
        let Arr([x, y, z]) = F3::from(V(axis).unit());
        let ct = theta.cosd();
        let cc = 1. - ct;
        let st = theta.sind();
        Self::from_rows(Arr([ct + x.sq() * cc,
                             x * y * cc - z * st,
                             x * z * cc + y * st]),
                        Arr([y * x * cc + z * st,
                             ct + y.sq() * cc,
                             y * z * cc - x * st]),
                        Arr([z * x * cc - y * st,
                             z * y * cc + x * st,
                             ct + z.sq() * cc]))
    }

    pub fn from_frame(v: V) -> Self {
        let v2 = V(if F::abs(v[X]) > F::abs(v[Y]) {
            Arr([-v[Z], 0., v[X]]) / F::sqrt(v[X].sq() + v[Z].sq())
        } else
        { Arr([0., v[Z], -v[Y]]) / F::sqrt(v[Y].sq() + v[Z].sq()) });
        Self::from_cols(F3::from(v2), F3::from(v * v2), F3::from(v))
    }

    pub fn look_at(dir: V, up: V) -> Self {
        let dir = dir.unit();
        let right = (up.unit() * dir).unit();
        let up = (dir * right).unit();
        Self::from_cols(F3::from(right), F3::from(up), F3::from(dir))
    }

    pub fn t(&self) -> Self {
        let m = self.0;
        Self::from_rows(Arr([m[0_usize][0], m[1_usize][0], m[2_usize][0]]),
                        Arr([m[0_usize][1], m[1_usize][1], m[2_usize][1]]),
                        Arr([m[0_usize][2], m[1_usize][2], m[2_usize][2]]))
    }
}

impl<A> Mul<A3<A>> for RotScale3
    where A: Copy + Zero + Add<Output = A> + Mul<F, Output = A>
{
    type Output = A3<A>;
    fn mul(self, o: A3<A>) -> A3<A> { A3::rep(o).zip(self.0, inner_product) }
}

impl Mul for RotScale3 {
    type Output = Self;
    fn mul(self, o: Self) -> Self { Self(o * self.t().0).t() }
}
