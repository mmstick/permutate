extern crate permutate;
use permutate::Permutator;

fn get_input<'a>() -> [&'a [&'a str]; 3] {
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
    let input = get_input();
    let expected = get_expected();

    for (output, expected) in Permutator::new(&input[..]).zip(expected[..].iter()) {
        assert_eq!(&output, expected);
    }

    let mut permutator = Permutator::new(&input[..]);
    let mut expected = expected[..].iter();
    assert_eq!(&(permutator.nth(10).unwrap()), expected.nth(10).unwrap());
    assert_eq!(&(permutator.nth(0).unwrap()), expected.nth(0).unwrap());
}

#[test]
// Verify that the permutations are generated with the correct values,
// in the correct order re-using the permutation buffer.
fn test_permutation_values_with_buffer() {
    let input = get_input();
    let expected = get_expected();

    let mut permutator = Permutator::new(&input[..]);
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

fn get_input_b<'a>() -> [&'a [&'a str]; 4] {
    [
        &["0", "1"][..],
        &["A", "B"][..],
        &["a", "b", "c"][..],
        &["_"][..],
    ]
}

fn get_expected_b<'a>() -> [&'a [&'a str]; 12] {
    [
        &["0", "A", "a", "_"],
        &["0", "A", "b", "_"],
        &["0", "A", "c", "_"],
        &["0", "B", "a", "_"],
        &["0", "B", "b", "_"],
        &["0", "B", "c", "_"],
        &["1", "A", "a", "_"],
        &["1", "A", "b", "_"],
        &["1", "A", "c", "_"],
        &["1", "B", "a", "_"],
        &["1", "B", "b", "_"],
        &["1", "B", "c", "_"],
    ]
}

#[test]
// Verify that the permutations are generated with the correct values,
// in the correct order.
fn test_permutation_values_b() {
    let input = get_input_b();
    let expected = get_expected_b();

    for (output, expected) in Permutator::new(&input[..]).zip(expected[..].iter()) {
        assert_eq!(&output, expected);
    }
}

#[test]
// Verify that the permutations are generated with the correct values,
// in the correct order.
fn test_permutation_values_b_derefs() {
    let input = get_input_b();
    let expected = get_expected_b();

    for (output, expected) in
        Permutator::<[&[&str]], _>::new(&&&&&&input[..]).zip(expected[..].iter())
    {
        assert_eq!(&output, expected);
    }
}
