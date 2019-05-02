# Permutate

Permutate exists as both a library and application for permutating
generic vectors or tuples of lists, as well as individual lists,
using an original Rust-based algorithm.
It has been developed primarily for the goal of inclusion within the
Rust implementation of the GNU Parallel program, and brace expansions
within Redox's Ion shell.

The source code documentation may be found on
[Docs.rs](https://docs.rs/permutate/).

## Features

- `bin-utils` - if set then the binary utilities are included.
    - Set by default and is required by the `bin` and `bench` profiles.

## Mechanics

Permutations work by incrementing a vector of index counters,
and returning a vector of references to the underlying data
(unless the container owns the values and they are Copy).
For optimal usage, it is best to perform one iteration with the 
`next()` method, and follow up successive iterations with the
`next_with_buffer()` method, so that you can re-use the previous
vector allocation.
It is also possible to obtain the state of the internal index counters
by using the `get_indexes()` method, and set the state with the
`set_indexes` method.

## Examples

These are a list of examples on how to use the library to manipulate
various types of data.
The only thing we may need to ensure is that our list of strings is in the `Vec<&[&str]>` format.

### An individual list: [&[&str]; 1]

```rust
extern crate permutate;
use permutate::{Permutator, PermutatorWrapper as _, Repeated};
use std::io::{self, Write};

fn main() {
    let list: &[&str] = &["one", "two", "three", "four"];
    let list = [list];
    let mut permutator = Permutator::<Repeated<_>, _>::new(&list);

    // iteration 1: re-utilizes the permutation buffer
    // you may opt to re-utilize or not (see iteration 2)
    if let Some(mut permutation) = permutator.next() {
        // prints each element
        for element in &permutation {
            println!("{:?}", &element);
        }
        // re-utilizes the permutation buffer
        while let Some(permutation) = permutator.next_with_buffer(&mut permutation) {
            // prints each element
            for element in permutation {
                println!("{:?}", &element);
            }
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
    let lists = [
        &["one", "two", "three"][..],
        &["four", "five", "six"][..],
        &["seven", "eight", "nine"][..],
    ];
    let mut permutator = Permutator::new(&lists.to_vec());

    // iteration 2: allocates a new buffer for each permutation
    // you may opt to re-allocate or not (see iteration 1)
    for permutation in permutator {
        println!("{:?}", &permutation);
    }
}
```

### A Vector of Vector of Strings: `Vec<Vec<String>>`

This is the most complicated example to accomplish because
you have to convert, essentially, a vector of a vector of vectors into
a slice of a slice of a slice,
as the String type itself is a vector of characters.

```rust
extern crate permutate;
use permutate::{Permutator, PermutatorWrapper as _};
use std::io::{self, Write};

fn main() {
    let lists: Vec<Vec<String>> = vec![
        vec!["one".to_owned(), "two".to_owned(), "three".to_owned()],
        vec!["four".to_owned(), "five".to_owned(), "six".to_owned()],
        vec!["seven".to_owned(), "eight".to_owned(), "nine".to_owned()],
    ];

    // Convert the `Vec<Vec<String>>` into a `Vec<Vec<&str>>`
    let tmp: Vec<Vec<&str>> = lists.iter()
        .map(|list| list.iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>()
        )
        .collect();

    // Convert the `Vec<Vec<&str>>` into a `Vec<&[&str]>`
    let vector_of_slices: Vec<&[&str]> = tmp.iter()
        .map(AsRef::as_ref).collect();

    // Initialize the Permutator
    let mut permutator = Permutator::new(&vector_of_slices);

    // iteration 2: allocates a new buffer for each permutation
    // you may opt to re-allocate or not (see iteration 1)
    for permutation in permutator {
        println!("{:?}", &permutation);
    }
}
```

### A tuple of slices: `(&[&str], &[bool])`

```rust
extern crate permutate;
use permutate::{Permutator, PermutatorWrapper as _};
use std::io::{self, Write};

fn main() {
    let lists = (
        &["one", "two", "three"][..],
        &[false, true][..],
    );
    let mut permutator = Permutator::new(&lists);

    // iteration 2: allocates a new buffer for each permutation
    // you may opt to re-allocate or not (see iteration 1)
    for permutation in permutator {
        let (s, b): (&str, bool) = permutation;
        println!("{:?}", &(s, b));
    }
}
```
Note that the slice of booleans actually owns the booleans,
and so they are `copy`ed at each permutation.

## Application

Following the spirit of the Rust and UNIX philosophy,
I am also releasing this as it's own simple application to bring the
capabilities of the permutate to the command-line, because shell lives
matter.
The syntax is very much identical to GNU Parallel,
so users of GNU Parallel will be right at home with this command.

```sh
$ permutate A B ::: C D ::: E F
A C E
A C F
A D E
A D F
B C E
B C F
B D E
B D F
```

```sh
$ permutate -n A B ::: C D ::: E F
ACE
ACF
ADE
ADF
BCE
BCF
BDE
BDF
```

Other accepted syntaxes are:

```sh
$ permutate -f file file :::+ arg arg :::: file file ::::+ file file ::: arg arg

```

### Benchmark

So how fast is it?
try running `cargo bench` to see how fast it creates 10k permutations 
over string references, repeated over 10k iterations.  
It takes ~550us per iteration on my laptop 
(i7-4710HQ Quad Core capped at 1600MHz on ArchLinux), 
ie. around 18M permutations per second
(= 10k[permutations/iteration] * 10k[iterations] / 5.50[seconds]).

Another way of benchmarking it is to use the generated binary directly:
```sh
$ cargo build --release
$ for char in A B C D E F G H I J; do echo $char >> A; done
$ time target/release/permutate --benchmark -n -f A :::: A :::: A :::: A :::: A :::: A :::: A :::: A
$ rm A
```
This will also test for 100M permutations (10^8 permutations),
then you only need to divide 100,000,000 by the duration.
