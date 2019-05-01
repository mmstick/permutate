#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;


fn blabla() {
    let a = ["A0", "B1", "C2", "D3", "E4", "F5", "G6", "H8", "I8", "J9"];

    let b = [
        &a[..], // 0
        &a[..], // 1
        &a[..], // 2
        &a[..], // 3
        &a[..], // 4
        &a[..], // 5
        &a[..], // 6
        &a[..], // 7
    ];
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
