extern crate permutate;
use permutate::Permutator;

fn get_inputs<'a>() -> [&'a [&'a str]; 6] {
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
    let inputs = get_inputs();

    assert_eq!(1_000_000, Permutator::new(&inputs[..]).count())
}