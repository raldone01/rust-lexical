//! Radix-generic, lexical integer-to-string conversion routines.
//!
//! These routines are optimized for code size: they are significantly
//! slower at the cost of smaller binary size.

#![cfg(feature = "compact")]
#![doc(hidden)]

use core::mem;

use lexical_util::algorithm::copy_to_dst;
use lexical_util::buffer::slice_assume_init_mut;
use lexical_util::constants::FormattedSize;
use lexical_util::digit::digit_to_char;
use lexical_util::num::{AsCast, UnsignedInteger};

/// Write integer to string.
pub trait Compact: UnsignedInteger + FormattedSize {
    /// Write our integer to string without optimizations.
    ///
    /// This iterates over the buffer in reverse, and ensures that at least
    /// 128 elements exist in the temporary buffer. This ensures that the digits
    /// can never overflow, even in base 2. The buffer then must be able to hold
    /// at least 128 digits as well, after subslicing the data. This guarantee
    /// is likely already made.
    fn compact(self, radix: u32, buffer: &mut [u8]) -> usize {
        // NOTE: We do not have to validate the buffer length because `copy_to_dst` is safe.
        assert!(Self::BITS <= 128);
        let mut digits: [mem::MaybeUninit<u8>; 128] = [mem::MaybeUninit::uninit(); 128];
        let mut index = digits.len();

        // SAFETY: safe as long as buffer is large enough to hold the max value.
        // We never read unwritten values, and we never assume the data is initialized.
        // Need at least 128-bits, at least as many as the bits in the current type.
        // Since we make our safety variants inside, this is always safe.
        let inititalized = unsafe {
            // Decode all but the last digit.
            let radix = Self::from_u32(radix);
            let mut value = self;
            while value >= radix {
                let r = value % radix;
                value /= radix;
                index -= 1;
                digits.get_unchecked_mut(index).write(digit_to_char(u32::as_cast(r)));
            }

            // Decode last digit.
            let r = value % radix;
            index -= 1;
            digits.get_unchecked_mut(index).write(digit_to_char(u32::as_cast(r)));
            let uninit = digits.get_unchecked_mut(index..);
            // SAFETY: All these values have been initialized, since we know
            // we've only written from `index..len()`
            slice_assume_init_mut(uninit)
        };
        copy_to_dst(buffer, inititalized)
    }
}

macro_rules! compact_impl {
    ($($t:ty)*) => ($(
        impl Compact for $t {}
    )*)
}

compact_impl! { u8 u16 u32 u64 u128 usize }
