#[macro_export]
macro_rules! op {
    ($trait:ident::$op:ident, *$type:ident) => {
        impl $trait for $type {
            type Output = $type;
            #[inline]
            fn $op(self) -> $type { $type($trait::$op(self.0)) }
        }
    };
    ($trait:ident::$op:ident, *$lhs:ident -> *$rhs:ident -> $out:ident) => {
        impl $trait<$rhs> for $lhs {
            type Output = $out;
            #[inline]
            fn $op(self, b: $rhs) -> $out { $out($trait::$op(self.0, b.0)) }
        }
    };
    ($trait:ident::$op:ident, $lhs:ident -> *$rhs:ident -> $out:ident) => {
        impl $trait<$rhs> for $lhs {
            type Output = $out;
            #[inline]
            fn $op(self, b: $rhs) -> $out { $out($trait::$op(self, b.0)) }
        }
    };
    ($trait:ident::$op:ident, *$lhs:ident -> $rhs:ident -> $out:ident) => {
        impl $trait<$rhs> for $lhs {
            type Output = $out;
            #[inline]
            fn $op(self, b: $rhs) -> $out { $out($trait::$op(self.0, b)) }
        }
    };
    ($trait:ident::$op:ident, *mut $lhs:ident -> *$rhs:ident -> ()) => {
        impl $trait<$rhs> for $lhs {
            #[inline]
            fn $op(&mut self, b: $rhs) { $trait::$op(&mut self.0, b.0); }
        }
    };
    ($trait:ident::$op:ident, *mut $lhs:ident -> $rhs:ident -> ()) => {
        impl $trait<$rhs> for $lhs {
            #[inline]
            fn $op(&mut self, b: $rhs) { $trait::$op(&mut self.0, b); }
        }
    };
}

#[macro_export]
macro_rules! cw_unary_op {
    ($array:ident, $trait:ident::$op:ident) => {
        impl<A> $trait for $array<A> where A: $trait<Output = A>
        {
            type Output = $array<A>;
            #[inline] fn $op(self) -> Self::Output
            { $array::map(self, $trait::$op) }
        }
    };
}

#[macro_export]
macro_rules! cw_binary_op {
    ($array:ident, $trait:ident::$op:ident) => {
        impl<A, B, C> $trait<$array<B>> for $array<A>
            where A: $trait<B, Output = C>
        {
            type Output = $array<C>;
            #[inline] fn $op(self, b: $array<B>) -> Self::Output
            { $array::zip(self, b, $trait::$op) }
        }
    };
}

#[macro_export]
macro_rules! cw_binary_assign_op {
    ($array:ident, $trait:ident::$op:ident) => {
        impl<A, B> $trait<$array<B>> for $array<A>
            where A: $trait<B>
        {
            #[inline] fn $op(&mut self, b: $array<B>)
            { $array::zipi(self, b, $trait::$op) }
        }
    };
}

#[macro_export]
macro_rules! scalar_binary_op {
    ($array:ident, $trait:ident::$op:ident) => {
        impl<A, B, N> $trait<N> for $array<A>
            where A: $trait<N, Output = B>,
                  N: Num
        {
            type Output = $array<B>;
            #[inline] fn $op(self, n: N) -> Self::Output
            { $array::zips(self, n, $trait::$op) }
        }
    };
}

#[macro_export]
macro_rules! scalar_binary_assign_op {
    ($array:ident, $trait:ident::$op:ident) => {
        impl<A, N> $trait<N> for $array<A>
            where N: Num,
                  A: $trait<N>
        {
            #[inline] fn $op(&mut self, n: N)
            { $array::zipsi(self, n, $trait::$op) }
        }
    };
}
