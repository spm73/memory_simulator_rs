use eframe::{run_native, NativeOptions};
use graphic_app::MyEguiApp;

use std::env::args;
use std::fs::{ exists, remove_file };

use memory::{Algorithm, Memory, OUTPUT_FILE_NAME};

mod process;
mod memory;
mod partition;
mod graphic_app;

fn main() -> eframe::Result {
    delete_output_file();
    let input_file_name = process_arguments();
    let mut mem = Memory::new(&input_file_name).expect("Smth went wrong");
    run_native(
        "MyApp",
        NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc))))
    )
}


fn process_arguments() -> String {
    let arguments = args().collect::<Vec<String>>();
    if arguments.len() != 2 {
        panic!("This program should receive 1 argument telling the input file name");
    }

    let file_exists = exists(arguments[1].clone()).expect("File cannot be accessed");
    if file_exists {
        return arguments[1].clone();
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