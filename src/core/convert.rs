#[macro_export]
macro_rules! conv {
    ($expr:expr) => { $expr };
    ($expr:expr => $t:ty $(=> $tt:ty)*) => { conv!(<$t>::of($expr) $(=> $tt)*) };
}

pub trait Conv<A> { fn conv(self) -> A; }
pub trait ConvFrom<A> { fn of(a: A) -> Self; }

impl<A, B> ConvFrom<A> for B where A: Conv<B>
{ #[inline(always)] fn of(a: A) -> Self { a.conv() } }
