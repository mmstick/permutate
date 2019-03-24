extern crate permutate;
use permutate::{Permutator, Repeated};

fn get_inputs<'a>() -> [&'a [&'a str]; 1] {
    [&["1", "2", "3"][..]]
}

fn get_expected<'a>() -> [&'a [&'a str]; 27] {
    [
        &["1", "1", "1"][..],
        &["1", "1", "2"][..],
        &["1", "1", "3"][..],
        &["1", "2", "1"][..],
        &["1", "2", "2"][..],
        &["1", "2", "3"][..],
        &["1", "3", "1"][..],
        &["1", "3", "2"][..],
        &["1", "3", "3"][..],
        &["2", "1", "1"][..],
        &["2", "1", "2"][..],
        &["2", "1", "3"][..],
        &["2", "2", "1"][..],
        &["2", "2", "2"][..],
        &["2", "2", "3"][..],
        &["2", "3", "1"][..],
        &["2", "3", "2"][..],
        &["2", "3", "3"][..],
        &["3", "1", "1"][..],
        &["3", "1", "2"][..],
        &["3", "1", "3"][..],
        &["3", "2", "1"][..],
        &["3", "2", "2"][..],
        &["3", "2", "3"][..],
        &["3", "3", "1"][..],
        &["3", "3", "2"][..],
        &["3", "3", "3"][..],
    ]
}

fn get_expected_b<'a>() -> [&'a [&'a str]; 3] {
    [&["1"][..], &["2"][..], &["3"][..]]
}

#[test]
fn single_list_permutation() {
    let input = get_inputs();
    let expected = get_expected();
    for (output, expected) in Permutator::<Repeated<_>, _>::new(&input).zip(expected[..].iter()) {
        assert_eq!(&output, expected);
    }
}

#[test]
fn single_list_permutation_b() {
    let input = get_inputs();
    let expected = get_expected_b();
    for (output, expected) in Permutator::new(&input[..]).zip(expected[..].iter()) {
        assert_eq!(&output, expected);
    }
}
