use std::env::args;
use std::fs::{ exists, remove_file };

use crate::memory::OUTPUT_FILE_NAME;

pub fn process_arguments() -> String {
    let arguments = args().collect::<Vec<String>>();
    if arguments.len() != 2 {
        panic!("This program should receive 1 argument telling the input file name");
    }

    let file_exists = exists(arguments[1].clone()).expect("File cannot be accessed");
    if !file_exists {
        panic!("Input file name does not exists");
    }
    
    arguments[1].clone()
}

pub fn delete_output_file() {
    let output_file_exists = exists(OUTPUT_FILE_NAME)
                                    .expect("Something went wrong trying to search for past results");
    if output_file_exists {
        remove_file(OUTPUT_FILE_NAME).expect(format!("Could not delete {}", OUTPUT_FILE_NAME).as_str());
    }
}