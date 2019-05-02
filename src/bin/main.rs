extern crate permutate;

use std::io::{self, Write};
use std::process::exit;

use permutate::bin;
use permutate::bin::arguments::InputError;

fn main() {
    let (input, benchmark, interpret_files, no_delimiters) = bin::arguments::parse_options();

    let mut list_vector = Vec::new();
    match bin::arguments::parse_arguments(&mut list_vector, &input.join(" "), interpret_files) {
        Ok(_) => bin::prepare(list_vector, benchmark, no_delimiters),
        Err(why) => {
            // Locking the buffers will improve performance greatly due to not needing
            // to worry about repeatedly locking and unlocking them throughout the program.
            let stderr = io::stderr();
            let mut stderr = stderr.lock();

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
