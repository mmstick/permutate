extern crate permutate;
use permutate::{Permutator, PermutatorWrapper as _};

fn get_input<'a>() -> [&'a [&'a str]; 6] {
    [
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..],
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..],
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..],
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..],
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..],
        &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][..],
    ]
}

#[test]
// Check to see if exactly 1,000,000 permutations were collected.
fn test_million_permutations() {
    let input: Vec<&[&str]> = get_input().to_vec();
    assert_eq!(1_000_000, Permutator::new(&input).count())
}
