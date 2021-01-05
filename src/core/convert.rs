pub trait Convert<A> { fn conv(self) -> A; }
pub trait ConvertFrom<A> { fn conv_from(a: A) -> Self; }

impl<A, B> ConvertFrom<A> for B where A: Convert<B>
{ #[inline] fn conv_from(a: A) -> Self { a.conv() } }
