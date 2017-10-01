#![feature(test)]

extern crate byteorder;
extern crate rand;
extern crate test;
extern crate untrusted;

use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use rand::Rng;
use test::Bencher;
use untrusted::{Input, Reader, EndOfInput};

const COUNT: usize = 1_000_000;

#[bench]
fn bench_read_byte(b: &mut Bencher) {
    let mut rng = rand::OsRng::new().expect("OsRng");
    let bytes: Vec<u8> = rng.gen_iter().take(COUNT).collect();

    b.iter(|| {
        test::black_box({
            let input = Input::from(&bytes);
            input.read_all(
                EndOfInput,
                |reader: &mut Reader| {
                    for _ in 0..COUNT {
                        let _: u8 = reader.read_byte().unwrap();
                    }
                    return Ok(())
                }
            ).expect("input.read_all");
        });
    });
}

#[bench]
fn bench_read_u8(b: &mut Bencher) {
    let mut rng = rand::OsRng::new().expect("OsRng");
    let bytes: Vec<u8> = rng.gen_iter().take(COUNT).collect();

    b.iter(|| {
        test::black_box({
            let input = Input::from(&bytes);
            input.read_all(
                EndOfInput,
                |reader: &mut Reader| {
                    for _ in 0..COUNT {
                        let _: u8 = reader.read().unwrap();
                    }
                    return Ok(())
                }
            ).expect("input.read_all");
        });
    });
}

macro_rules! bench_read {
    ($name:ident, $type:ty, $endian:ty, $write:ident, $from:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut rng = rand::OsRng::new().expect("OsRng");
            let mut bytes = Vec::new();
            for v in rng.gen_iter().take(COUNT) {
                bytes.$write::<$endian>(v).expect("$write");
            }

            b.iter(|| {
                test::black_box({
                    let input = Input::$from(&bytes);
                    input.read_all(
                        EndOfInput,
                        |reader: &mut Reader| {
                            for _ in 0..COUNT {
                                let _: $type = reader.read().unwrap();
                            }
                            return Ok(())
                        }
                    ).expect("input.read_all");
                });
            });
        }
    }
}

bench_read!(bench_read_u16_be, u16, BigEndian, write_u16, from_be);
bench_read!(bench_read_u32_be, u32, BigEndian, write_u32, from_be);
bench_read!(bench_read_u64_be, u64, BigEndian, write_u64, from_be);

bench_read!(bench_read_u16_le, u16, LittleEndian, write_u16, from_le);
bench_read!(bench_read_u32_le, u32, LittleEndian, write_u32, from_le);
bench_read!(bench_read_u64_le, u64, LittleEndian, write_u64, from_le);

bench_read!(bench_read_i16_be, i16, BigEndian, write_i16, from_be);
bench_read!(bench_read_i32_be, i32, BigEndian, write_i32, from_be);
bench_read!(bench_read_i64_be, i64, BigEndian, write_i64, from_be);

bench_read!(bench_read_i16_le, i16, LittleEndian, write_i16, from_le);
bench_read!(bench_read_i32_le, i32, LittleEndian, write_i32, from_le);
bench_read!(bench_read_i64_le, i64, LittleEndian, write_i64, from_le);
