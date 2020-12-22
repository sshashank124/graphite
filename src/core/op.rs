#[macro_export]
macro_rules! op {
    ($trait:ident::$op:ident, *$type:ident) => {
        impl $trait for $type {
            type Output = $type;
            #[inline(always)]
            fn $op(self) -> $type { $type($trait::$op(self.0)) }
        }
    };
    ($trait:ident::$op:ident, *$lhs:ident -> *$rhs:ident -> $out:ident) => {
        impl $trait<$rhs> for $lhs {
            type Output = $out;
            #[inline(always)]
            fn $op(self, b: $rhs) -> $out { $out($trait::$op(self.0, b.0)) }
        }
    };
    ($trait:ident::$op:ident, $lhs:ident -> *$rhs:ident -> $out:ident) => {
        impl $trait<$rhs> for $lhs {
            type Output = $out;
            #[inline(always)]
            fn $op(self, b: $rhs) -> $out { $out($trait::$op(self, b.0)) }
        }
    };
    ($trait:ident::$op:ident, *$lhs:ident -> $rhs:ident -> $out:ident) => {
        impl $trait<$rhs> for $lhs {
            type Output = $out;
            #[inline(always)]
            fn $op(self, b: $rhs) -> $out { $out($trait::$op(self.0, b)) }
        }
    };
    ($trait:ident::$op:ident, *mut $lhs:ident -> *$rhs:ident -> ()) => {
        impl $trait<$rhs> for $lhs {
            #[inline(always)]
            fn $op(&mut self, b: $rhs) { $trait::$op(&mut self.0, b.0); }
        }
    };
    ($trait:ident::$op:ident, *mut $lhs:ident -> $rhs:ident -> ()) => {
        impl $trait<$rhs> for $lhs {
            #[inline(always)]
            fn $op(&mut self, b: $rhs) { $trait::$op(&mut self.0, b); }
        }
    };
}
