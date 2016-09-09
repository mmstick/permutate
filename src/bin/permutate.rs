extern crate permutate;
use permutate::Permutator;

use std::env::args;
use std::fs;
use std::io::{self, BufRead, BufReader, StdoutLock, Write};
use std::process::exit;

mod man;

#[cfg(not(unix))]
mod platform {
    pub const BUFFER_SIZE: usize = 16 * 1024; // Windows only supports 16K buffers.
}

#[cfg(unix)]
mod platform {
    pub const BUFFER_SIZE: usize = 64 * 1024; // 4.75% performance boost over 16K buffers
}

use platform::*;

fn main() {
    // First, the program should grab a handle to stdout and stderr and lock them.
    let stdout = io::stdout();
    let stderr = io::stderr();

    // Locking the buffers will improve performance greatly due to not needing
    // to worry about repeatedly locking and unlocking them throughout the program.
    let mut stdout = stdout.lock();
    let mut stderr = stderr.lock();

    let (input, benchmark, interpret_files, no_delimiters) = parse_options(&mut stdout);

    let mut list_vector = Vec::new();
    match parse_arguments(&mut list_vector, &input.join(" "), interpret_files) {
        Ok(_) => {
            // Convert the Vec<Vec<String>> into a Vec<Vec<&str>>
            let tmp: Vec<Vec<&str>> = list_vector.iter()
                .map(|list| list.iter().map(AsRef::as_ref).collect::<Vec<&str>>())
                .collect();

            // Convert the Vec<Vec<&str>> into a Vec<&[&str]>
            let list_array: Vec<&[&str]> = tmp.iter().map(AsRef::as_ref).collect();

            // Create a `Permutator` with the &[&[&str]] as the input.
            let mut permutator = Permutator::new(&list_array[..]);

            if benchmark {
                let _ = permutator.count();
            } else {
                // Manually buffering standard output speeds up output by 90% and
                // reduces the required CPU cycles spent performing syscalls by 76%.
                if no_delimiters {
                    permutate_without_delims(&mut stdout, &mut permutator);
                } else {
                    permutate(&mut stdout, &mut permutator);
                }
            }
        },
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
                },
                InputError::NotEnoughInputs  => {
                    let _ = stderr.write(b"not enough inputs were provided.\n");
                },
            }
            let _ = stderr.write(b"Example Usage: permutate 1 2 3 ::: 4 5 6 ::: 1 2 3\n");
            exit(1);
        }
    }
}

fn buffer_value(buffer: &mut [u8; BUFFER_SIZE], current_size: usize, value: &str) -> usize {
    let capacity = value.len();
    buffer[current_size..current_size + capacity]
        .clone_from_slice(value.as_bytes());
    current_size + capacity
}

fn permutate(stdout: &mut StdoutLock, permutator: &mut Permutator<str>) {
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut current_size = 0;
    // This first run through will count the number of bytes that will be
    // required to print each permutation to standard output.
    {
        let current_permutation = permutator.next().unwrap();
        let mut current_permutation = current_permutation.iter();

        // The first element will print a space after the element.
        current_size = buffer_value(&mut buffer, current_size,
            current_permutation.next().unwrap());
        buffer[current_size] = b' ';
        current_size += 1;

        // There will always be at least two elements in a permutation.
        current_size = buffer_value(&mut buffer, current_size,
            current_permutation.next().unwrap());

        for element in current_permutation {
            buffer[current_size] = b' ';
            current_size += 1;
            current_size = buffer_value(&mut buffer, current_size, element);
        }
    }

    // Using the number of bytes of the first iteration, we can calculate
    // how many iterations that we can safely fit into our buffer.
    buffer[current_size] = b'\n';
    current_size += 1;

    // Using the number of bytes of the first iteration, we can calculate
    // how many iterations that we can safely fit into our buffer.
    let permutations_per_buffer = BUFFER_SIZE / current_size;

    // Each permutation will check to see if the max number of permutations per
    // buffer has been allocated and prints it to standard output if true.
    let mut counter = 1;
    for permutation in permutator {
        if counter == permutations_per_buffer {
            let _ = stdout.write_all(&buffer[..]);
            buffer = [0; BUFFER_SIZE];
            current_size = 0;
            counter = 0;
        }

        let mut current_permutation = permutation.iter();

        // The first element will print a space after the element.
        current_size = buffer_value(&mut buffer, current_size,
            current_permutation.next().unwrap());
        buffer[current_size] = b' ';
        current_size += 1;

        // There will always be at least two elements in a permutation.
        current_size = buffer_value(&mut buffer, current_size,
            current_permutation.next().unwrap());

        for element in current_permutation {
            buffer[current_size] = b' ';
            current_size += 1;
            current_size = buffer_value(&mut buffer, current_size, element);
        }
        buffer[current_size] = b'\n';
        current_size += 1;
        counter += 1;
    }

    // Print the remaining buffer to standard output.
    let _ = stdout.write_all(&buffer[..]);
}

fn permutate_without_delims(stdout: &mut StdoutLock, permutator: &mut Permutator<str>) {
    // This first run through will count the number of bytes that will be
    // required to print each permutation to standard output.
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut current_size = 0;
    {
        // There will always be at least two elements in a permutation.
        let permutation     = permutator.next().unwrap();
        let mut permutation = permutation.iter();
        current_size = buffer_value(&mut buffer, current_size, permutation.next().unwrap());
        current_size = buffer_value(&mut buffer, current_size, permutation.next().unwrap());
        for element in permutation {
            current_size = buffer_value(&mut buffer, current_size, element);
        }
    }

    // Append a newline after each permutation to print them on separate lines
    buffer[current_size] = b'\n';
    current_size += 1;

    // Using the number of bytes of the first iteration, we can calculate
    // how many iterations that we can safely fit into our buffer.
    let permutations_per_buffer = BUFFER_SIZE / current_size;

    // Each permutation will check to see if the max number of permutations per
    // buffer has been allocated and prints it to standard output if true.
    let mut counter = 1;
    for permutation in permutator {
        let mut permutation = permutation.iter();
        if counter == permutations_per_buffer {
            let _ = stdout.write_all(&buffer[..]);
            buffer = [0; BUFFER_SIZE];
            current_size = 0;
            counter = 0;
        }

        // There will always be at least two elements in a permutation.
        current_size = buffer_value(&mut buffer, current_size, permutation.next().unwrap());
        current_size = buffer_value(&mut buffer, current_size, permutation.next().unwrap());
        for element in permutation {
            current_size = buffer_value(&mut buffer, current_size, element);
        }

        buffer[current_size] = b'\n';
        current_size += 1;

        counter += 1;
    }

    // Print the remaining buffer to standard output.
    let _ = stdout.write_all(&buffer[..]);
}

#[derive(Debug)]
enum InputError {
    FileError(String, String),
    NoInputsProvided,
    NotEnoughInputs
}


/// Scans input arguments for flags that control the behaviour of the program.
fn parse_options(stdout: &mut StdoutLock) -> (Vec<String>, bool, bool, bool) {
    let mut input = Vec::new();
    let (mut benchmark, mut interpret_files, mut no_delimiters) = (false, false, false);
    for argument in args().skip(1) {
        match argument.as_str() {
            "-b" | "--benchmark" => benchmark = true,
            "-f" | "--files" => interpret_files = true,
            "-h" | "--help" => {
                let _ = stdout.write(man::MANPAGE.as_bytes());
                exit(0);
            },
            "-n" | "--no-delimiters" => no_delimiters = true,
            _ => input.push(argument)
        }
    }
    (input, benchmark, interpret_files, no_delimiters)
}

/// This is effectively a command-line interpreter designed specifically for this program.
fn parse_arguments(list_collection: &mut Vec<Vec<String>>, input: &str, interpret_files: bool)
    -> Result<(), InputError>
{
    let mut add_to_previous_list = false;
    let mut backslash            = false;
    let mut double_quote         = false;
    let mut single_quote         = false;
    let mut match_set            = false;
    let mut interpret_files      = interpret_files;
    let mut matches              = 0;
    let mut current_list         = Vec::new();
    let mut current_argument     = String::new();

    for character in input.chars() {
        if match_set {
            match character {
                '+' => add_to_previous_list = true,
                ' ' => {
                    if matches == 3 {
                        if add_to_previous_list {
                            add_to_previous_list = false;
                        } else {
                            if current_list.is_empty() {
                                return Err(InputError::NoInputsProvided);
                            } else {
                                list_collection.push(current_list.clone());
                                current_list.clear();
                            }
                        }
                        interpret_files = false;
                    } else if matches == 4 {
                        if add_to_previous_list {
                            add_to_previous_list = false;
                        } else {
                            if current_list.is_empty() {
                                return Err(InputError::NoInputsProvided);
                            } else {
                                list_collection.push(current_list.clone());
                                current_list.clear();
                            }
                        }
                        interpret_files = true;
                    } else {
                        for _ in 0..matches { current_argument.push(':'); }
                        current_list.push(current_argument.clone());
                        current_argument.clear();
                    }
                    match_set = false;
                    matches = 0;
                } ,
                ':' if !add_to_previous_list => matches += 1,
                _ => {
                    for _ in 0..matches { current_argument.push(':'); }
                    current_argument.push(character);
                    match_set = false;
                    matches = 0;
                },
            }
        } else if backslash {
            match character {
                '\\' | '\'' | ' ' | '\"' => current_argument.push(character),
                _    => {
                    current_argument.push('\\');
                    current_argument.push(' ');
                },
            }
            backslash = false;
        } else if single_quote {
            match character {
                '\\' => backslash = true,
                '\'' => single_quote = false,
                _    => current_argument.push(character)
            }
        } else if double_quote {
            match character {
                '\\' => backslash = true,
                '\"' => double_quote = false,
                _    => current_argument.push(character)
            }
        } else {
            match character {
                ' ' => {
                    if !current_argument.is_empty() {
                        if interpret_files {
                            for argument in try!(file_parse(&current_argument)) {
                                current_list.push(argument);
                            }
                        } else {
                            current_list.push(current_argument.clone());
                        }
                        current_argument.clear();
                    }
                },
                '\\' => backslash = true,
                '\'' => single_quote = true,
                '\"' => double_quote = true,
                ':' => {
                    match_set = true;
                    matches = 1;
                },
                _ => current_argument.push(character)
            }
        }
    }

    if !current_argument.is_empty() {
        if interpret_files {
            for argument in try!(file_parse(&current_argument)) {
                current_list.push(argument);
            }
        } else {
            current_list.push(current_argument);
        }
    }

    if !current_list.is_empty() {
        list_collection.push(current_list);
    }

    if list_collection.len() == 0 || (list_collection.len() == 1 && list_collection[0].len() == 1) {
        return Err(InputError::NotEnoughInputs)
    } else {
        Ok(())
    }
}

/// Attempts to open an input argument and adds each line to the `inputs` list.
fn file_parse(path: &str) -> Result<Vec<String>, InputError> {
    let mut inputs = Vec::new();
    let file = try!(fs::File::open(path)
        .map_err(|err| InputError::FileError(path.to_owned(), err.to_string())));
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line { inputs.push(line); }
    }
    Ok(inputs)
}

#[cfg(test)]
mod test {
    use super::parse_arguments;

    #[test]
    fn test_parse_arguments() {
        let mut output = Vec::new();
        let inputs = "A B ::: \"C D\" \\\"EF\\\" ::: five:six seven\\ eight";
        let expected = vec![
            vec!["A".to_owned(), "B".to_owned()],
            vec!["C D".to_owned(), "\"EF\"".to_owned()],
            vec!["five:six".to_owned(), "seven eight".to_owned()]
        ];
        let _ = parse_arguments(&mut output, inputs, false);
        assert_eq!(output, expected);
    }
}
