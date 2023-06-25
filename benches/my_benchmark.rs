//! 使用criterion进行基准测试

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// 定义 fib 10 的基准测试函数
fn benchmark_fibonacci_10(c: &mut Criterion) {
    c.bench_function("fib 10", |b| b.iter(|| fibonacci(black_box(10))));
}

// 定义 fib 20 的基准测试函数
fn benchmark_fibonacci_20(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

// 定义 fib 30 的基准测试函数
fn benchmark_fibonacci_30(c: &mut Criterion) {
    c.bench_function("fib 30", |b| b.iter(|| fibonacci(black_box(30))));
}

// 将基准测试函数组合到一个基准组中
criterion_group!(
    benches,
    benchmark_fibonacci_10,
    benchmark_fibonacci_20,
    benchmark_fibonacci_30
);

// 使用 criterion_main 宏运行基准测试
criterion_main!(benches);
