// Copyright 2015-2021 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use crate::Input;

/// A read-only, forward-only cursor into the data in an `Input`.
///
/// Using `Reader` to parse input helps to ensure that no byte of the input
/// will be accidentally processed more than once. Using `Reader` in
/// conjunction with `read_all` and `read_all_optional` helps ensure that no
/// byte of the input is accidentally left unprocessed. The methods of `Reader`
/// never panic, so `Reader` also assists the writing of panic-free code.
#[derive(Debug)]
pub struct Reader<'a>(Input<'a>);

impl<'a> Reader<'a> {
    /// Construct a new Reader for the given input. Use `read_all` or
    /// `read_all_optional` instead of `Reader::new` whenever possible.
    #[inline]
    pub fn new(input: Input<'a>) -> Self {
        Self(input)
    }

    /// Returns `true` if the reader is at the end of the input, and `false`
    /// otherwise.
    #[inline]
    pub fn at_end(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if there is at least one more byte in the input and that
    /// byte is equal to `b`, and false otherwise.
    #[inline]
    pub fn peek(&self, b: u8) -> bool {
        self.0.first().copied() == Some(b)
    }

    /// Reads the next input byte.
    ///
    /// Returns `Ok(b)` where `b` is the next input byte, or `Err(EndOfInput)`
    /// if the `Reader` is at the end of the input.
    #[inline]
    pub fn read_byte(&mut self) -> Result<u8, EndOfInput> {
        let (h, t) = self.0.split_first().ok_or(EndOfInput)?;
        self.0 = t;
        Ok(h)
    }

    /// Skips `num_bytes` of the input, returning the skipped input as an
    /// `Input`.
    ///
    /// Returns `Ok(i)` if there are at least `num_bytes` of input remaining,
    /// and `Err(EndOfInput)` otherwise.
    #[inline]
    pub fn read_bytes(&mut self, num_bytes: usize) -> Result<Input<'a>, EndOfInput> {
        let (before, after) = self.0.split_at(num_bytes).ok_or(EndOfInput)?;
        self.0 = after;
        Ok(before)
    }

    /// Skips the reader to the end of the input, returning the skipped input
    /// as an `Input`.
    #[inline]
    pub fn read_bytes_to_end(&mut self) -> Input<'a> {
        core::mem::replace(&mut self.0, Input::empty())
    }

    /// Calls `read()` with the given input as a `Reader`. On success, returns a
    /// pair `(bytes_read, r)` where `bytes_read` is what `read()` consumed and
    /// `r` is `read()`'s return value.
    pub fn read_partial<F, R, E>(&mut self, read: F) -> Result<(Input<'a>, R), E>
    where
        F: FnOnce(&mut Reader<'a>) -> Result<R, E>,
    {
        let original = self.0;
        let r = read(self)?;
        let amount_read = original.len().checked_sub(self.0.len()).unwrap();
        let (bytes_read, _) = original.split_at(amount_read).unwrap();
        Ok((bytes_read, r))
    }

    /// Skips `num_bytes` of the input.
    ///
    /// Returns `Ok(i)` if there are at least `num_bytes` of input remaining,
    /// and `Err(EndOfInput)` otherwise.
    #[inline]
    pub fn skip(&mut self, num_bytes: usize) -> Result<(), EndOfInput> {
        self.read_bytes(num_bytes).map(|_| ())
    }

    /// Skips the reader to the end of the input.
    #[inline]
    pub fn skip_to_end(&mut self) {
        let _ = self.read_bytes_to_end();
    }
}

/// The error type used to indicate the end of the input was reached before the
/// operation could be completed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EndOfInput;
