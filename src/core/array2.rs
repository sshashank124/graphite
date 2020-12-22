use std::iter::{Product, Sum};
use std::ops::*;

use super::*;
use crate::{
    cw_unary_op, cw_binary_op, cw_binary_assign_op,
    scalar_binary_op, scalar_binary_assign_op
};


pub type F2 = A2<F>;
pub type I2 = A2<I>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct A2<A>(pub A, pub A);

// General Arrays

impl<A> A2<A> {
    #[inline(always)] pub fn map<B>(self, f: impl Fn(A) -> B) -> A2<B>
    { A2(f(self.0), f(self.1)) }

    #[inline(always)]
    pub fn zip<B, C>(self, b: A2<B>, f: impl Fn(A, B) -> C) -> A2<C>
    { A2(f(self.0, b.0), f(self.1, b.1)) }

    #[inline(always)]
    pub fn zips<B, C>(self, b: B, f: impl Fn(A, B) -> C) -> A2<C>
        where B: Copy,
    { A2(f(self.0, b), f(self.1, b)) }

    #[inline(always)] pub fn zipi<B>(&mut self, b: A2<B>, f: impl Fn(&mut A, B))
    { f(&mut self.0, b.0); f(&mut self.1, b.1); }

    #[inline(always)] pub fn zipsi<B>(&mut self, b: B, f: impl Fn(&mut A, B))
        where B: Copy,
    { f(&mut self.0, b); f(&mut self.1, b); }

    #[inline(always)]
    pub fn reduce<B>(self, f: impl Fn(A, A) -> B) -> B { f(self.0, self.1) }

    #[inline(always)] pub fn inner_product<B, C, AA, BB>(a: AA, b: BB) -> C
        where AA: Into<A2<A>>,
              BB: Into<A2<B>>,
              C: Zero + Add<Output = C>,
              A2<A>: Mul<A2<B>, Output = A2<C>>,
    { (a.into() * b.into()).sum() }

    #[inline(always)] pub fn dot<AA, BB>(a: AA, b: BB) -> F
        where AA: Into<F2>,
              BB: Into<F2>
    { A2::inner_product(a, b) }
}

impl<A> A2<A> where A: Copy {
    #[inline(always)] pub const fn rep(a: A) -> A2<A> { A2(a, a) }

    #[inline(always)] pub fn flip(self) -> Self { A2(self[1], self[0]) }
}

impl<A> A2<A> where A: Add<Output = A>
{ #[inline(always)] pub fn sum(self) -> A { self.reduce(Add::add) } }

impl<A> A2<A>
    where A: Copy + Zero + Add<Output = A> + Div<F, Output = A>
{ #[inline(always)] pub fn mean(self) -> A { self.sum() / 2. } }

impl<A> A2<A> where A: Mul<Output = A>
{ #[inline(always)] pub fn product(self) -> A { self.reduce(Mul::mul) } }

impl<A> Default for A2<A> where A: Copy + Default
{ #[inline(always)] fn default() -> Self { Self::rep(A::default()) } }

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
    #[inline(always)]
    pub fn basis(dim: Dim) -> Self {
        match dim {
            X => A2(A::ONE, A::ZERO),
            Y => A2(A::ZERO, A::ONE),
            _ => unreachable!(),
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

index!(I[0, 1]);
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

impl<A> From<(A, A)> for A2<A>
{ #[inline(always)] fn from(aa: (A, A)) -> A2<A> { A2(aa.0, aa.1) } }

impl<A> From<A2<A>> for (A, A)
{ #[inline(always)] fn from(aa: A2<A>) -> (A, A) { (aa.0, aa.1) } }

impl From<I2> for F2
{ fn from(ii: I2) -> Self { A2::map(ii, |i| i as F) } }

impl From<F2> for I2
{ fn from(ff: F2) -> Self { A2::map(ff, |f| f as I) } }

impl From<A2<usize>> for I2
{ fn from(uu: A2<usize>) -> Self { A2::map(uu, |u| u as I) } }

impl From<I2> for A2<usize>
{ fn from(ii: I2) -> Self { A2::map(ii, |i| i as usize) } }
