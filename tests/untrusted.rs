use byteorder::{WriteBytesExt, BigEndian, LittleEndian};
use quickcheck::quickcheck;
use untrusted::{Input, Reader};

fn reader_be(buf: &[u8]) -> Reader {
    Reader::new(Input::from_be(buf))
}

fn reader_le(buf: &[u8]) -> Reader {
    Reader::new(Input::from_le(buf))
}

quickcheck! {
    fn prop_read_byte(xs: u8) -> bool {
        let mut buf = Vec::new();
        buf.write_u8(xs).expect("write_u8");
        let mut reader = reader_be(&buf);
        xs == reader.read_byte().expect("read")
    }
}

quickcheck! {
    fn prop_read_u8(xs: u8) -> bool {
        let mut buf = Vec::new();
        buf.write_u8(xs).expect("write_u8");
        let mut reader = reader_be(&buf);
        xs == reader.read().expect("read")
    }
}

macro_rules! prop_read {
    ($name:ident, $type:ty, $endian:ty, $write:ident, $reader:ident) => {
        quickcheck! {
            fn $name(xs: $type) -> bool {
                let mut buf = Vec::new();
                buf.$write::<$endian>(xs).expect("$write");
                let mut reader = $reader(&buf);
                xs == reader.read().expect("read")
            }
        }
    }
}

prop_read!(prop_read_u16be, u16, BigEndian, write_u16, reader_be);
prop_read!(prop_read_u32be, u32, BigEndian, write_u32, reader_be);
prop_read!(prop_read_u64be, u64, BigEndian, write_u64, reader_be);

prop_read!(prop_read_u16le, u16, LittleEndian, write_u16, reader_le);
prop_read!(prop_read_u32le, u32, LittleEndian, write_u32, reader_le);
prop_read!(prop_read_u64le, u64, LittleEndian, write_u64, reader_le);

prop_read!(prop_read_i16be, i16, BigEndian, write_i16, reader_be);
prop_read!(prop_read_i32be, i32, BigEndian, write_i32, reader_be);
prop_read!(prop_read_i64be, i64, BigEndian, write_i64, reader_be);

prop_read!(prop_read_i16le, i16, LittleEndian, write_i16, reader_le);
prop_read!(prop_read_i32le, i32, LittleEndian, write_i32, reader_le);
prop_read!(prop_read_i64le, i64, LittleEndian, write_i64, reader_le);
