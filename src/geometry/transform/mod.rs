mod affine3;
mod normalization2;
mod rotscale3;

use std::ops::{Add, Div, Mul, Neg};

use super::*;

pub use normalization2::Norm2 as T2;
pub use TransformPair3 as T;

type T3 = affine3::Affine3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize)]
#[serde(from="TransformPair3Config")]
pub struct TransformPair3 {
    f: T3,
    i: T3,
}

impl One for TransformPair3 { const ONE: Self = Self::new(T3::ONE, T3::ONE); }

impl TransformPair3 {
    #[inline(always)] const fn new(f: T3, i: T3) -> Self { Self { f, i } }

    #[inline(always)] pub fn translate<A>(v: A) -> Self
        where A: Copy + Neg<Output=A> + Into<F3>
    { Self::new(T3::translate(v), T3::translate(-v)) }

    #[inline(always)] pub fn scale<A>(v: A) -> Self
        where A: Copy + Inv + Into<F3>
    { Self::new(T3::scale(v), T3::scale(v.inv())) }

    #[inline(always)] pub fn rotate<A>(axis: A, angle: F) -> Self
        where A: Copy + Into<F3>
    { Self::new(T3::rotate(axis, angle), T3::rotate(axis, -angle)) }

    #[inline(always)] pub fn look_at(pos: P, target: P, up: V) -> Self
    { Self::new(T3::look_at(pos, target, up), T3::ONE) }

    #[inline(always)] pub fn from_frame<A: Into<F3>>(v: A) -> Self {
        let t = T3::from_frame(v);
        Self::new(t, t.t())
    }

    #[inline(always)] pub fn rot(&self) -> Self
    { Self::new(self.f.rot(), self.i.rot()) }

    #[inline(always)] pub fn t(&self) -> Self
    { Self::new(self.f.t(), self.i.t()) }

    #[inline(always)]
    pub fn product<It>(it: It) -> Self where It: DoubleEndedIterator<Item=Self>
    { it.rfold(Self::ONE, Mul::mul) }
}

impl Inv for TransformPair3 {
    type Output = Self;
    #[inline(always)] fn inv(self) -> Self { Self::new(self.i, self.f) }
}

impl Mul for TransformPair3 {
    type Output = Self;
    #[inline(always)] fn mul(self, s: Self) -> Self
    { Self::new(self.f * s.f, s.i * self.i) }
}

impl<A> Mul<A3<A>> for TransformPair3
    where A: Copy + Zero + Add<F, Output = A> + Add<Output = A>
           + Mul<F, Output = A>
{
    type Output = A3<A>;
    #[inline(always)] fn mul(self, t: A3<A>) -> A3<A> { self.f * t }
}

impl<A> Div<A3<A>> for TransformPair3
    where A: Copy + Zero + Add<F, Output = A> + Add<Output = A>
           + Mul<F, Output = A>
{
    type Output = A3<A>;
    #[inline(always)] fn div(self, v: A3<A>) -> A3<A> { self.i * v }
}


#[derive(Debug, Deserialize)]
#[serde(rename_all="snake_case")]
enum TransformPair3Config {
    Translate(F3),
    Scale(F3),
    Rotate {
        axis: F3,
        angle: F,
    },
    LookAt {
        pos: P,
        target: P,
        up: V,
    },
}

impl From<TransformPair3Config> for TransformPair3 {
    fn from(tc: TransformPair3Config) -> Self {
        match tc {
            TransformPair3Config::Translate(v) => Self::translate(v),
            TransformPair3Config::Scale(s) => Self::scale(s),
            TransformPair3Config::Rotate { axis, angle }
                => Self::rotate(axis, angle),
            TransformPair3Config::LookAt { pos, target, up }
                => Self::look_at(pos, target, up),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn translate() {
        let s = r#"{ "translate": [1, -2, 0.5] }"#;
        assert_eq!(serde_json::from_str::<TransformPair3>(s).unwrap(),
                   TransformPair3::translate(A3(1., -2., 0.5)));
    }

    #[test] fn scale() {
        let s = r#"{ "scale": [1, -2, 0.5] }"#;
        assert_eq!(serde_json::from_str::<TransformPair3>(s).unwrap(),
                   TransformPair3::scale(A3(1., -2., 0.5)));
    }

    #[test] fn rotate() {
        let s = r#"{ "rotate": { "axis": [1, -2, 0.5], "angle": 50 } }"#;
        assert_eq!(serde_json::from_str::<TransformPair3>(s).unwrap(),
                   TransformPair3::rotate(A3(1., -2., 0.5), 50.));
    }

    #[test] fn look_at() {
        let s = r#"{ "look_at": { "pos": [1, -2, 0.5],
                                  "target": [100, 4, -10],
                                  "up": [0, 1, 1] } }"#;
        assert_eq!(serde_json::from_str::<TransformPair3>(s).unwrap(),
                   TransformPair3::look_at(P(A3(1., -2., 0.5)),
                                           P(A3(100., 4., -10.)),
                                           V(A3(0., 1., 1.))));
    }
}
