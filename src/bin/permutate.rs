extern crate permutate;

mod arguments;
mod buffer;
mod man;

use std::io::{self, StdoutLock, Write};
use std::process::exit;

use arguments::InputError;
use buffer::platform::BUFFER_SIZE;
use buffer::StdoutBuffer;
use permutate::{ListWrapper, Permutator, PermutatorWrapper, Repeated};

type PermutatorStr<'a> = Permutator<Vec<&'a [&'a str]>, Vec<&'a str>>;
type PermutatorRepeated<'a> = Permutator<Repeated<'a, &'a str>, Vec<&'a str>>;

fn main() {
    // First, the program should grab a handle to stdout and stderr and lock them.
    let stdout = io::stdout();
    let stderr = io::stderr();

    // Locking the buffers will improve performance greatly due to not needing
    // to worry about repeatedly locking and unlocking them throughout the program.
    let mut stdout = stdout.lock();
    let mut stderr = stderr.lock();

    let (input, benchmark, interpret_files, no_delimiters) = arguments::parse_options(&mut stdout);

    let mut list_vector = Vec::new();
    match arguments::parse_arguments(&mut list_vector, &input.join(" "), interpret_files) {
        Ok(_) => {
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
                        permutate_without_delims(&mut stdout, &mut permutator);
                    } else {
                        permutate(&mut stdout, &mut permutator);
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
                        permutate_without_delims(&mut stdout, &mut permutator);
                    } else {
                        permutate(&mut stdout, &mut permutator);
                    }
                }
            };
        }
        Err(why) => {
            let _ = stderr.write(b"permutate: parse error: ");
            match why {
                InputError::FileError(path, why) => {
                    let _ = stderr.write(path.as_bytes());
                    let _ = stderr.write(b" could not be read: ");
                    let _ = stderr.write(why.as_bytes());
                    let _ = stderr.write(b".\n");
                }
                InputError::NoInputsProvided => {
                    let _ = stderr.write(b"no input was provided after separator.\n");
                }
                InputError::NotEnoughInputs => {
                    let _ = stderr.write(b"not enough inputs were provided.\n");
                }
            }
            let _ = stderr.write(b"Example Usage: permutate 1 2 3 ::: 4 5 6 ::: 1 2 3\n");
            exit(1);
        }
    }
}

fn permutate<'a, P, LW>(stdout: &mut StdoutLock, permutator: &'a mut P)
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

    // Each permutation will check to see if the max number of permutations per
    // buffer has been allocated and prints it to standard output if true.
    let mut counter = 1;
    while let Some(current_output) = permutator.next_with_buffer(&mut current_output) {
        if counter == permutations_per_buffer {
            buffer.write_and_clear(stdout);
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

fn permutate_without_delims<'a, P, LW>(stdout: &mut StdoutLock, permutator: &'a mut P)
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

    // Each permutation will check to see if the max number of permutations per
    // buffer has been allocated and prints it to standard output if true.
    let mut counter = 1;
    while let Some(current_output) = permutator.next_with_buffer(&mut current_output) {
        let mut permutation = current_output.iter();
        if counter == permutations_per_buffer {
            buffer.write_and_clear(stdout);
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
