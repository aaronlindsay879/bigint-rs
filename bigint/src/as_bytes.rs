use std::convert::TryInto;

pub trait AsBytes: Sized {
    /// Gets the native byte representation of the type.
    fn as_bytes(&self) -> Vec<u8>;

    /// Converts the native byte representation to the specified type.
    fn from_bytes(bytes: &[u8]) -> Option<Self>;

    /// Whether the keep extra carry values by default. This is false for unsigned types, and true for signed types.
    ///
    /// If returns true, then extra carry values from operations like addition are simply appended to the end.
    /// If false, the final carry value is discarded.
    fn keep_carry() -> bool;
}

macro_rules! single_impl {
    ($type:ty, $carry:expr) => {
        impl AsBytes for $type {
            fn as_bytes(&self) -> Vec<u8> {
                self.to_ne_bytes().to_vec()
            }

            fn from_bytes(bytes: &[u8]) -> Option<Self> {
                let bytes = match bytes.try_into() {
                    Ok(bytes) => bytes,
                    Err(_) => return None,
                };
                Some(<$type>::from_ne_bytes(bytes))
            }

            fn keep_carry() -> bool {
                $carry
            }
        }
    };
}

macro_rules! impl_asbytes {
    (keep_carry: $($keep_carry:ty )+; discard_carry: $($discard_carry:ty )+) => {
        $(
            single_impl!($keep_carry, false);
        )+

        $(
            single_impl!($discard_carry, true);
        )+
    };
}

impl_asbytes!(
    keep_carry: u8 u16 u32 u64 u128 usize;
    discard_carry: i8 i16 i32 i64 i128 isize f32 f64
);
