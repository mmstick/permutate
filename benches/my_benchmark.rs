#[macro_use]
extern crate criterion;

extern crate permutate;

use criterion::Criterion;
use permutate::{Permutator, PermutatorWrapper as _, Repeated};

fn get_input<'a>() -> [&'a [&'a str]; 4] {
    [
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..], // 10
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..], // 100
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..], // 1k
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..], // 10k
    ]
}

// Check to see if exactly 10,000 permutations were collected.
fn bin_like_input() {
    let input: Vec<Vec<String>> = get_input()
        .into_iter()
        .map(|v| v.into_iter().cloned().map(String::from).collect())
        .collect();
    let count = bin_like_behaviour(input);
    assert_eq!(10_000, count);
}

fn bin_like_behaviour(list_vector: Vec<Vec<String>>) -> usize {
    type PermutatorStr<'a> = Permutator<Vec<&'a [&'a str]>, Vec<&'a str>>;
    type PermutatorRepeated<'a> = Permutator<Repeated<'a, &'a str>, Vec<&'a str>>;

    if let _should_repeat @ true = list_vector.len() == 1 {
        // Convert the Vec<Vec<String>> into a Vec<&str>
        let list_array: Vec<&str> = list_vector
            .iter()
            .next()
            .unwrap()
            .iter()
            .map(AsRef::as_ref)
            .collect();

        // Convert the Vec<&str> into a [&[&str]; 1]
        let list_array: Repeated<_> = [list_array.as_ref()];

        let mut permutator: PermutatorRepeated = Permutator::new(&list_array);
        permutator.count()
    } else {
        // Convert the Vec<Vec<String>> into a Vec<Vec<&str>>
        let list_array: Vec<Vec<&str>> = list_vector
            .iter()
            .map(|list| list.iter().map(AsRef::as_ref).collect::<Vec<&str>>())
            .collect();

        // Convert the Vec<Vec<&str>> into a Vec<&[&str]>
        let list_array: Vec<&[&str]> = list_array.iter().map(AsRef::as_ref).collect();

        // And then convert the `Permutator` with the &[&[&str]] as the input.
        let mut permutator: PermutatorStr = Permutator::new(&list_array);
        permutator.count()
    }
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
