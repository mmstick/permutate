extern crate permutate;
use permutate::{Permutator, Repeated};

fn get_input<'a>() -> [&'a [&'a str]; 1] {
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
    let input = get_input();
    let expected = get_expected();
    for (output, expected) in Permutator::<Repeated<_>, _>::new(&input).zip(expected[..].iter()) {
        assert_eq!(&output, expected);
    }
}

#[test]
fn single_list_permutation_with_buffer() {
    let input = get_input();
    let expected = get_expected();

    let mut permutator = Permutator::<Repeated<_>, _>::new(&input);
    let mut expected_iterator = expected[..].iter();

    if let Some(mut permutation) = permutator.next() {
        assert_eq!(&permutation, expected_iterator.next().unwrap());

        while let Some(permutation) = permutator.next_with_buffer(&mut permutation) {
            assert_eq!(&permutation, &expected_iterator.next().unwrap());
        }
    }

    // verifies that the expected iterator has been fully consumed
    assert!(expected_iterator.next().is_none())
}

#[test]
fn single_list_permutation_b() {
    let input = get_input();
    let expected = get_expected_b();
    for (output, expected) in Permutator::new(&input[..]).zip(expected[..].iter()) {
        assert_eq!(&output, expected);
    }
}

#[test]
fn single_list_permutation_b_with_buffer() {
    let input = get_input();
    let expected = get_expected_b();

    let mut permutator = Permutator::new(&input[..]);
    let mut expected_iterator = expected[..].iter();

    if let Some(mut permutation) = permutator.next() {
        assert_eq!(&permutation, expected_iterator.next().unwrap());

        while let Some(permutation) = permutator.next_with_buffer(&mut permutation) {
            assert_eq!(&permutation, &expected_iterator.next().unwrap());
        }
    }

    // verifies that the expected iterator has been fully consumed
    assert!(expected_iterator.next().is_none())
}
