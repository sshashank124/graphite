use std::iter::{Product, Sum};
use std::mem::MaybeUninit;
use std::ops::*;

use super::*;

pub type A4<A> = Arr<A, 4>;
pub type A3<A> = Arr<A, 3>;
pub type A2<A> = Arr<A, 2>;

pub type FF<const N: usize> = Arr<F, N>;
pub type II<const N: usize> = Arr<I, N>;

pub type F4 = FF<4>;
pub type F3 = FF<3>;
pub type F2 = FF<2>;
pub type I2 = II<2>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Arr<A, const N: usize>(pub [A; N]);

impl<A, const N: usize> Arr<A, N> {
    pub fn from_iter<It>(it: It) -> Self where It: Iterator<Item=A> {
        let mut aa: [MaybeUninit<A>; N] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        for (a, item) in aa.iter_mut().zip(it) { *a = MaybeUninit::new(item); }
        let aaptr: *const [A; N] = (&aa as *const [MaybeUninit<A>; N]).cast();
        std::mem::forget(aa);
        Self(unsafe { aaptr.read() })
    }

    pub fn zips<B, C>(self, b: B, f: impl Fn(A, B) -> C) -> Arr<C, N>
        where B: Copy
    { Arr(self.0.map(|x| f(x, b))) }

    pub fn zipi<B>(&mut self, b: Arr<B, N>, f: impl Fn(&mut A, B))
        where B: Copy
    { self.0.iter_mut().zip(b.0.iter()).for_each(|(x, &y)| f(x, y)); }

    pub fn zipsi<B>(&mut self, b: B, f: impl Fn(&mut A, B))
        where B: Copy
    { self.0.iter_mut().for_each(|x| f(x, b)); }
}

pub fn map<A, AA, B, const N: usize>(a: AA, f: impl Fn(A) -> B) -> Arr<B, N>
    where AA: Into<Arr<A, N>>
{ Arr(a.into().0.map(f)) }

impl<A, const N: usize> Arr<A, N> where A: Copy {
    pub fn rep(a: A) -> Self { Self([a; N]) }

    pub fn zip<B, C>(self, b: Arr<B, N>, f: impl Fn(A, B) -> C) -> Arr<C, N>
        where B: Copy
    { Arr::from_iter(self.0.iter().zip(b.0.iter()).map(|(&x, &y)| f(x, y))) }

    pub fn fold<B>(self, acc: B, f: impl Fn(B, A) -> B) -> B
    { self.0.iter().fold(acc, |b, &a| f(b, a)) }

    pub fn reduce(self, f: impl Fn(A, A) -> A) -> A {
        let mut it = self.0.iter();
        let first = it.next().unwrap();
        it.fold(*first, |b, &a| f(b, a))
    }

    pub fn shl(self) -> Self {
        let mut aa = self.0.clone();
        aa.rotate_left(1);
        Self(aa)
    }

    pub fn shr(self) -> Self {
        let mut aa = self.0.clone();
        aa.rotate_right(1);
        Self(aa)
    }
}

impl<A, const N: usize> Default for Arr<A, N> where A: Copy + Default
{ fn default() -> Self { Self::rep(A::default()) } }

impl<A, const N: usize> Zero for Arr<A, N> where A: Zero
{ const ZERO: Self = Self([A::ZERO; N]); }

impl<A, const N: usize> One for Arr<A, N> where A: One
{ const ONE: Self = Self([A::ONE; N]); }

macro_rules! cw_unary_op {
    ($trait:ident::$op:ident) => {
        impl<A, const N: usize> $trait for Arr<A, N>
            where A: $trait<Output = A>
        {
            type Output = Arr<A, N>;
            fn $op(self) -> Self::Output { map(self, $trait::$op) }
        }
    };
}
cw_unary_op!(Neg::neg);
cw_unary_op!(Not::not);
cw_unary_op!(Inv::inv);

macro_rules! cw_binary_op {
    ($trait:ident::$op:ident) => {
        impl<A, B, C, const N: usize> $trait<Arr<B, N>> for Arr<A, N>
            where A: Copy + $trait<B, Output = C>,
                  B: Copy
        {
            type Output = Arr<C, N>;
            fn $op(self, b: Arr<B, N>) -> Self::Output
            { self.zip(b, $trait::$op) }
        }
    };
}
cw_binary_op!(Add::add);
cw_binary_op!(Sub::sub);
cw_binary_op!(Mul::mul);
cw_binary_op!(Div::div);
cw_binary_op!(BitAnd::bitand);
cw_binary_op!(BitOr::bitor);
cw_binary_op!(BitXor::bitxor);
cw_binary_op!(Rem::rem);

macro_rules! cw_binary_assign_op {
    ($trait:ident::$op:ident) => {
        impl<A, B, const N: usize> $trait<Arr<B, N>> for Arr<A, N>
            where A: $trait<B>,
                  B: Copy
        { fn $op(&mut self, b: Arr<B, N>) { self.zipi(b, $trait::$op) } }
    };
}
cw_binary_assign_op!(AddAssign::add_assign);
cw_binary_assign_op!(SubAssign::sub_assign);
cw_binary_assign_op!(MulAssign::mul_assign);
cw_binary_assign_op!(DivAssign::div_assign);
cw_binary_assign_op!(BitAndAssign::bitand_assign);
cw_binary_assign_op!(BitOrAssign::bitor_assign);
cw_binary_assign_op!(BitXorAssign::bitxor_assign);
cw_binary_assign_op!(RemAssign::rem_assign);

macro_rules! scalar_binary_op {
    ($trait:ident::$op:ident) => {
        impl<A, B, S, const N: usize> $trait<S> for Arr<A, N>
            where A: $trait<S, Output = B>,
                  S: Num
        {
            type Output = Arr<B, N>;
            fn $op(self, s: S) -> Self::Output { self.zips(s, $trait::$op) }
        }
    };
}
scalar_binary_op!(Add::add);
scalar_binary_op!(Sub::sub);
scalar_binary_op!(Mul::mul);
scalar_binary_op!(Div::div);
scalar_binary_op!(BitAnd::bitand);
scalar_binary_op!(BitOr::bitor);
scalar_binary_op!(BitXor::bitxor);
scalar_binary_op!(Rem::rem);

macro_rules! scalar_binary_assign_op {
    ($trait:ident::$op:ident) => {
        impl<A, S, const N: usize> $trait<S> for Arr<A, N>
            where A: $trait<S>,
                  S: Num
        { fn $op(&mut self, s: S) { self.zipsi(s, $trait::$op) } }
    };
}
scalar_binary_assign_op!(AddAssign::add_assign);
scalar_binary_assign_op!(SubAssign::sub_assign);
scalar_binary_assign_op!(MulAssign::mul_assign);
scalar_binary_assign_op!(DivAssign::div_assign);
scalar_binary_assign_op!(BitAndAssign::bitand_assign);
scalar_binary_assign_op!(BitOrAssign::bitor_assign);
scalar_binary_assign_op!(BitXorAssign::bitxor_assign);
scalar_binary_assign_op!(RemAssign::rem_assign);

impl<A, const N: usize> Index<I> for Arr<A, N> {
    type Output = A;
    fn index(&self, i: I) -> &A { &self.0[i as usize] }
}
impl<A, const N: usize> IndexMut<I> for Arr<A, N>
{ fn index_mut(&mut self, i: I) -> &mut A { &mut self.0[i as usize] } }

impl<A, const N: usize> Index<usize> for Arr<A, N> {
    type Output = A;
    fn index(&self, i: usize) -> &A { &self.0[i] }
}
impl<A, const N: usize> IndexMut<usize> for Arr<A, N>
{ fn index_mut(&mut self, i: usize) -> &mut A { &mut self.0[i] } }

impl<A, const N: usize> Sum for Arr<A, N> where Self: Zero + Add<Output=Self> {
    fn sum<It>(it: It) -> Self where It: Iterator<Item=Self>
    { it.fold(Self::ZERO, Add::add) }
}

impl<A, const N: usize> Product for Arr<A, N> where Self: One + Mul<Output=Self>
{
    fn product<It>(it: It) -> Self where It: Iterator<Item=Self>
    { it.fold(Self::ONE, Mul::mul) }
}

impl<A, const N: usize> Arr<A, N> where A: Copy + Zero + One {
    pub fn unit_dim(dim: usize) -> Self {
        let mut aa = Self::ZERO;
        aa[dim] = A::ONE;
        aa
    }
}

impl<A, const N: usize> Arr<A, N> where A: Copy + Add<Output = A>
{ pub fn sum(self) -> A { self.reduce(Add::add) } }

impl<A, const N: usize> Arr<A, N>
    where A: Copy + Zero + Add<Output = A> + Div<F, Output = A>
{
    pub fn mean(self) -> A { self.sum() / (N as F) }
}

impl<A, const N: usize> Arr<A, N> where A: Copy + Mul<Output = A>
{ pub fn product(self) -> A { self.reduce(Mul::mul) } }

pub fn inner_product<A, B, C, AA, BB, const N: usize>(a: AA, b: BB) -> C
    where AA: Into<Arr<A, N>>,
          BB: Into<Arr<B, N>>,
          C: Copy + Zero + Add<Output = C>,
          Arr<A, N>: Mul<Arr<B, N>, Output = Arr<C, N>>,
{ (a.into() * b.into()).sum() }

impl<A, const N: usize> From<[A; N]> for Arr<A, N>
{ fn from(arr: [A; N]) -> Self { Self(arr) } }

// Specifics

impl<A> A3<A> where A: Copy
{ pub fn a2a(a2: A2<A>, a: A) -> Self { Self([a2[0], a2[1], a]) } }

impl<const N: usize> FF<N> {
    pub fn min(self) -> F { self.reduce(F::min) }
    pub fn max(self) -> F { self.reduce(F::max) }
}

pub fn dot<AA, BB, const N: usize>(a: AA, b: BB) -> F
    where AA: Into<FF<N>>,
          BB: Into<FF<N>>,
{ inner_product(a, b) }

macro_rules! index {
    ($n:tt, $type:ident[$($vals:tt),*]) => {
        impl<A> Index<$type> for Arr<A, $n> {
            type Output = A;
            fn index(&self, i: $type) -> &Self::Output {
                index!(i, 0, &self.0, $($vals),*)
            }
        }
    };
    ($expr:expr, $idx:expr, $self:expr, $val:tt) => {
        if let $val = $expr { &$self[$idx] } else { unreachable!() }
    };
    ($expr:expr, $idx:expr, $self:expr, $val:tt, $($vals:tt),*) => {
        if let $val = $expr { &$self[$idx] }
        else { index!($expr, $idx+1, $self, $($vals),*) }
    };
}
macro_rules! index_mut {
    ($n:tt, $type:ident[$($vals:tt),*]) => {
        impl<A> IndexMut<$type> for Arr<A, $n> {
            fn index_mut(&mut self, i: $type) -> &mut Self::Output {
                index_mut!(i, 0, &mut self.0, $($vals),*)
            }
        }
    };
    ($expr:expr, $idx:expr, $self:expr, $val:tt) => {
        if let $val = $expr { &mut $self[$idx] } else { unreachable!() }
    };
    ($expr:expr, $idx:expr, $self:expr, $val:tt, $($vals:tt),*) => {
        if let $val = $expr { &mut $self[$idx] }
        else { index_mut!($expr, $idx+1, $self, $($vals),*) }
    };
}

index!(3, Dim[X, Y, Z]); index_mut!(3, Dim[X, Y, Z]);
index!(2, Dim[X, Y]); index_mut!(2, Dim[X, Y]);
index!(2, bool[false, true]); index_mut!(2, bool[false, true]);

impl<const N: usize> From<II<N>> for FF<N>
{ fn from(ii: II<N>) -> Self { map(ii, |i: I| i as F) } }

impl<const N: usize> From<FF<N>> for II<N>
{ fn from(ff: FF<N>) -> Self { map(ff, |f: F| f as I) } }

impl<const N: usize> From<Arr<usize, N>> for II<N>
{ fn from(uu: Arr<usize, N>) -> Self { map(uu, |u: usize| u as I) } }

impl<const N: usize> From<II<N>> for Arr<usize, N>
{ fn from(ii: II<N>) -> Self { map(ii, |i: I| i as usize) } }
