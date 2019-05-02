extern crate permutate;
use permutate::{Permutator, PermutatorWrapper as _};

fn get_input<'a>() -> &'a (&'a [&'a str],) {
    &(&["A", "B", "C"],)
}
fn get_expected_a<'a>() -> [&'a (&'a str,); 3] {
    [&("A",), &("B",), &("C",)]
}

#[test]
fn test_tuple_a() {
    let input = get_input().clone();
    let expected = get_expected_a();
    for (output, expected) in Permutator::new(&input).zip(expected[..].iter()) {
        assert_eq!(&output, *expected);

        // also assert that for each value pair, both point to the same address
        let (o, e) = (output.0, expected.0);
        assert_eq!(o as *const str, e as *const str);
    }
}

#[test]
fn test_tuple_a_with_buffer() {
    let input = get_input().clone();
    let expected = get_expected_a();

    let mut permutator = Permutator::new(&input);
    let mut expected_iterator = expected[..].iter();

    if let Some(mut permutation) = permutator.next() {
        let expected_permutation = expected_iterator.next().unwrap();
        assert_eq!(&&permutation, expected_permutation);

        // also assert that for each value pair, both point to the same address
        let (o, e) = (permutation.0, expected_permutation.0);
        assert_eq!(o as *const str, e as *const str);

        while let Some(permutation) = permutator.next_with_buffer(&mut permutation) {
            let expected_permutation = expected_iterator.next().unwrap();
            assert_eq!(&permutation, expected_permutation);

            // also assert that for each value pair, both point to the same address
            let (o, e) = (permutation.0, expected_permutation.0);
            assert_eq!(o as *const str, e as *const str);
        }
    }

    // verifies that the expected iterator has been fully consumed
    assert!(expected_iterator.next().is_none())
}

fn get_input_b<'a>() -> &'a (&'a [&'a str], &'a [&'a i32]) {
    &(&["A", "B", "C"], &[&0, &1, &2])
}
fn get_expected_b<'a>() -> [&'a (&'a str, &'a i32); 9] {
    [
        &("A", &0),
        &("A", &1),
        &("A", &2),
        &("B", &0),
        &("B", &1),
        &("B", &2),
        &("C", &0),
        &("C", &1),
        &("C", &2),
    ]
}

#[test]
fn test_tuple_b() {
    let input = get_input_b().clone();
    let expected = get_expected_b();
    for (output, expected) in Permutator::new(&input).zip(expected[..].iter()) {
        assert_eq!(&output, *expected);

        // also assert that for each value pair, both point to the same address
        let (o, e) = (output.0, expected.0);
        assert_eq!(o as *const str, e as *const str);
        let (o, e) = (output.1, expected.1);
        assert_eq!(o as *const i32, e as *const i32);
    }
}

#[test]
fn test_tuple_b_with_buffer() {
    let input = get_input_b().clone();
    let expected = get_expected_b();

    let mut permutator = Permutator::new(&input);
    let mut expected_iterator = expected[..].iter();

    if let Some(mut permutation) = permutator.next() {
        let expected_permutation = expected_iterator.next().unwrap();
        assert_eq!(&&permutation, expected_permutation);

        // also assert that for each value pair, both point to the same address
        let (o, e) = (permutation.0, expected_permutation.0);
        assert_eq!(o as *const str, e as *const str);
        let (o, e) = (permutation.1, expected_permutation.1);
        assert_eq!(o as *const i32, e as *const i32);

        while let Some(permutation) = permutator.next_with_buffer(&mut permutation) {
            let expected_permutation = expected_iterator.next().unwrap();
            assert_eq!(&permutation, expected_permutation);

            // also assert that for each value pair, both point to the same address
            let (o, e) = (permutation.0, expected_permutation.0);
            assert_eq!(o as *const str, e as *const str);
            let (o, e) = (permutation.1, expected_permutation.1);
            assert_eq!(o as *const i32, e as *const i32);
        }
    }

    // verifies that the expected iterator has been fully consumed
    assert!(expected_iterator.next().is_none())
}

fn get_input_c<'a>() -> &'a (&'a [&'a str], &'a [&'a i32], &'a [&'a bool]) {
    &(&["A", "B"], &[&0, &1, &2], &[&false, &true])
}
fn get_expected_c<'a>() -> [&'a (&'a str, &'a i32, &'a bool); 12] {
    [
        &("A", &0, &false),
        &("A", &0, &true),
        &("A", &1, &false),
        &("A", &1, &true),
        &("A", &2, &false),
        &("A", &2, &true),
        &("B", &0, &false),
        &("B", &0, &true),
        &("B", &1, &false),
        &("B", &1, &true),
        &("B", &2, &false),
        &("B", &2, &true),
    ]
}

#[test]
fn test_tuple_c() {
    let input = get_input_c().clone();
    let expected = get_expected_c();
    for (output, expected) in Permutator::new(&input).zip(expected[..].iter()) {
        assert_eq!(&output, *expected);

        // also assert that for each value pair, both point to the same address
        let (o, e) = (output.0, expected.0);
        assert_eq!(o as *const str, e as *const str);
        let (o, e) = (output.1, expected.1);
        assert_eq!(o as *const i32, e as *const i32);
        let (o, e) = (output.2, expected.2);
        assert_eq!(o as *const bool, e as *const bool);
    }
}

#[test]
fn test_tuple_c_with_buffer() {
    let input = get_input_c().clone();
    let expected = get_expected_c();

    let mut permutator = Permutator::new(&input);
    let mut expected_iterator = expected[..].iter();

    if let Some(mut permutation) = permutator.next() {
        let expected_permutation = expected_iterator.next().unwrap();
        assert_eq!(&&permutation, expected_permutation);

        // also assert that for each value pair, both point to the same address
        let (o, e) = (permutation.0, expected_permutation.0);
        assert_eq!(o as *const str, e as *const str);
        let (o, e) = (permutation.1, expected_permutation.1);
        assert_eq!(o as *const i32, e as *const i32);
        let (o, e) = (permutation.2, expected_permutation.2);
        assert_eq!(o as *const bool, e as *const bool);

        while let Some(permutation) = permutator.next_with_buffer(&mut permutation) {
            let expected_permutation = expected_iterator.next().unwrap();
            assert_eq!(&permutation, expected_permutation);

            // also assert that for each value pair, both point to the same address
            let (o, e) = (permutation.0, expected_permutation.0);
            assert_eq!(o as *const str, e as *const str);
            let (o, e) = (permutation.1, expected_permutation.1);
            assert_eq!(o as *const i32, e as *const i32);
            let (o, e) = (permutation.2, expected_permutation.2);
            assert_eq!(o as *const bool, e as *const bool);
        }
    }

    // verifies that the expected iterator has been fully consumed
    assert!(expected_iterator.next().is_none())
}

fn get_input_d<'a>() -> &'a (&'a [&'a str], &'a [i32]) {
    &(&["A", "B"], &[0, 1])
}
fn get_expected_d<'a>() -> [&'a (&'a str, i32); 4] {
    [&("A", 0), &("A", 1), &("B", 0), &("B", 1)]
}

#[test]
fn test_tuple_d() {
    let input = get_input_d().clone();
    let expected = get_expected_d();
    for (output, expected) in Permutator::new(&input).zip(expected[..].iter()) {
        assert_eq!(&output, *expected);

        // also assert that for the first value pair, both point to the same address
        let (o, e) = (output.0, expected.0);
        assert_eq!(o as *const str, e as *const str);
        // and assert that for the second value pair, they have different addresses
        let (o, e) = (output.1, expected.1);
        assert!(&o as *const i32 != &e as *const i32);
    }
}

#[test]
fn test_tuple_d_with_buffer() {
    let input = get_input_d().clone();
    let expected = get_expected_d();

    let mut permutator = Permutator::new(&input);
    let mut expected_iterator = expected[..].iter();

    if let Some(mut permutation) = permutator.next() {
        let expected_permutation = expected_iterator.next().unwrap();
        assert_eq!(&&permutation, expected_permutation);

        // also assert that for the first value pair, both point to the same address
        let (o, e) = (permutation.0, expected_permutation.0);
        assert_eq!(o as *const str, e as *const str);
        // and assert that for the second value pair, they have different addresses
        let (o, e) = (permutation.1, expected_permutation.1);
        assert!(&o as *const i32 != &e as *const i32);

        while let Some(permutation) = permutator.next_with_buffer(&mut permutation) {
            let expected_permutation = expected_iterator.next().unwrap();
            assert_eq!(&permutation, expected_permutation);

            // also assert that for the first value pair, both point to the same address
            let (o, e) = (permutation.0, expected_permutation.0);
            assert_eq!(o as *const str, e as *const str);
            // and assert that for the second value pair, they have different addresses
            let (o, e) = (permutation.1, expected_permutation.1);
            assert!(&o as *const i32 != &e as *const i32)
        }
    }

    // verifies that the expected iterator has been fully consumed
    assert!(expected_iterator.next().is_none())
}
