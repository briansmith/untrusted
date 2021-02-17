// Copyright 2015-2019 Brian Smith.
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

use untrusted::Input;

#[test]
fn test_input_from() {
    let _ = untrusted::Input::from(b"foo");
}

#[test]
fn test_input_is_empty() {
    let input = untrusted::Input::from(b"");
    assert!(input.is_empty());
    let input = untrusted::Input::from(b"foo");
    assert!(!input.is_empty());
}

#[test]
fn test_input_len() {
    let input = untrusted::Input::from(b"foo");
    assert_eq!(input.len(), 3);
}

#[test]
fn test_input_read_all() {
    let input = untrusted::Input::from(b"foo");
    let result = input.read_all(untrusted::EndOfInput, |input| {
        assert_eq!(b'f', input.read_byte()?);
        assert_eq!(b'o', input.read_byte()?);
        assert_eq!(b'o', input.read_byte()?);
        assert!(input.at_end());
        Ok(())
    });
    assert_eq!(result, Ok(()));
}

#[test]
fn test_input_read_all_unconsume() {
    let input = untrusted::Input::from(b"foo");
    let result = input.read_all(untrusted::EndOfInput, |input| {
        assert_eq!(b'f', input.read_byte()?);
        assert!(!input.at_end());
        Ok(())
    });
    assert_eq!(result, Err(untrusted::EndOfInput));
}

#[test]
fn test_input_as_slice_less_safe() {
    let slice = b"foo";
    let input = untrusted::Input::from(slice);
    assert_eq!(input.as_slice_less_safe(), slice);
}

#[test]
fn using_reader_after_skip_and_get_error_returns_error_must_not_panic() {
    let input = untrusted::Input::from(&[]);
    let r = input.read_all(untrusted::EndOfInput, |input| {
        let r = input.read_bytes(1);
        assert_eq!(r, Err(untrusted::EndOfInput));
        Ok(input.read_bytes_to_end())
    });
    let _ = r; // "Use" r. The value of `r` is undefined here.
}

#[test]
fn size_assumptions() {
    // Assume that a pointer can address any point in the address space, and
    // infer that this implies that a byte slice will never be
    // `core::usize::MAX` bytes long.
    assert_eq!(
        core::mem::size_of::<*const u8>(),
        core::mem::size_of::<usize>()
    );
}

#[test]
fn const_fn() {
    const _INPUT: untrusted::Input<'static> = untrusted::Input::from(&[]);
}

#[test]
fn test_vec_into() {
    extern crate std;
    let vec = vec![0u8; 0];
    let _x: untrusted::Input = (&vec[..]).into();
}

#[test]
fn test_from_slice() {
    let slice: &[u8] = &[0u8];
    let _x: untrusted::Input = slice.into();
}

#[test]
fn test_debug() {
    let vec: Vec<_> = (0..0x100).map(|b| b as u8).collect();

    let expected = "b\"\
        \\0\\x01\\x02\\x03\\x04\\x05\\x06\\x07\
        \\x08\\t\\n\\x0b\\x0c\\r\\x0e\\x0f\
        \\x10\\x11\\x12\\x13\\x14\\x15\\x16\\x17\
        \\x18\\x19\\x1a\\x1b\\x1c\\x1d\\x1e\\x1f\
        \x20!\\\"#$%&'()*+,-./0123456789:;<=>?\
        @ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\\\]^_\
        `abcdefghijklmnopqrstuvwxyz{|}~\\x7f\
        \\x80\\x81\\x82\\x83\\x84\\x85\\x86\\x87\
        \\x88\\x89\\x8a\\x8b\\x8c\\x8d\\x8e\\x8f\
        \\x90\\x91\\x92\\x93\\x94\\x95\\x96\\x97\
        \\x98\\x99\\x9a\\x9b\\x9c\\x9d\\x9e\\x9f\
        \\xa0\\xa1\\xa2\\xa3\\xa4\\xa5\\xa6\\xa7\
        \\xa8\\xa9\\xaa\\xab\\xac\\xad\\xae\\xaf\
        \\xb0\\xb1\\xb2\\xb3\\xb4\\xb5\\xb6\\xb7\
        \\xb8\\xb9\\xba\\xbb\\xbc\\xbd\\xbe\\xbf\
        \\xc0\\xc1\\xc2\\xc3\\xc4\\xc5\\xc6\\xc7\
        \\xc8\\xc9\\xca\\xcb\\xcc\\xcd\\xce\\xcf\
        \\xd0\\xd1\\xd2\\xd3\\xd4\\xd5\\xd6\\xd7\
        \\xd8\\xd9\\xda\\xdb\\xdc\\xdd\\xde\\xdf\
        \\xe0\\xe1\\xe2\\xe3\\xe4\\xe5\\xe6\\xe7\
        \\xe8\\xe9\\xea\\xeb\\xec\\xed\\xee\\xef\
        \\xf0\\xf1\\xf2\\xf3\\xf4\\xf5\\xf6\\xf7\
        \\xf8\\xf9\\xfa\\xfb\\xfc\\xfd\\xfe\\xff\"";

    assert_eq!(expected, format!("{:?}", Input::from(&vec)));
}
