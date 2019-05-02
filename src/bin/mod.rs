#![allow(missing_docs)]

pub mod arguments;
pub mod buffer;
pub mod man;

use std::io::{self, StdoutLock, Write};
use std::process::exit;

use self::arguments::InputError;
use self::buffer::platform::BUFFER_SIZE;
use self::buffer::StdoutBuffer;
use crate::{ListWrapper, Permutator, PermutatorWrapper, Repeated};

pub fn prepare(list_vector: Vec<Vec<String>>, benchmark: bool, no_delimiters: bool) -> () {
    type PermutatorStr<'a> = Permutator<Vec<&'a [&'a str]>, Vec<&'a str>>;
    type PermutatorRepeated<'a> = Permutator<Repeated<'a, &'a str>, Vec<&'a str>>;

    if let _should_repeat @ true = list_vector.len() == 1 {
        // Convert the Vec<Vec<String>> into a Vec<&str>
        let list_array: Vec<&str> = list_vector
            .iter()
            .next()
            .unwrap()
            .iter()
            .map(AsRef::as_ref)
            .collect();

        // Convert the Vec<&str> into a [&[&str]; 1]
        let list_array: Repeated<_> = [list_array.as_ref()];

        let mut permutator: PermutatorRepeated = Permutator::new(&list_array);
        if benchmark {
            let _ = permutator.count();
        } else {
            if no_delimiters {
                permutate_without_delims(&mut permutator);
            } else {
                permutate(&mut permutator);
            }
        }
    } else {
        // Convert the Vec<Vec<String>> into a Vec<Vec<&str>>
        let list_array: Vec<Vec<&str>> = list_vector
            .iter()
            .map(|list| list.iter().map(AsRef::as_ref).collect::<Vec<&str>>())
            .collect();

        // Convert the Vec<Vec<&str>> into a Vec<&[&str]>
        let list_array: Vec<&[&str]> = list_array.iter().map(AsRef::as_ref).collect();

        // And then convert the `Permutator` with the &[&[&str]] as the input.
        let mut permutator: PermutatorStr = Permutator::new(&list_array);
        if benchmark {
            let _ = permutator.count();
        } else {
            if no_delimiters {
                permutate_without_delims(&mut permutator);
            } else {
                permutate(&mut permutator);
            }
        }
    };
}

pub fn permutate<'a, P, LW>(permutator: &'a mut P)
where
    P: PermutatorWrapper<LW, Vec<&'a str>> + Iterator<Item = Vec<&'a str>>,
    LW: ListWrapper<Vec<&'a str>> + Clone,
{
    let mut buffer = StdoutBuffer::new();
    let mut current_output = permutator.next().unwrap();
    // This first run through will count the number of bytes that will be
    // required to print each permutation to standard output.
    {
        let mut current_permutation = current_output.iter();
        buffer.write(current_permutation.next().unwrap().as_bytes());
        buffer.push(b' ');
        buffer.write(current_permutation.next().unwrap().as_bytes());
        for element in current_permutation {
            buffer.push(b' ');
            buffer.write(element.as_bytes())
        }
    }

    buffer.push(b'\n');

    // Using the number of bytes of the first iteration, we can calculate
    // how many iterations that we can safely fit into our buffer.
    let permutations_per_buffer = BUFFER_SIZE / buffer.capacity;

    // Locking the buffers will improve performance greatly due to not needing
    // to worry about repeatedly locking and unlocking them throughout the program.
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Each permutation will check to see if the max number of permutations per
    // buffer has been allocated and prints it to standard output if true.
    let mut counter = 1;
    while let Some(current_output) = permutator.next_with_buffer(&mut current_output) {
        if counter == permutations_per_buffer {
            buffer.write_and_clear(&mut stdout);
            counter = 0;
        }

        // The first element will print a space after the element.
        let mut current_permutation = current_output.iter();
        buffer.write(current_permutation.next().unwrap().as_bytes());
        buffer.push(b' ');
        buffer.write(current_permutation.next().unwrap().as_bytes());
        for element in current_permutation {
            buffer.push(b' ');
            buffer.write(element.as_bytes())
        }
        buffer.push(b'\n');
        counter += 1;
    }

    // Print the remaining buffer to standard output.
    let _ = stdout.write_all(&buffer.data[..]);
}

pub fn permutate_without_delims<'a, P, LW>(permutator: &'a mut P)
where
    P: PermutatorWrapper<LW, Vec<&'a str>> + Iterator<Item = Vec<&'a str>>,
    LW: ListWrapper<Vec<&'a str>> + Clone,
{
    // This first run through will count the number of bytes that will be
    // required to print each permutation to standard output.
    let mut buffer = StdoutBuffer::new();
    let mut current_output = permutator.next().unwrap();
    {
        // There will always be at least two elements in a permutation.
        let mut permutation = current_output.iter();
        buffer.write(permutation.next().unwrap().as_bytes());
        buffer.write(permutation.next().unwrap().as_bytes());
        for element in permutation {
            buffer.write(element.as_bytes());
        }
    }

    // Append a newline after each permutation to print them on separate lines.
    buffer.push(b'\n');

    // Using the number of bytes of the first iteration, we can calculate
    // how many iterations that we can safely fit into our buffer.
    let permutations_per_buffer = BUFFER_SIZE / buffer.capacity;

    // Locking the buffers will improve performance greatly due to not needing
    // to worry about repeatedly locking and unlocking them throughout the program.
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Each permutation will check to see if the max number of permutations per
    // buffer has been allocated and prints it to standard output if true.
    let mut counter = 1;
    while let Some(current_output) = permutator.next_with_buffer(&mut current_output) {
        let mut permutation = current_output.iter();
        if counter == permutations_per_buffer {
            buffer.write_and_clear(&mut stdout);
            counter = 0;
        }

        // There will always be at least two elements in a permutation.
        buffer.write(permutation.next().unwrap().as_bytes());
        buffer.write(permutation.next().unwrap().as_bytes());
        for element in permutation {
            buffer.write(element.as_bytes());
        }
        buffer.push(b'\n');
        counter += 1;
    }

    // Print the remaining buffer to standard output.
    let _ = stdout.write_all(&buffer.data[..]);
}
