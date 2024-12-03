use std::env::args;
use std::fs::{ exists, remove_file };
use std::process::exit;
use iced::{application, Element, Result, Task, color, Point};
use iced::widget::{column, row, button, Space, Column, text, Canvas};

use memory::{Algorithm, Memory, OUTPUT_FILE_NAME};
use graphic_partitions::{PartitionRectangle, SIZE};

mod process;
mod memory;
mod partition;
mod graphic_partitions;

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
    if !mem.has_processes_waiting() {
        exit(0);
    }

    let stack_elements = create_stack(mem);
    column![
        row![
            button("Next: Best Fit").on_press(Algorithm::BestFit),
            Space::with_width(30),
            button("Next: Worst Fit").on_press(Algorithm::WorstFit)
        ],
        stack_elements
    ].into()
}

fn create_stack(mem: &Memory) -> Element<Algorithm> {
    let mut stack_elements = Column::new();
    let mut top_left = Point::new(100f32, 100f32);
    for partition in &mem.get_partitions() {
        stack_elements = stack_elements.push(
            Canvas::new(PartitionRectangle::new(top_left, format!("{partition}")))
        );
        top_left.y += SIZE.height;
    }

    stack_elements.into()
}

fn update(mem: &mut Memory, algorithm: Algorithm) {
    mem.update(algorithm);
}