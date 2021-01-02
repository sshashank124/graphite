use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature="serde-derive", derive(Deserialize))]
#[repr(C)]
pub struct RotScale3(Option<A3<F3>>);

impl One for RotScale3 { const ONE: Self = Self(None); }
impl Default for RotScale3
{ #[inline] fn default() -> Self { Self::ONE } }

impl RotScale3 {
    #[inline] const fn from_rows(r1: F3, r2: F3, r3: F3) -> Self
    { Self(Some(A3(r1, r2, r3))) }

    #[inline] pub fn from_cols(c1: F3, c2: F3, c3: F3) -> Self
    { Self::from_rows(c1, c2, c3).t() }

    #[inline] pub fn scale<A: Into<F3>>(s: A) -> Self
    { Self(Some(XYZ.map(F3::basis) * s.into())) }

    #[inline] pub fn rotate<A: Into<F3>>(axis: A, angle: F) -> Self {
        let A3(x, y, z) = F3::from(V(axis.into()).unit());
        let ct = angle.cosd();
        let cc = 1. - ct;
        let st = angle.sind();
        Self::from_rows(A3(x.sq().mul_add(cc, ct),
                           (x * y).mul_add(cc, -z * st),
                           (x * z).mul_add(cc, y * st)),
                        A3((y * x).mul_add(cc, z * st),
                           y.sq().mul_add(cc, ct),
                           (y * z).mul_add(cc, -x * st)),
                        A3((z * x).mul_add(cc, -y * st),
                           (z * y).mul_add(cc, x * st),
                           z.sq().mul_add(cc, ct)))
    }

    #[inline] pub fn look_at(dir: V, up: V) -> Self {
        let dir = dir.unit();
        let right = (up.unit() * dir).unit();
        let up = (dir * right).unit();
        Self::from_cols(F3::from(right), F3::from(up), F3::from(dir))
    }

    #[inline] pub fn from_frame<A: Into<F3>>(v: A) -> Self {
        let v = V(v.into());
        let v2 = V(if F::abs(v[X]) > F::abs(v[Y]) {
            A3(-v[Z], 0., v[X]) / F::sqrt(v[X].sq() + v[Z].sq())
        } else { A3(0., v[Z], -v[Y]) / F::sqrt(v[Y].sq() + v[Z].sq()) });
        Self::from_cols(F3::from(v2), F3::from(v * v2), F3::from(v))
    }

    #[inline] pub fn t(&self) -> Self {
        self.0.map_or_else(|| *self, |m|
            Self::from_rows(A3(m[0_usize][0], m[1_usize][0], m[2_usize][0]),
                            A3(m[0_usize][1], m[1_usize][1], m[2_usize][1]),
                            A3(m[0_usize][2], m[1_usize][2], m[2_usize][2]))
        )
    }
}

impl<A> Mul<A3<A>> for RotScale3
    where A: Copy + Zero + Add<Output = A> + Mul<F, Output = A>
{
    type Output = A3<A>;
    #[inline] fn mul(self, o: A3<A>) -> A3<A>
    { self.0.map_or_else(|| o, |m| A3::rep(o).zip(m, A3::inner_product)) }
}

impl Mul for RotScale3 {
    type Output = Self;
    #[inline] fn mul(self, o: Self) -> Self
    { self.t().0.map_or_else(|| o, |m| Self(Some(o * m)).t()) }
}
