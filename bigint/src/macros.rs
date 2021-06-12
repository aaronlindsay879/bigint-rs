/// Implements a trait for various combinations of references, assuming already implemented for &T and &T
#[macro_export]
macro_rules! trait_impl {
    ($type:ty, $trait:ident, $func:ident) => {
        impl $trait<&$type> for $type {
            type Output = <&'static $type as $trait<&'static $type>>::Output;

            fn add(self, rhs: &BigInt) -> Self::Output {
                $trait::$func(&self, rhs)
            }
        }

        impl $trait<$type> for &$type {
            type Output = <&'static $type as $trait<&'static $type>>::Output;

            fn add(self, rhs: BigInt) -> Self::Output {
                $trait::$func(self, &rhs)
            }
        }

        impl $trait<$type> for $type {
            type Output = <&'static $type as $trait<&'static $type>>::Output;

            fn add(self, rhs: BigInt) -> Self::Output {
                $trait::$func(&self, &rhs)
            }
        }
    };
}
