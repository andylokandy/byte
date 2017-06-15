#![feature(test)]

extern crate test;
extern crate scroll;

use test::black_box;
use scroll::*;

#[bench]
fn bench_pread(b: &mut test::Bencher) {
    const NITER: i32 = 100_000;
    b.iter(|| for _ in 1..NITER {
               let data = black_box([1, 2]);
               let _: u16 = black_box(data.pread_with(0, LE).unwrap());
           });
    b.bytes = 2 * NITER as u64;
}

#[bench]
fn bench_try_from_ctx(b: &mut test::Bencher) {
    const NITER: i32 = 100_000;
    b.iter(|| for _ in 1..NITER {
               let data: &[u8] = &black_box([1, 2]);
               let _: u16 = black_box(u16::try_from_ctx(&data, LE).unwrap().0);
           });
    b.bytes = 2 * NITER as u64;
}