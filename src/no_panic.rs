use core::fmt;

/// A wrapper around a slice that exposes no functions that can panic.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Slice<'a> {
    bytes: &'a [u8],
}

impl<'a> Slice<'a> {
    #[inline]
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    #[inline]
    pub fn get(&self, i: usize) -> Option<&u8> {
        self.bytes.get(i)
    }

    #[inline]
    pub fn subslice(&self, r: core::ops::Range<usize>) -> Option<Self> {
        self.bytes.get(r).map(|bytes| Self { bytes })
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    #[inline]
    pub fn as_slice_less_safe(&self) -> &'a [u8] {
        self.bytes
    }
}

/// Alternative implementation of `std::fmt::Debug` for byte slice.
///
/// Standard `Debug` implementation for `[u8]` is comma separated
/// list of numbers. Since large amount of byte strings are in fact
/// ASCII strings or contain a lot of ASCII strings (e. g. HTTP),
/// it is convenient to print strings as ASCII when possible.
impl fmt::Debug for Slice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "b\"")?;
        for &b in self.bytes {
            // https://doc.rust-lang.org/reference/tokens.html#byte-escapes
            if b == b'\n' {
                write!(f, "\\n")?;
            } else if b == b'\r' {
                write!(f, "\\r")?;
            } else if b == b'\t' {
                write!(f, "\\t")?;
            } else if b == b'\\' || b == b'"' {
                write!(f, "\\{}", b as char)?;
            } else if b == b'\0' {
                write!(f, "\\0")?;
                // ASCII printable
            } else if (0x20..0x7f).contains(&b) {
                write!(f, "{}", b as char)?;
            } else {
                write!(f, "\\x{:02x}", b)?;
            }
        }
        write!(f, "\"")?;
        Ok(())
    }
}
