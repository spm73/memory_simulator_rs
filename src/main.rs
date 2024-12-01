use std::env::args;
use std::fs::{ exists, remove_file};
use iced::{application, Element, Result, Task};
use iced::widget::{stack, column, row, button, text};
use memory::{Algorithm, Memory, OUTPUT_FILE_NAME};

mod process;
mod memory;
mod partition;

fn main() -> Result {
    delete_output_file();
    let arguments: Vec<String> = args().collect();
    let input_file_name = process_arguments(&arguments);
    let mem = Memory::new(&input_file_name).expect("Could not open input file");
    application("Memory Simulator", update, view).run_with(|| {(mem, Task::none())})
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

fn delete_output_file() {
    let output_file_exists = exists(OUTPUT_FILE_NAME)
                                    .expect("Something went wrong trying to search for past results");
    if output_file_exists {
        remove_file(OUTPUT_FILE_NAME).expect(format!("Could not delete {}", OUTPUT_FILE_NAME).as_str());
    }
}


fn view(mem: &Memory) -> Element<Algorithm> {
    let stack_elements = create_stack(mem);
    
    row![
        column![
            button("Next: Best Fit").on_press(Algorithm::BestFit),
            button("Next: Worst Fit").on_press(Algorithm::WorstFit)
        ],
        column![ stack(stack_elements) ]
    ].into()
}

fn create_stack(mem: &Memory) -> Vec<Element<Algorithm>> {
    let mut stack_elements: Vec<Element<Algorithm>> = Vec::new();
    for partition in &mem.get_partitions() {
        stack_elements.push(
            row![
                text(format!("{partition}"))
            ].into()
        )
    }

    stack_elements
}

fn update(mem: &mut Memory, algorithm: Algorithm) {
    mem.update(algorithm);
}