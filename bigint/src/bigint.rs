use crate::as_bytes::AsBytes;
use crate::trait_impl;
use itertools::{EitherOrBoth, Itertools};
use std::{cmp::max, ops::Add};

/// A big integer type, supporting arbitrarily sized integers.
///
/// All backing data is stored on the heap, allowing this integer to grow if needed.
#[derive(Debug, Default)]
pub struct BigInt {
    /// Backing data for storing the big integer.
    backing: Vec<u8>,
    /// Whether to discard the final carry value in an operation like addition.
    discard_carry: bool,
}

impl BigInt {
    /// Constructs a big integer with a backing vector with a given capacity.
    ///
    /// # Example
    /// ```
    /// # use bigint::bigint::*;
    /// let bigint = BigInt::with_capacity(100);
    /// assert!(bigint.backing().capacity() >= 100);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            backing: Vec::with_capacity(capacity),
            discard_carry: false,
        }
    }

    /// Constructs a big integer from an existing backing vector.
    ///
    /// # Example
    /// ```
    /// # use bigint::bigint::*;
    /// let bigint = BigInt::from_backing(vec![1, 2, 3, 4]);
    /// assert_eq!(bigint.to_value(), Some(67305985));
    /// assert_eq!(bigint.backing(), &vec![1, 2, 3, 4]);
    /// ```
    pub fn from_backing(backing: Vec<u8>) -> Self {
        Self {
            backing,
            discard_carry: false,
        }
    }

    /// Constructs a big integer from a given value, given that it can be safely converted to bytes.
    ///
    /// This will give different results on big and little endian systems.
    ///
    /// # Example
    /// ```
    /// # use bigint::bigint::*;
    /// let bigint = BigInt::from_value(701u16);
    /// assert_eq!(bigint.to_value(), Some(701u16));
    /// assert_eq!(bigint.backing(), &vec![189, 2]);
    /// ```
    pub fn from_value<T: AsBytes>(value: T) -> Self {
        Self {
            backing: value.as_bytes(),
            discard_carry: T::keep_carry(),
        }
    }

    /// Constructs a value from the bytes stored, returning None if byte lengths don't match.
    ///
    /// This will give different results on big and little endian systems.
    ///
    /// # Example
    /// ```
    /// # use bigint::bigint::*;
    /// let bigint = BigInt::from_backing(vec![189, 2]);
    /// assert_eq!(bigint.to_value(), Some(701u16));
    /// ```
    pub fn to_value<T: AsBytes>(&self) -> Option<T> {
        T::from_bytes(&self.backing)
    }

    /// Gets a reference to the backing data of the big integer.
    ///
    /// # Example
    /// ```
    /// # use bigint::bigint::*;
    /// let bigint = BigInt::from_backing(vec![1, 2, 3, 4]);
    /// assert_eq!(bigint.backing(), &vec![1, 2, 3, 4]);
    /// ```
    pub fn backing(&self) -> &Vec<u8> {
        &self.backing
    }
}

trait_impl!(BigInt, Add, add);
impl Add<&BigInt> for &BigInt {
    type Output = BigInt;

    /// Adds two big integers together.
    ///
    /// # Example
    /// ```
    /// # use bigint::bigint::*;
    /// assert_eq!((BigInt::from_value(3u8) + BigInt::from_value(16u8)).to_value(), Some(19u8));
    /// assert_eq!((BigInt::from_value(3u8) + BigInt::from_value(255u8)).to_value(), Some(258u16));
    /// assert_eq!((BigInt::from_value(255u8) + BigInt::from_value(255u8)).to_value(), Some(510u16));
    /// assert_eq!((BigInt::from_value(u64::MAX) + BigInt::from_value(u128::MAX / 2)).to_value(), Some(u64::MAX as u128 + u128::MAX / 2));
    ///
    /// let one = BigInt::from_backing(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255]);
    /// let two = BigInt::from_backing(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 17, 1]);
    /// assert_eq!((one + two).backing(), &vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 18, 1]);
    /// ```
    fn add(self, rhs: &BigInt) -> Self::Output {
        let max_len = max(self.backing.len(), rhs.backing.len());

        let mut out = Vec::with_capacity(max_len + 1);
        let mut carry = 0;

        // iterate through all bytes of both ints
        for item in self.backing.iter().zip_longest(rhs.backing.iter()) {
            // get the byte and number of overflows that occur from this position
            let (byte, overflow) = match item {
                EitherOrBoth::Both(left, right) => {
                    // if both integers have a byte at this position, add them both together and store the overflow results
                    let (output, overflow_a) = left.overflowing_add(*right);
                    let (output, overflow_b) = output.overflowing_add(carry);

                    // then add the overflows together as ints, so if two overflows occured it returns 2
                    // this is needed rather than something like (overflow_a || overflow_b) as that would return 1 if both were true, not 2
                    (output, overflow_a as u8 + overflow_b as u8)
                }
                EitherOrBoth::Left(single) | EitherOrBoth::Right(single) => {
                    // if only one integer has a byte at this position, simply add to overflow and return
                    let (output, overflow) = single.overflowing_add(carry);
                    (output, overflow as u8)
                }
            };

            // then push to output and reset the carry
            out.push(byte);
            carry = overflow;
        }

        // if the carry exists and neither of the bigints need to discard the carry, push the carry to the output too
        if !(carry == 0 || self.discard_carry || rhs.discard_carry) {
            out.push(carry);
        }

        // then finally construct a bigint from the backing vector
        BigInt::from_backing(out)
    }
}
