use eframe::{run_native, NativeOptions};

use graphic_app::MyEguiApp;
use memory::Memory;
use function_utils::*;

mod process;
mod memory;
mod partition;
mod graphic_app;
mod function_utils;

fn main() -> eframe::Result {
    delete_output_file();
    let input_file_name = process_arguments();
    let mem = Memory::new(&input_file_name).expect("Smth went wrong");
    run_native(
        "MyApp",
        NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc, mem))))
    )
}
