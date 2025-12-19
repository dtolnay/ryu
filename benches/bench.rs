// cargo bench

#![feature(test)]

extern crate test;

use std::hint;
use std::io::Write;
use std::{f32, f64};
use test::Bencher;

macro_rules! benches {
    ($($name:ident($value:expr),)*) => {
        mod bench_ryu {
            use super::*;
            $(
                #[bench]
                fn $name(b: &mut Bencher) {
                    let mut buf = ryu::Buffer::new();

                    b.iter(move || {
                        let value = hint::black_box($value);
                        let formatted = buf.format_finite(value);
                        hint::black_box(formatted);
                    });
                }
            )*
        }

        mod bench_std_fmt {
            use super::*;
            $(
                #[bench]
                fn $name(b: &mut Bencher) {
                    let mut buf = Vec::with_capacity(20);

                    b.iter(|| {
                        buf.clear();
                        let value = hint::black_box($value);
                        write!(&mut buf, "{}", value).unwrap();
                        hint::black_box(buf.as_slice());
                    });
                }
            )*
        }
    };
}

benches! {
    bench_0_f64(0f64),
    bench_short_f64(0.1234f64),
    bench_e_f64(f64::consts::E),
    bench_max_f64(f64::MAX),
    bench_0_f32(0f32),
    bench_short_f32(0.1234f32),
    bench_e_f32(f32::consts::E),
    bench_max_f32(f32::MAX),
}
