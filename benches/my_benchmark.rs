#[macro_use]
extern crate criterion;

extern crate permutate;

use criterion::Criterion;
use permutate::{bin, Permutator, PermutatorWrapper as _};

fn get_input<'a>() -> [&'a [&'a str]; 4] {
    [
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..], // 10
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..], // 100
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..], // 1k
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..], // 10k
    ]
}

// Executes the same function as the binary, for 10,000 permutations.
fn bin_like_input() {
    let input: Vec<Vec<String>> = get_input()
        .into_iter()
        .map(|v| v.into_iter().cloned().map(String::from).collect())
        .collect();
    let (benchmark, _no_delimiters) = (true, false);
    bin::prepare(input, benchmark, _no_delimiters);
}

// Check to see if exactly 10,000 permutations were collected.
fn test_10k_permutations() {
    let input: Vec<&[&str]> = get_input().to_vec();
    assert_eq!(10_000, Permutator::new(&input).count())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("perm 10k", |b| b.iter(|| test_10k_permutations()));
    c.bench_function("bin-like 10k", |b| b.iter(|| bin_like_input()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
