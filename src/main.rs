use std::env::args;
use std::fs::exists;
use iced::{Result, application};
use memory::Memory;

mod process;
mod memory;
mod partition;

fn main() {
    let arguments: Vec<String> = args().collect();
    let input_file_name = process_arguments(&arguments);
    let mem = Memory::new(&input_file_name).expect("Could not open input file");
}


fn process_arguments(args: &Vec<String>) -> String {
    if args.len() != 2 {
        panic!("This program should receive 1 argument telling the input file name");
    }

    let file_exists = exists(args[1].clone()).expect("File cannot be accessed");
    if file_exists {
        return args[1].clone();
    }

    panic!("Input file name does not exists");
}