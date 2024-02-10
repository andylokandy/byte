#![feature(test)]

extern crate byte;
extern crate byteorder;
extern crate test;

use byte::ctx::*;
use byte::*;
use byteorder::*;
use test::black_box;

#[bench]
fn bench_byteorder(b: &mut test::Bencher) {
    b.iter(|| black_box(byteorder::LittleEndian::read_u16(&black_box([1, 2]))));
    b.bytes = 2;
}

#[bench]
fn bench_read_num(b: &mut test::Bencher) {
    b.iter(|| black_box(black_box([1, 2]).read_with::<u16>(&mut 0, LE).unwrap()));
    b.bytes = 2;
}

#[bench]
fn bench_str(b: &mut test::Bencher) {
    let bytes = b"abcdefghijkl";
    b.iter(|| black_box(black_box(bytes).read_with::<&str>(&mut 0, Len(5)).unwrap()));
    b.bytes = 5;
}

#[bench]
fn bench_str_hardcode(b: &mut test::Bencher) {
    let bytes = b"abcdefghijkl";
    b.iter(|| black_box(std::str::from_utf8(&black_box(bytes)[0..5]).unwrap()));
    b.bytes = 5;
}

#[bench]
fn bench_example_read(b: &mut test::Bencher) {
    let bytes = black_box([0, 5, b"H"[0], b"E"[0], b"L"[0], b"L"[0], b"O"[0], 0]);
    b.iter(|| black_box(bytes.read_with::<Header>(&mut 0, BE).unwrap()));
    b.bytes = 8;
}

#[bench]
fn bench_example_write(b: &mut test::Bencher) {
    let mut bytes = [0u8; 8];
    b.iter(|| {
        let header = black_box(Header {
            name: "HELLO",
            enabled: false,
        });
        bytes.write_with::<Header>(&mut 0, header, BE).unwrap()
    });
    b.bytes = 8;
}

#[bench]
fn bench_example_read_hardcode(b: &mut test::Bencher) {
    let bytes = black_box([0, 5, b"H"[0], b"E"[0], b"L"[0], b"L"[0], b"O"[0], 0]);
    b.iter(|| black_box(example_read_hardcode(&bytes[..]).unwrap()));
    b.bytes = 8;
}

#[bench]
fn bench_example_write_hardcode(b: &mut test::Bencher) {
    let mut bytes = [0u8; 8];
    b.iter(|| {
        let header = black_box(Header {
            name: "HELLO",
            enabled: false,
        });
        example_write_hardcode(&mut bytes[..], header).unwrap()
    });
    b.bytes = 8;
}

fn example_read_hardcode<'a>(bytes: &'a [u8]) -> Option<Header<'a>> {
    if bytes.len() < 3 {
        return None;
    }

    let name_len = unsafe { *(&bytes[0] as *const _ as *const u16) };
    let name_len = name_len.to_be() as usize;

    if bytes.len() < name_len + 3 {
        return None;
    }

    let name = std::str::from_utf8(&bytes[2..name_len + 2]).unwrap();
    let enabled = bytes[name_len + 2] != 0;

    Some(Header { name, enabled })
}

fn example_write_hardcode(bytes: &mut [u8], header: Header) -> Option<()> {
    if bytes.len() < header.name.len() + 3 {
        return None;
    }

    unsafe { *(&mut bytes[0] as *mut _ as *mut u16) = header.name.len().to_be() as u16 };

    bytes[2..header.name.len() + 2].clone_from_slice(header.name.as_bytes());
    bytes[header.name.len()] = if header.enabled { u8::max_value() } else { 0 };

    Some(())
}

#[bench]
fn bench_example_read_write_numbers(b: &mut test::Bencher) {
    let numbers = Numbers {
        one: 1,
        two: 2,
        three: 3,
        four: 4,
        five: 5,
        six: 6,
        seven: 7,
        eight: 8,
    };
    let buf = &mut [0u8; 30];
    b.iter(|| {
        black_box(numbers.try_write(buf, LE).unwrap());
        black_box(Numbers::try_read(buf, LE).unwrap())
    });
}

struct Header<'a> {
    name: &'a str,
    enabled: bool,
}

impl<'a, Ctx: Endianess> TryRead<'a, Ctx> for Header<'a> {
    fn try_read(bytes: &'a [u8], endian: Ctx) -> Result<(Self, usize)> {
        let offset = &mut 0;

        let name_len = black_box(bytes.read_with::<u16>(offset, endian)? as usize);
        let name = bytes.read_with::<&str>(offset, Len(name_len))?;
        let enabled = bytes.read(offset)?;

        Ok((Header { name, enabled }, *offset))
    }
}

impl<'a, Ctx: Endianess> TryWrite<Ctx> for Header<'a> {
    fn try_write(self, bytes: &mut [u8], endian: Ctx) -> Result<usize> {
        let offset = &mut 0;

        bytes.write_with(offset, self.name.len() as u16, endian)?;
        bytes.write(offset, self.name)?;
        bytes.write(offset, self.enabled)?;

        Ok(*offset)
    }
}

struct Numbers {
    one: u8,
    two: u16,
    three: u32,
    four: u64,
    five: i8,
    six: i16,
    seven: i32,
    eight: i64,
}

impl<'a, Ctx: Endianess> TryRead<'a, Ctx> for Numbers {
    fn try_read(bytes: &'a [u8], endian: Ctx) -> Result<(Self, usize)> {
        let offset = &mut 0;

        let one = bytes.read_with(offset, endian)?;
        let two = bytes.read_with(offset, endian)?;
        let three = bytes.read_with(offset, endian)?;
        let four = bytes.read_with(offset, endian)?;
        let five = bytes.read_with(offset, endian)?;
        let six = bytes.read_with(offset, endian)?;
        let seven = bytes.read_with(offset, endian)?;
        let eight = bytes.read_with(offset, endian)?;

        Ok((
            Numbers {
                one,
                two,
                three,
                four,
                five,
                six,
                seven,
                eight,
            },
            *offset,
        ))
    }
}

impl<'a, Ctx: Endianess> TryWrite<Ctx> for &Numbers {
    fn try_write(self, bytes: &mut [u8], endian: Ctx) -> Result<usize> {
        let offset = &mut 0;

        bytes.write_with(offset, self.one, endian)?;
        bytes.write_with(offset, self.two, endian)?;
        bytes.write_with(offset, self.three, endian)?;
        bytes.write_with(offset, self.four, endian)?;
        bytes.write_with(offset, self.five, endian)?;
        bytes.write_with(offset, self.six, endian)?;
        bytes.write_with(offset, self.seven, endian)?;
        bytes.write_with(offset, self.eight, endian)?;

        Ok(*offset)
    }
}
