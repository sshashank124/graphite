use std::iter::{Product, Sum};
use std::ops::*;

use super::*;
use crate::{
    cw_unary_op, cw_binary_op, cw_binary_assign_op,
    scalar_binary_op, scalar_binary_assign_op
};


pub type F3 = A3<F>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct A3<A>(pub A, pub A, pub A);

// General Arrays

impl<A> A3<A> {
    #[inline(always)] pub fn map<B>(self, f: impl Fn(A) -> B) -> A3<B>
    { A3(f(self.0), f(self.1), f(self.2)) }

    #[inline(always)]
    pub fn zip<B, C>(self, b: A3<B>, f: impl Fn(A, B) -> C) -> A3<C>
    { A3(f(self.0, b.0), f(self.1, b.1), f(self.2, b.2)) }

    #[inline(always)]
    pub fn zips<B, C>(self, b: B, f: impl Fn(A, B) -> C) -> A3<C>
        where B: Copy,
    { A3(f(self.0, b), f(self.1, b), f(self.2, b)) }

    #[inline(always)] pub fn zipi<B>(&mut self, b: A3<B>, f: impl Fn(&mut A, B))
    { f(&mut self.0, b.0); f(&mut self.1, b.1); f(&mut self.2, b.2); }

    #[inline(always)] pub fn zipsi<B>(&mut self, b: B, f: impl Fn(&mut A, B))
        where B: Copy,
    { f(&mut self.0, b); f(&mut self.1, b); f(&mut self.2, b); }

    #[inline(always)] pub fn reduce(self, f: impl Fn(A, A) -> A) -> A
    { f(f(self.0, self.1), self.2) }

    #[inline(always)] pub fn inner_product<B, C, AA, BB>(a: AA, b: BB) -> C
        where AA: Into<A3<A>>,
              BB: Into<A3<B>>,
              C: Zero + Add<Output = C>,
              A3<A>: Mul<A3<B>, Output = A3<C>>,
    { (a.into() * b.into()).sum() }

    #[inline(always)] pub fn dot<AA, BB>(a: AA, b: BB) -> F
        where AA: Into<F3>,
              BB: Into<F3>
    { A3::inner_product(a, b) }
}

impl<A> A3<A> where A: Copy {
    #[inline(always)] pub const fn rep(a: A) -> A3<A> { A3(a, a, a) }

    #[inline(always)] pub fn a2a(a2: A2<A>, a: A) -> A3<A>
    { A3(a2[0], a2[1], a) }

    #[inline(always)] pub fn swizzle(self, i1: I, i2: I, i3: I) -> Self
    { A3(self[i1], self[i2], self[i3]) }
}

impl<A> A3<A> where A: Add<Output = A>
{ #[inline(always)] pub fn sum(self) -> A { self.reduce(Add::add) } }

impl<A> A3<A>
    where A: Copy + Zero + Add<Output = A> + Div<F, Output = A>
{ #[inline(always)] pub fn mean(self) -> A { self.sum() / 3. } }

impl<A> A3<A> where A: Mul<Output = A>
{ #[inline(always)] pub fn product(self) -> A { self.reduce(Mul::mul) } }

impl<A> Default for A3<A> where A: Copy + Default
{ #[inline(always)] fn default() -> Self { Self::rep(A::default()) } }

impl<A> Sum for A3<A> where Self: Zero + Add<Output=Self> {
    #[inline(always)] fn sum<It>(it: It) -> Self where It: Iterator<Item=Self>
    { it.fold(Self::ZERO, Add::add) }
}

impl<A> Product for A3<A> where Self: One + Mul<Output=Self> {
    #[inline(always)]
    fn product<It>(it: It) -> Self where It: Iterator<Item=Self>
    { it.fold(Self::ONE, Mul::mul) }
}

impl<A> A3<A> where A: Zero + One {
    #[inline(always)]
    pub fn basis(dim: Dim) -> Self {
        match dim {
            X => A3(A::ONE, A::ZERO, A::ZERO),
            Y => A3(A::ZERO, A::ONE, A::ZERO),
            Z => A3(A::ZERO, A::ZERO, A::ONE),
        }
    }
}

impl<A> Zero for A3<A> where A: Zero { const ZERO: Self = A3::rep(A::ZERO); }
impl<A> One for A3<A> where A: One { const ONE: Self = A3::rep(A::ONE); }
impl<A> Half for A3<A> where A: Half { const HALF: Self = A3::rep(A::HALF); }
impl<A> Two for A3<A> where A: Two { const TWO: Self = A3::rep(A::TWO); }

macro_rules! index {
    ($type:ident[$v1:tt, $v2:tt, $v3:tt]) => {
        impl<A> Index<$type> for A3<A> {
            type Output = A;
            #[inline(always)]
            #[allow(clippy::match_bool)]
            fn index(&self, i: $type) -> &Self::Output {
                match i {
                    $v1 => &self.0,
                    $v2 => &self.1,
                    $v3 => &self.2,
                    #[allow(unreachable_patterns)]
                    _ => unreachable!(),
                }
            }
        }

        impl<A> IndexMut<$type> for A3<A> {
            #[inline(always)]
            #[allow(clippy::match_bool)]
            fn index_mut(&mut self, i: $type) -> &mut Self::Output {
                match i {
                    $v1 => &mut self.0,
                    $v2 => &mut self.1,
                    $v3 => &mut self.2,
                    #[allow(unreachable_patterns)]
                    _ => unreachable!(),
                }
            }
        }
    };
}

index!(I[0, 1, 2]);
index!(usize[0, 1, 2]);
index!(Dim[X, Y, Z]);

cw_unary_op!(A3, Neg::neg);
cw_unary_op!(A3, Not::not);
cw_unary_op!(A3, Inv::inv);

cw_binary_op!(A3, Add::add);
cw_binary_op!(A3, Sub::sub);
cw_binary_op!(A3, Mul::mul);
cw_binary_op!(A3, Div::div);
cw_binary_op!(A3, BitAnd::bitand);
cw_binary_op!(A3, BitOr::bitor);
cw_binary_op!(A3, BitXor::bitxor);
cw_binary_op!(A3, Rem::rem);

cw_binary_assign_op!(A3, AddAssign::add_assign);
cw_binary_assign_op!(A3, SubAssign::sub_assign);
cw_binary_assign_op!(A3, MulAssign::mul_assign);
cw_binary_assign_op!(A3, DivAssign::div_assign);
cw_binary_assign_op!(A3, BitAndAssign::bitand_assign);
cw_binary_assign_op!(A3, BitOrAssign::bitor_assign);
cw_binary_assign_op!(A3, BitXorAssign::bitxor_assign);
cw_binary_assign_op!(A3, RemAssign::rem_assign);

scalar_binary_op!(A3, Add::add);
scalar_binary_op!(A3, Sub::sub);
scalar_binary_op!(A3, Mul::mul);
scalar_binary_op!(A3, Div::div);
scalar_binary_op!(A3, BitAnd::bitand);
scalar_binary_op!(A3, BitOr::bitor);
scalar_binary_op!(A3, BitXor::bitxor);
scalar_binary_op!(A3, Rem::rem);

scalar_binary_assign_op!(A3, AddAssign::add_assign);
scalar_binary_assign_op!(A3, SubAssign::sub_assign);
scalar_binary_assign_op!(A3, MulAssign::mul_assign);
scalar_binary_assign_op!(A3, DivAssign::div_assign);
scalar_binary_assign_op!(A3, BitAndAssign::bitand_assign);
scalar_binary_assign_op!(A3, BitOrAssign::bitor_assign);
scalar_binary_assign_op!(A3, BitXorAssign::bitxor_assign);
scalar_binary_assign_op!(A3, RemAssign::rem_assign);

impl F3 {
    #[inline(always)] pub fn min(self) -> F { self.reduce(F::min) }
    #[inline(always)] pub fn max(self) -> F { self.reduce(F::max) }
}