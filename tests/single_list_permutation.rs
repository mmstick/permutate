extern crate permutate;
use permutate::{Permutator, PermutatorWrapper as _, Repeated};

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

        // also assert that for each value pair, both point to the same address
        for (o, e) in (*output).iter().zip(&expected[..]) {
            assert_eq!(*o as *const str, *e as *const str);
        }
    }
}

#[test]
fn single_list_permutation_with_buffer() {
    let input = get_input();
    let expected = get_expected();

    let mut permutator = Permutator::<Repeated<_>, _>::new(&input);
    let mut expected_iterator = expected[..].iter();

    if let Some(mut permutation) = permutator.next() {
        let expected_permutation = expected_iterator.next().unwrap();
        assert_eq!(&permutation, expected_permutation);

        // also assert that for each value pair, both point to the same address
        for (o, e) in (*permutation).iter().zip(*expected_permutation) {
            assert_eq!(*o as *const str, *e as *const str);
        }

        while let Some(permutation) = permutator.next_with_buffer(&mut permutation) {
            let expected_permutation = expected_iterator.next().unwrap();
            assert_eq!(&permutation.as_slice(), expected_permutation);

            // also assert that for each value pair, both point to the same address
            for (o, e) in (*permutation).iter().zip(*expected_permutation) {
                assert_eq!(*o as *const str, *e as *const str);
            }
        }
    }

    // verifies that the expected iterator has been fully consumed
    assert!(expected_iterator.next().is_none())
}

#[test]
fn single_list_permutation_b() {
    let input = get_input().to_vec();
    let expected = get_expected_b();
    for (output, expected) in Permutator::new(&input).zip(expected[..].iter()) {
        assert_eq!(&output, expected);

        // also assert that for each value pair, both point to the same address
        for (o, e) in (*output).iter().zip(*expected) {
            assert_eq!(*o as *const str, *e as *const str);
        }
    }
}

#[test]
fn single_list_permutation_b_with_buffer() {
    let input = get_input().to_vec();
    let expected = get_expected_b();

    let mut permutator = Permutator::new(&input);
    let mut expected_iterator = expected[..].iter();

    if let Some(mut permutation) = permutator.next() {
        let expected_permutation = expected_iterator.next().unwrap();
        assert_eq!(&permutation, expected_permutation);

        // also assert that for each value pair, both point to the same address
        for (o, e) in (*permutation).iter().zip(*expected_permutation) {
            assert_eq!(*o as *const str, *e as *const str);
        }

        while let Some(permutation) = permutator.next_with_buffer(&mut permutation) {
            let expected_permutation = expected_iterator.next().unwrap();
            assert_eq!(&permutation, &expected_permutation);

            // also assert that for each value pair, both point to the same address
            for (o, e) in (*permutation).iter().zip(*expected_permutation) {
                assert_eq!(*o as *const str, *e as *const str);
            }
        }
    }

    // verifies that the expected iterator has been fully consumed
    assert!(expected_iterator.next().is_none())
}

fn get_input_c<'a>() -> [&'a [i32]; 1] {
    [&[1, 2, 3][..]]
}

fn get_expected_c<'a>() -> [&'a [i32]; 27] {
    [
        &[1, 1, 1][..],
        &[1, 1, 2][..],
        &[1, 1, 3][..],
        &[1, 2, 1][..],
        &[1, 2, 2][..],
        &[1, 2, 3][..],
        &[1, 3, 1][..],
        &[1, 3, 2][..],
        &[1, 3, 3][..],
        &[2, 1, 1][..],
        &[2, 1, 2][..],
        &[2, 1, 3][..],
        &[2, 2, 1][..],
        &[2, 2, 2][..],
        &[2, 2, 3][..],
        &[2, 3, 1][..],
        &[2, 3, 2][..],
        &[2, 3, 3][..],
        &[3, 1, 1][..],
        &[3, 1, 2][..],
        &[3, 1, 3][..],
        &[3, 2, 1][..],
        &[3, 2, 2][..],
        &[3, 2, 3][..],
        &[3, 3, 1][..],
        &[3, 3, 2][..],
        &[3, 3, 3][..],
    ]
}

#[test]
// Verify that the permutations are generated with the correct values,
// in the correct order.
fn test_value_owned_permutation() {
    let input = get_input_c();
    let expected = get_expected_c();

    for (output, expected) in Permutator::<Repeated<_>, _>::new(&input).zip(expected[..].iter()) {
        assert_eq!(&output, expected);

        // also assert that for each value pair, they have different addresses
        for (o, e) in (*output).iter().zip(&expected[..]) {
            assert!(o as *const i32 != e as *const i32);
        }
    }

    let mut permutator = Permutator::new(&input);
    let mut expected = expected[..].iter();
    assert_eq!(&(permutator.nth(10).unwrap()), expected.nth(10).unwrap());
    assert_eq!(&(permutator.nth(0).unwrap()), expected.nth(0).unwrap());
}

fn get_input_d<'a>() -> [&'a [&'a i32]; 1] {
    [&[&1, &2, &3][..]]
}

fn get_expected_d<'a>() -> [&'a [&'a i32]; 27] {
    [
        &[&1, &1, &1][..],
        &[&1, &1, &2][..],
        &[&1, &1, &3][..],
        &[&1, &2, &1][..],
        &[&1, &2, &2][..],
        &[&1, &2, &3][..],
        &[&1, &3, &1][..],
        &[&1, &3, &2][..],
        &[&1, &3, &3][..],
        &[&2, &1, &1][..],
        &[&2, &1, &2][..],
        &[&2, &1, &3][..],
        &[&2, &2, &1][..],
        &[&2, &2, &2][..],
        &[&2, &2, &3][..],
        &[&2, &3, &1][..],
        &[&2, &3, &2][..],
        &[&2, &3, &3][..],
        &[&3, &1, &1][..],
        &[&3, &1, &2][..],
        &[&3, &1, &3][..],
        &[&3, &2, &1][..],
        &[&3, &2, &2][..],
        &[&3, &2, &3][..],
        &[&3, &3, &1][..],
        &[&3, &3, &2][..],
        &[&3, &3, &3][..],
    ]
}

#[test]
// Verify that the permutations are generated with the correct values,
// in the correct order.
fn test_value_permutation_b() {
    let input = get_input_d();
    let expected = get_expected_d();

    for (output, expected) in Permutator::<Repeated<_>, _>::new(&input).zip(expected[..].iter()) {
        assert_eq!(&output, expected);

        // also assert that for each value pair, both point to the same address
        for (o, e) in (*output).iter().zip(&expected[..]) {
            assert_eq!(*o as *const i32, *e as *const i32);
        }
    }

    let mut permutator = Permutator::new(&input);
    let mut expected = expected[..].iter();
    assert_eq!(&(permutator.nth(10).unwrap()), expected.nth(10).unwrap());
    assert_eq!(&(permutator.nth(0).unwrap()), expected.nth(0).unwrap());
}
