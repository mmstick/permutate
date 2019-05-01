# Permutate

Permutate exists as both a library and application for permutating generic vectors or tuples
of lists, as well as individual lists, using an original Rust-based algorithm.
It has been developed primarily for the goal of inclusion within the Rust implementation of
the GNU Parallel program, and brace expansions within Redox's Ion shell.

Permutations work by incrementing a vector of index counters, and returning a vector of
references to the underlying data. For optimal usage, it is best to perform one iteration
with the `next()` method, and follow up successive iterations with the `next_with_buffer()`
method, so that you can re-use the previous vector allocation. It is also possible to obtain
the state of the internal index counters by using the `get_indexes()` method, and set the
state with the `set_indexes` method.

## Examples

These are a list of examples on how to use the library to manipulate string-based data.
The only thing we need to ensure is that our list of strings is in the `Vec<&[&str]>` format.

### An individual list: [&[&str]; 1]

```rust
extern crate permutate;
use permutate::{Permutator, PermutatorWrapper as _, Repeated};
use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let list: &[&str] = &["one", "two", "three", "four"];
    let list = [list];
    let mut permutator = Permutator::<Repeated<_>, _>::new(&list);

    if let Some(mut permutation) = permutator.next() {
        for element in &permutation {
            let _ = stdout.write(element.as_bytes());
        }
        let _ = stdout.write(b"\n");
        while let Some(permutation) = permutator.next_with_buffer(&mut permutation) {
            for element in permutation {
                let _ = stdout.write(element.as_bytes());
            }
            let _ = stdout.write(b"\n");
        }
    }
}
```

### A vec of slices: `Vec<&[&str]>`

```rust
extern crate permutate;
use permutate::{Permutator, PermutatorWrapper as _};
use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let lists = [
        &["one", "two", "three"][..],
        &["four", "five", "six"][..],
        &["seven", "eight", "nine"][..],
    ];
    let mut permutator = Permutator::new(&lists.to_vec());

    if let Some(mut permutation) = permutator.next() {
        for element in &permutation {
            let _ = stdout.write(element.as_bytes());
        }
        let _ = stdout.write(b"\n");
        while let Some(permutation) = permutator.next_with_buffer(&mut permutation) {
            for element in permutation {
                let _ = stdout.write(element.as_bytes());
            }
            let _ = stdout.write(b"\n");
        }
    }
}
```

### A Vector of Vector of Strings: `Vec<Vec<String>>`

This is the most complicated example to accomplish because you have to convert, essentially,
A vector of a vector of vectors into a slice of a slice of a slice, as the String type itself
is a vector of characters.

```rust
extern crate permutate;
use permutate::{Permutator, PermutatorWrapper as _};
use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let lists: Vec<Vec<String>> = vec![
        vec!["one".to_owned(), "two".to_owned(), "three".to_owned()],
        vec!["four".to_owned(), "five".to_owned(), "six".to_owned()],
        vec!["seven".to_owned(), "eight".to_owned(), "nine".to_owned()],
    ];

    // Convert the `Vec<Vec<String>>` into a `Vec<Vec<&str>>`
    let tmp: Vec<Vec<&str>> = lists.iter()
        .map(|list| list.iter().map(AsRef::as_ref).collect::<Vec<&str>>())
        .collect();

    // Convert the `Vec<Vec<&str>>` into a `Vec<&[&str]>`
    let vector_of_slices: Vec<&[&str]> = tmp.iter()
        .map(AsRef::as_ref).collect();

    // Initialize the Permutator
    let mut permutator = Permutator::new(&vector_of_slices);

    if let Some(mut permutation) = permutator.next() {
        for element in &permutation {
            let _ = stdout.write(element.as_bytes());
        }
        let _ = stdout.write(b"\n");
        while let Some(permutation) = permutator.next_with_buffer(&mut permutation) {
            for element in permutation {
                let _ = stdout.write(element.as_bytes());
            }
            let _ = stdout.write(b"\n");
        }
    }
}
```

### A tuple of slices: `(&[&str], &[bool])`

```rust
extern crate permutate;
use permutate::{Permutator, PermutatorWrapper as _};
use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let lists = (
        &["one", "two", "three"][..],
        &[false, true][..],
    );
    let mut permutator = Permutator::new(&lists);

    if let Some(mut permutation) = permutator.next() {
        let _ = stdout.write(permutation.0.as_bytes());
        let _ = stdout.write(permutation.1.to_string().as_bytes());
        let _ = stdout.write(b"\n");
        while let Some(permutation) = permutator.next_with_buffer(&mut permutation) {
            let _ = stdout.write(permutation.0.as_bytes());
            let _ = stdout.write(permutation.1.to_string().as_bytes());
            let _ = stdout.write(b"\n");
        }
    }
}
```
