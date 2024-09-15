//! Specialized iterator traits.
//!
//! The traits are for iterables containing bytes, and provide optimizations
//! which then can be used for contiguous or non-contiguous iterables,
//! including containers or iterators of any kind.

#![cfg(feature = "parse")]

// Re-export our digit iterators.
#[cfg(not(feature = "format"))]
pub use crate::noskip::{AsBytes, Bytes};
#[cfg(feature = "format")]
pub use crate::skip::{AsBytes, Bytes};

/// A trait for working with iterables of bytes.
///
/// These iterators can either be contiguous or not contiguous and provide
/// methods for reading data and accessing underlying data. The readers
/// can either be contiguous or non-contiguous, although performance and
/// some API methods may not be available for both.
///
/// # Safety
///
/// // TODO: FIX CORRECTNESS DOCUMENTATION
/// This trait is effectively safe but the implementor must guarantee that
/// `is_empty` is implemented correctly. For most implementations, this can
/// be `self.as_slice().is_empty()`, where `as_slice` is implemented as
/// `&self.bytes[self.index..]`.
#[cfg(feature = "parse")]
pub unsafe trait Iter<'a> {
    /// Determine if the buffer is contiguous in memory.
    const IS_CONTIGUOUS: bool;

    // CURSORS
    // -------

    /// Get a ptr to the current start of the buffer.
    fn as_ptr(&self) -> *const u8;

    /// Get a slice to the current start of the buffer.
    fn as_slice(&self) -> &'a [u8];

    /// Get a slice to the full underlying contiguous buffer,
    fn get_buffer(&self) -> &'a [u8];

    // TODO: Rename to `buffer_length`.
    /// Get the total number of elements in the underlying buffer.
    #[inline(always)]
    fn length(&self) -> usize {
        self.get_buffer().len()
    }

    /// Get the current index of the iterator in the slice.
    fn cursor(&self) -> usize;

    /// Set the current index of the iterator in the slice.
    ///
    /// This is **NOT** the current position of the iterator,
    /// since iterators may skip digits: this is the cursor
    /// in the underlying buffer. For example, if `slc[2]` is
    /// skipped, `set_cursor(3)` would be the 3rd element in
    /// the iterator, not the 4th.
    ///
    /// # Safety
    ///
    /// Safe if `index <= self.length()`. Although this won't
    /// affect safety, the caller also should be careful it
    /// does not set the cursor within skipped characters
    /// since this could affect correctness: an iterator that
    /// only accepts non-consecutive digit separators would
    /// pass if the cursor was set between the two.
    unsafe fn set_cursor(&mut self, index: usize);

    /// Get the current number of values returned by the iterator.
    fn current_count(&self) -> usize;

    // TODO: DOCUMENT
    // PROPERTIES

    // TODO: Fix this naming convention
    /// Get if no bytes are available in the buffer.
    ///
    /// This operators on the underlying buffer: that is,
    /// it returns if [as_slice] would return an empty slice.
    ///
    /// [as_slice]: Iter::as_slice
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    /// Determine if the buffer is contiguous.
    #[inline(always)]
    fn is_contiguous(&self) -> bool {
        Self::IS_CONTIGUOUS
    }

    // TODO: Ensure we get this **RIGHT**

    /// Get the next value available without consuming it.
    ///
    /// This does **NOT** skip digits, and directly fetches the item
    /// from the underlying buffer.
    ///
    /// # Safety
    ///
    /// An implementor must implement `is_empty` correctly in
    /// order to guarantee the traitt is safe: `is_empty` **MUST**
    /// ensure that one value remains, if the iterator is non-
    /// contiguous this means advancing the iterator to the next
    /// position.
    fn first(&self) -> Option<&'a u8>;

    /// Check if the next element is a given value.
    #[inline(always)]
    fn first_is_cased(&self, value: u8) -> bool {
        Some(&value) == self.first()
    }

    /// Check if the next element is a given value without case sensitivity.
    #[inline(always)]
    fn first_is_uncased(&self, value: u8) -> bool {
        if let Some(&c) = self.first() {
            c.to_ascii_lowercase() == value.to_ascii_lowercase()
        } else {
            false
        }
    }

    // TODO(ahuszagh) Change `first_is` to have cased or uncased

    // STEP BY
    // -------

    /// Advance the internal slice by `N` elements.
    ///
    /// This does not advance the iterator by `N` elements for
    /// non-contiguous iterators: this just advances the internal,
    /// underlying buffer. This is useful for multi-digit optimizations
    /// for contiguous iterators.
    ///
    /// # Panics
    ///
    /// This will panic if the buffer advances for non-contiguous
    /// iterators if the current byte is a digit separator, or if the
    /// count is more than 1.
    ///
    /// # Safety
    ///
    /// As long as the iterator is at least `N` elements, this
    /// is safe.
    unsafe fn step_by_unchecked(&mut self, count: usize);

    /// Advance the internal slice by 1 element.
    ///
    /// # Panics
    ///
    /// This will panic if the buffer advances for non-contiguous
    /// iterators if the current byte is a digit separator.
    ///
    /// # Safety
    ///
    /// Safe as long as the iterator is not empty.
    #[inline(always)]
    unsafe fn step_unchecked(&mut self) {
        debug_assert!(!self.as_slice().is_empty());
        // SAFETY: safe if `self.index < self.length()`.
        unsafe { self.step_by_unchecked(1) };
    }

    // READ
    // ----

    /// Read a value of a difference type from the iterator.
    ///
    /// This advances the internal state of the iterator. This
    /// can only be implemented for contiguous iterators: non-
    /// contiguous iterators **MUST** panic.
    ///
    /// # Safety
    ///
    /// Safe as long as the number of the buffer is contains as least as
    /// many bytes as the size of V. This must be unimplemented for
    /// non-contiguous iterators.
    #[inline(always)]
    unsafe fn read_unchecked<V>(&self) -> V {
        unimplemented!();
    }

    /// Try to read a the next four bytes as a u32.
    ///
    /// This advances the internal state of the iterator.
    fn read_u32(&self) -> Option<u32>;

    /// Try to read the next eight bytes as a u64.
    ///
    /// This advances the internal state of the iterator.
    fn read_u64(&self) -> Option<u64>;
}

/// Iterator over a contiguous block of bytes.
///
/// This allows us to convert to-and-from-slices, raw pointers, and
/// peek/query the data from either end cheaply.
///
/// A default implementation is provided for slice iterators.
/// This trait **should never** return `null` from `as_ptr`, or be
/// implemented for non-contiguous data.
///
/// # Safety
///
/// TODO: Fix the safety documentation here...
/// The safe methods are sound as long as the caller ensures that
/// the methods for `read_32`, `read_64`, etc. check the bounds
/// of the underlying contiguous buffer and is only called on
/// contiguous buffers.
pub trait DigitsIter<'a>: Iterator<Item = &'a u8> + Iter<'a> {

    // TODO: Fix the documentation
    /// Get if the iterator cannot return any more elements.
    ///
    /// This may advance the internal iterator state, but not
    /// modify the next returned value.
    ///
    /// If this is an iterator, this is based on the number of items
    /// left to be returned. We do not necessarly know the length of
    /// the buffer. If this is a non-contiguous iterator, this **MUST**
    /// advance the state until it knows a value can be returned.
    ///
    /// Any incorrect implementations of this affect all safety invariants
    /// for the rest of the trait. For contiguous iterators, this can be
    /// as simple as checking if `self.cursor >= self.slc.len()`, but for
    /// non-contiguous iterators you **MUST** advance to the next element
    /// to be returned, then check to see if a value exists. The safest
    /// implementation is always to check if `self.peek().is_none()` and
    /// ensure [peek] is always safe.
    ///
    /// If you would like to see if the cursor is at the end of the buffer,
    /// see [is_done] or [is_empty] instead.
    ///
    /// [is_done]: DigitsIter::is_done
    /// [is_empty]: Iter::is_empty
    /// [peek]: DigitsIter::peek
    #[inline(always)]
    #[allow(clippy::wrong_self_convention)]
    fn is_consumed(&mut self) -> bool {
        self.peek().is_none()
    }

    /// Get if the buffer underlying the iterator is empty.
    ///
    /// This might not be the same thing as [is_consumed]: [is_consumed]
    /// checks if any more elements may be returned, which may require
    /// peeking the next value. Consumed merely checks if the
    /// iterator has an empty slice. It is effectively a cheaper,
    /// but weaker variant of [is_consumed].
    ///
    /// [is_consumed]: DigitsIter::is_consumed
    fn is_done(&self) -> bool;

    /// Peek the next value of the iterator, without consuming it.
    ///
    /// Note that this can modify the internal state, by skipping digits
    /// for iterators that find the first non-zero value, etc.
    fn peek(&mut self) -> Option<Self::Item>;

    /// Peek the next value of the iterator, and step only if it exists.
    #[inline(always)]
    fn try_read(&mut self) -> Option<Self::Item> {
        // TODO: Fix this
        if let Some(value) = self.peek() {
            // SAFETY: the slice cannot be empty because we peeked a value.
            unsafe { self.step_unchecked() };
            Some(value)
        } else {
            None
        }
    }

    /// Check if the next element is a given value.
    // TODO: Change this to peek_is_cased
    #[inline(always)]
    fn peek_is_cased(&mut self, value: u8) -> bool {
        Some(&value) == self.peek()
    }

    /// Check if the next element is a given value without case sensitivity.
    #[inline(always)]
    fn peek_is_uncased(&mut self, value: u8) -> bool {
        if let Some(&c) = self.peek() {
            c.to_ascii_lowercase() == value.to_ascii_lowercase()
        } else {
            false
        }
    }

    /// Check if the next element is a given value with optional case sensitivity.
    #[inline(always)]
    fn peek_is(&mut self, value: u8, is_cased: bool) -> bool {
        if let Some(&c) = self.peek() {
            if is_cased {
                c == value
            } else {
                c.to_ascii_lowercase() == value.to_ascii_lowercase()
            }
        } else {
            false
        }
    }

    // TODO: Add `peek_is` to have cased or uncased

    /// Peek the next value and consume it if the read value matches the
    /// expected one.
    #[inline(always)]
    fn read_if<Pred: FnOnce(&u8) -> bool>(&mut self, pred: Pred) -> Option<Self::Item> {
        // NOTE: This was implemented to remove usage of unsafe throughout to code
        // base, however, performance was really not up to scratch. I'm not sure
        // the cause of this.
        if let Some(peeked) = self.peek() {
            if pred(peeked) {
                // SAFETY: the slice cannot be empty because we peeked a value.
                unsafe { self.step_unchecked() };
                Some(peeked)
            } else {
                None
            }
        } else {
            None
        }
    }

    // TODO: Add read_is_value_cased
    // TODO: Add read_is_value_uncased
    // TODO: Add read_is_value

    /// Skip zeros from the start of the iterator
    #[inline(always)]
    fn skip_zeros(&mut self) -> usize {
        let start = self.cursor();
        // TODO: Change to `read_if` for performance.
        while let Some(&b'0') = self.peek() {
            // TODO: Can probably remove peek? This would be a lot slower like this
            self.next();
        }
        self.cursor() - start
    }

    /// Determine if the character is a digit.
    fn is_digit(&self, value: u8) -> bool;
}
