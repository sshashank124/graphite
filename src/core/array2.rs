use std::iter::{Product, Sum};
use std::ops::*;

use super::*;
use crate::{
    cw_unary_op, cw_binary_op, cw_binary_assign_op,
    scalar_binary_op, scalar_binary_assign_op
};

pub type F2 = A2<F>;
pub type I2 = A2<I>;
pub type U2 = A2<U>;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature="serde-derive", derive(Deserialize, Serialize))]
#[repr(C)]
pub struct A2<A>(pub A, pub A);

impl<A> A2<A> {
    #[inline(always)] pub const fn as_ref(&self) -> A2<&A> { A2(&self.0, &self.1) }

    #[inline(always)] pub fn map<B>(self, f: impl Fn(A) -> B) -> A2<B> { A2(f(self.0), f(self.1)) }

    #[inline(always)] pub fn zip<B, C>(self, b: A2<B>, f: impl Fn(A, B) -> C) -> A2<C>
    { A2(f(self.0, b.0), f(self.1, b.1)) }

    #[inline(always)] pub fn zips<B, C>(self, b: B, f: impl Fn(A, B) -> C) -> A2<C> where B: Copy
    { A2(f(self.0, b), f(self.1, b)) }

    #[inline(always)] pub fn zipi<B>(&mut self, b: A2<B>, f: impl Fn(&mut A, B))
    { f(&mut self.0, b.0); f(&mut self.1, b.1); }

    #[inline(always)] pub fn zipsi<B>(&mut self, b: B, f: impl Fn(&mut A, B)) where B: Copy
    { f(&mut self.0, b); f(&mut self.1, b); }

    #[inline(always)] pub fn fold<B>(self, b: B, f: impl Fn(B, A) -> B) -> B
    { f(f(b, self.0), self.1) }

    #[inline(always)] pub fn reduce<B>(self, f: impl Fn(A, A) -> B) -> B { f(self.0, self.1) }

    #[inline(always)] pub fn dot<B, C>(a: A2<A>, b: A2<B>) -> C
        where C: Zero + Add<Output = C>,
              A2<A>: Mul<A2<B>, Output = A2<C>>,
    { (a * b).sum() }
}

impl<A> A2<A> where A: Copy {
    #[inline(always)] pub const fn rep(a: A) -> A2<A> { A2(a, a) }

    #[inline(always)] pub fn flip(self) -> Self { A2(self[1], self[0]) }
}

impl<A> A2<A> where A: Add<Output = A>
{ #[inline(always)] pub fn sum(self) -> A { self.reduce(Add::add) } }

impl<A> A2<A> where A: Add<Output = A> + Mul<F, Output = A>
{ #[inline(always)] pub fn mean(self) -> A { self.sum() * 0.5 } }

impl<A> A2<A> where A: Mul<Output = A>
{ #[inline(always)] pub fn product(self) -> A { self.reduce(Mul::mul) } }

impl<A> Sum for A2<A> where Self: Zero + Add<Output=Self> {
    #[inline(always)] fn sum<It>(it: It) -> Self where It: Iterator<Item=Self>
    { it.fold(Self::ZERO, Add::add) }
}

impl<A> Product for A2<A> where Self: One + Mul<Output=Self> {
    #[inline(always)]
    fn product<It>(it: It) -> Self where It: Iterator<Item=Self>
    { it.fold(Self::ONE, Mul::mul) }
}

impl<A> A2<A> where A: Zero + One {
    #[inline(always)] pub fn basis(dim: Dim) -> Self {
        match dim {
            X => A2(A::ONE, A::ZERO),
            Y => A2(A::ZERO, A::ONE),
            Z => unreachable!(),
        }
    }
}

impl<A> Zero for A2<A> where A: Copy + Zero
{ const ZERO: Self = A2::rep(A::ZERO); }

impl<A> One for A2<A> where A: Copy + One
{ const ONE: Self = A2::rep(A::ONE); }

impl<A> Half for A2<A> where A: Copy + Half
{ const HALF: Self = A2::rep(A::HALF); }

impl<A> Two for A2<A> where A: Copy + Two
{ const TWO: Self = A2::rep(A::TWO); }

macro_rules! index {
    ($type:ident[$v1:tt, $v2:tt]) => {
        impl<A> Index<$type> for A2<A> {
            type Output = A;
            #[inline(always)]
            #[allow(clippy::match_bool)]
            fn index(&self, i: $type) -> &Self::Output {
                match i {
                    $v1 => &self.0,
                    $v2 => &self.1,
                    #[allow(unreachable_patterns)]
                    _ => unreachable!(),
                }
            }
        }

        impl<A> IndexMut<$type> for A2<A> {
            #[inline(always)]
            #[allow(clippy::match_bool)]
            fn index_mut(&mut self, i: $type) -> &mut Self::Output {
                match i {
                    $v1 => &mut self.0,
                    $v2 => &mut self.1,
                    #[allow(unreachable_patterns)]
                    _ => unreachable!(),
                }
            }
        }
    };
}

index!(i32[0, 1]);
index!(u32[0, 1]);
index!(usize[0, 1]);
index!(Dim[X, Y]);
index!(bool[false, true]);

cw_unary_op!(A2, Neg::neg);
cw_unary_op!(A2, Not::not);
cw_unary_op!(A2, Inv::inv);

cw_binary_op!(A2, Add::add);
cw_binary_op!(A2, Sub::sub);
cw_binary_op!(A2, Mul::mul);
cw_binary_op!(A2, Div::div);
cw_binary_op!(A2, BitAnd::bitand);
cw_binary_op!(A2, BitOr::bitor);
cw_binary_op!(A2, BitXor::bitxor);
cw_binary_op!(A2, Rem::rem);

cw_binary_assign_op!(A2, AddAssign::add_assign);
cw_binary_assign_op!(A2, SubAssign::sub_assign);
cw_binary_assign_op!(A2, MulAssign::mul_assign);
cw_binary_assign_op!(A2, DivAssign::div_assign);
cw_binary_assign_op!(A2, BitAndAssign::bitand_assign);
cw_binary_assign_op!(A2, BitOrAssign::bitor_assign);
cw_binary_assign_op!(A2, BitXorAssign::bitxor_assign);
cw_binary_assign_op!(A2, RemAssign::rem_assign);

scalar_binary_op!(A2, Add::add);
scalar_binary_op!(A2, Sub::sub);
scalar_binary_op!(A2, Mul::mul);
scalar_binary_op!(A2, Div::div);
scalar_binary_op!(A2, BitAnd::bitand);
scalar_binary_op!(A2, BitOr::bitor);
scalar_binary_op!(A2, BitXor::bitxor);
scalar_binary_op!(A2, Rem::rem);

scalar_binary_assign_op!(A2, AddAssign::add_assign);
scalar_binary_assign_op!(A2, SubAssign::sub_assign);
scalar_binary_assign_op!(A2, MulAssign::mul_assign);
scalar_binary_assign_op!(A2, DivAssign::div_assign);
scalar_binary_assign_op!(A2, BitAndAssign::bitand_assign);
scalar_binary_assign_op!(A2, BitOrAssign::bitor_assign);
scalar_binary_assign_op!(A2, BitXorAssign::bitxor_assign);
scalar_binary_assign_op!(A2, RemAssign::rem_assign);

impl F2 {
    #[inline(always)] pub fn min(self) -> F { self.reduce(F::min) }
    #[inline(always)] pub fn max(self) -> F { self.reduce(F::max) }
}

impl<A> Conv<A2<A>> for (A, A)
{ #[inline(always)] fn conv(self) -> A2<A> { A2(self.0, self.1) } }

impl<A> Conv<(A, A)> for A2<A>
{ #[inline(always)] fn conv(self) -> (A, A) { (self.0, self.1) } }

impl<A> Conv<A2<A>> for [A; 2] where A: Copy
{ #[inline(always)] fn conv(self) -> A2<A> { A2(self[0], self[1]) } }

impl<A> Conv<[A; 2]> for A2<A>
{ #[inline(always)] fn conv(self) -> [A; 2] { [self.0, self.1] } }

impl<A, B> Conv<A2<B>> for A2<A> where A: Conv<B>
{ #[inline(always)] fn conv(self) -> A2<B> { A2(self.0.conv(), self.1.conv()) } }


#[cfg(feature="serde-derive")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn deser_i2()
    { assert_eq!(serde_json::from_str::<I2>("[-1, 1]").unwrap(), A2(-1, 1)); }

    #[test] fn deser_f2()
    { assert_eq!(serde_json::from_str::<F2>("[-1, 1]").unwrap(), A2(-1., 1.)); }
}
