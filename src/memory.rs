use crate::process::Process;
use std::fs::read_to_string;
use std::error::Error;

const INITIAL_MEMORY: u32 = 2000;

pub struct Memory {
    max_capacity: u32,
    free_capacity: u32,
    initial_processes: Vec<Process>,
    processes_in_mem: Vec<Process>, // change to partitions in memory
    waiting_queue: Vec<Process>,
    runtime: u32
}

impl Memory {
    pub fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file_content = read_to_string(file_path)?;
        let mut result = Self {
            max_capacity: INITIAL_MEMORY,
            free_capacity: 0,
            initial_processes: Vec::new(),
            processes_in_mem: Vec::new(),
            waiting_queue: Vec::new(),
            runtime: 0
        };

        for line in file_content.lines() {
            result.initial_processes.push(Process::from(line));
        }

        Ok(result)
    }

    pub fn update(&mut self) {
        for process in self.processes_in_mem.iter_mut() {
            process.update();
            self.runtime += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; 
    
    #[test]
    fn test_new() {
        const FILE_NAME: &str = "input_prueba.txt";
        let result = Memory::new(FILE_NAME);
        match result {
            Ok(mem) => {
                assert_eq!(mem.max_capacity, INITIAL_MEMORY);
                assert_eq!(mem.free_capacity, 0);
                assert_eq!(mem.runtime, 0);
                assert_eq!(mem.processes_in_mem.len(), 0);
                assert_eq!(mem.waiting_queue.len(), 0);
                assert_eq!(mem.initial_processes.len(), 2);
            },
            Err(e) => panic!("An error ocurred, {}", e)
        }
    }

    #[test]
    fn test_new2() {
        const FILE_NAME: &str = "../input_prueba.txt";
        let result = Memory::new(FILE_NAME);
        match result {
            Ok(_) => panic!("File does not exist"),
            Err(_) => ()
        }
    }
}