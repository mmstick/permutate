extern crate permutate;
use permutate::Permutator;

fn get_inputs<'a>() -> [&'a [&'a str]; 4] {
    [
        &["0", "1"][..],
        &["A", "B"][..],
        &["a", "b", "c"][..],
        &["_"][..],
    ]
}

fn get_expected<'a>() -> [&'a [&'a str]; 12] {
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
fn test_permutation_values_with_different_sizes() {
    let inputs = get_inputs();
    let expected = get_expected();

    for (output, expected) in Permutator::new(&inputs[..]).zip(expected[..].iter()) {
        assert_eq!(&output, expected);
    }
}

#[test]
// Verify that the permutations are generated with the correct values,
// in the correct order.
fn test_same_various_derefs() {
    let inputs = get_inputs();
    let expected = get_expected();

    for (output, expected) in
        Permutator::<[&[&str]], _>::new(&&&&&&inputs[..]).zip(expected[..].iter())
    {
        assert_eq!(&output, expected);
    }
}
