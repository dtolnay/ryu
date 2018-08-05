extern crate rand;
extern crate ryu;

use std::mem;

use rand::{Rng, SeedableRng};

const SAMPLES: usize = 10000;
const ITERATIONS: usize = 1000;

struct MeanAndVariance {
    n: i64,
    mean: f64,
    m2: f64,
}

impl MeanAndVariance {
    fn new() -> Self {
        MeanAndVariance {
            n: 0,
            mean: 0.0,
            m2: 0.0,
        }
    }

    fn update(&mut self, x: f64) {
        self.n += 1;
        let d = x - self.mean;
        self.mean += d / self.n as f64;
        let d2 = x - self.mean;
        self.m2 += d * d2;
    }

    fn variance(&self) -> f64 {
        self.m2 / (self.n - 1) as f64
    }
}

macro_rules! benchmark {
    ($name:ident, |$var:ident: $ty:ident| $computation:expr) => {
        fn $name() -> usize {
            let mut rng = rand::prng::XorShiftRng::from_seed([123u8; 16]);
            let mut mv = MeanAndVariance::new();
            let mut throwaway = 0;
            for _ in 0..SAMPLES {
                let $var = loop {
                    let f = $ty::from_bits(rng.gen());
                    if f.is_finite() {
                        break f;
                    }
                };

                let t1 = std::time::SystemTime::now();
                for _ in 0..ITERATIONS {
                    throwaway += $computation;
                }
                let duration = t1.elapsed().unwrap();
                let nanos = duration.as_secs() * 1_000_000_000 + duration.subsec_nanos() as u64;
                mv.update(nanos as f64 / ITERATIONS as f64);
            }
            println!(
                "{:12} {:8.3} {:8.3}",
                concat!(stringify!($name), ":"),
                mv.mean,
                mv.variance().sqrt()
            );
            throwaway
        }
    };
}

benchmark!(original32, |f: f32| unsafe {
    let mut buffer: [u8; 15] = mem::uninitialized();
    ryu::raw::f2s_buffered_n(f, &mut buffer[0])
});

benchmark!(pretty32, |f: f32| ryu::Buffer::new().format(f).len());

benchmark!(original64, |f: f64| unsafe {
    let mut buffer: [u8; 24] = mem::uninitialized();
    ryu::raw::d2s_buffered_n(f, &mut buffer[0])
});

benchmark!(pretty64, |f: f64| ryu::Buffer::new().format(f).len());

fn main() {
    println!("{:>20}{:>9}", "Average", "Stddev");
    let mut throwaway = 0;
    throwaway += original32();
    throwaway += pretty32();
    throwaway += original64();
    throwaway += pretty64();
    if std::env::var_os("ryu-benchmark").is_some() {
        // Prevent the compiler from optimizing the code away.
        println!("{}", throwaway);
    }
}
