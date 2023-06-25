//! 注意要使用nightly
//! rustup default nightly

#![feature(test)]

extern crate test;

fn fibonacci_u64(number: u64) -> u64 {
    let mut last: u64 = 1;
    let mut current: u64 = 0;
    let mut buffer: u64;
    let mut position: u64 = 1;

    return loop {
        if position == number {
            break current;
        }

        buffer = last;
        last = current;
        current = buffer + current;
        position += 1;
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
       assert_eq!(fibonacci_u64(1), 0);
       assert_eq!(fibonacci_u64(2), 1);
       assert_eq!(fibonacci_u64(12), 89);
       assert_eq!(fibonacci_u64(30), 514229);
    }

    #[bench]
    fn bench_u64(b: &mut Bencher) {
        b.iter(|| {
            for i in 100..200 {
                fibonacci_u64(i);
            }
        });
    }

    // 使用black_box
    #[bench]
    fn bench_u64_black_box(b: &mut Bencher) {
        b.iter(|| {
            for i in 100..200 {
                test::black_box(fibonacci_u64(i));
            }
        });
    }
}