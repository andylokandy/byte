#![feature(test)]

extern crate test;
extern crate byteorder;
extern crate scroll;

use test::black_box;
use byteorder::*;
use scroll::*;

#[bench]
fn bench_byteorder(b: &mut test::Bencher) {
    const N: u64 = 10_000;
    b.iter(|| for _ in 1..N {
               black_box(LittleEndian::read_u16(&[1, 2]));
           });
    b.bytes = 2 * N;
}

#[bench]
fn bench_read(b: &mut test::Bencher) {
    const N: u64 = 10_000;
    b.iter(|| for _ in 1..N {
               black_box([1, 2].read_with::<u16>(0, LE).unwrap());
           });
    b.bytes = 2 * N;
}