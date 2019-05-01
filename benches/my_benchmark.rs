#[macro_use]
extern crate criterion;

extern crate permutate;

// use permutate::{Permutator, PermutatorWrapper as _};
use permutate::Permutator;
use criterion::Criterion;
use criterion::black_box;

fn get_input<'a>() -> [&'a [&'a str]; 4] {
    [
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..], // 10
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..], // 100
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..], // 1k
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..], // 10k
    ]
}

// Check to see if exactly 10,000 permutations were collected.
fn test_10k_permutations() {
    let input: Vec<&[&str]> = get_input().to_vec();
    assert_eq!(10_000, Permutator::new(&input).count())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("perm 10k", |b| b.iter(|| test_10k_permutations()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
