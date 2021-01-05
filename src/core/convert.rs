pub trait Conv<A> { fn conv(self) -> A; }
pub trait ConvFrom<A> { fn conv_from(a: A) -> Self; }

impl<A, B> ConvFrom<A> for B where A: Conv<B>
{ #[inline] fn conv_from(a: A) -> Self { a.conv() } }
