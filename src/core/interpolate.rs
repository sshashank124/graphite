use std::ops::{Add, Mul};

use super::*;

#[derive(Clone, Copy, Debug)]
pub struct LinearScale;

#[derive(Clone, Copy, Debug)]
pub struct PowerScale;

#[derive(Clone, Copy, Debug)]
pub struct SmoothScale;

pub trait Interp<A> {
    fn interp(a: A2<A>, t: F) -> A;
}

impl<A> Interp<A> for LinearScale
    where A2<A>: Mul<F2, Output = A2<A>>,
              A: Copy + Zero + Add<Output = A>
{
    #[inline]
    fn interp(a: A2<A>, t: F) -> A { inner_product(a, Arr([1. - t, t])) }
}

impl<A> Interp<A> for SmoothScale
    where A: Copy + Zero + Add<Output = A> + Mul<F, Output = A>
{
    #[inline]
    fn interp(a: A2<A>, t: F) -> A
    { LinearScale::interp(a, t.sq() * (3. - 2. * t)) }
}

pub trait Balance {
    fn balance(a: F2) -> F;
    fn balance2(a: F, b: F) -> F { Self::balance(Arr([a, b])) }
}

impl Balance for LinearScale
{ #[inline] fn balance(a: F2) -> F { a[0] / a.sum() } }

impl Balance for PowerScale
{ #[inline] fn balance(a: F2) -> F { LinearScale::balance(map(a, F::sq)) } }
