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

#[test]
fn test_reset() {
    let input = get_input();
    let expected = get_expected();
    let mut permutator = Permutator::<Repeated<_>, _>::new(&input);
    for (output, expected) in permutator.by_ref().zip(expected[..].iter()) {
        assert_eq!(&output, expected);
    }
    permutator.reset();
    for (output, expected) in permutator.zip(expected[..].iter()) {
        assert_eq!(&output, expected);
    }
}
