extern crate permutate;
use permutate::Permutator;

fn get_inputs<'a>() -> [&'a [&'a str]; 3] {
    [
        &["1", "2", "3"][..],
        &["1", "2", "3"][..],
        &["1", "2", "3"][..],
    ]
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

#[test]
// Verify that the permutations are generated with the correct values,
// in the correct order.
fn test_permutation_values() {
    let inputs = get_inputs();
    let expected = get_expected();

    for (output, expected) in Permutator::new(&inputs[..]).zip(expected[..].iter()) {
        assert_eq!(&output, expected);
    }

    let mut permutator = Permutator::new(&inputs[..]);
    let mut expected = expected[..].iter();
    assert_eq!(&(permutator.nth(10).unwrap()), expected.nth(10).unwrap());
    assert_eq!(&(permutator.nth(0).unwrap()), expected.nth(0).unwrap());
}

#[test]
// Verify that the permutations are generated with the correct values,
// in the correct order re-using the permutation buffer.
fn test_permutation_values_with_buffer() {
    let inputs = get_inputs();
    let expected = get_expected();

    let mut permutator = Permutator::new(&inputs[..]);
    let mut expected_iterator = expected[..].iter();

    if let Some(mut permutation) = permutator.next() {
        assert_eq!(&permutation, expected_iterator.next().unwrap());

        while let Some(permutation) = permutator.next_with_buffer(&mut permutation) {
            assert_eq!(&permutation, &expected_iterator.next().unwrap());
        }
    }

    // verifies that the expected iterator has been fully consumed
    assert!(expected_iterator.next().is_none());
}
