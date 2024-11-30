use crate::process::Process;
use crate::partition::Partition;
use std::fs::{ read_to_string, File };
use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;

pub const INITIAL_MEMORY: u32 = 2000;
pub const OUTPUT_FILE_NAME: &str = "partitions.txt";

#[derive(Debug, Clone)]
pub struct Memory {
    size: u32,
    processes: Vec<Rc<RefCell<Process>>>,
    partitions: Vec<Partition>, 
    runtime: u32
}

#[derive(Clone, Debug)]
pub enum Algorithm {
    BestFit,
    WorstFit
}

impl Memory {
    pub fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file_content = read_to_string(file_path)?;
        let mut result = Self {
            size: INITIAL_MEMORY,
            processes: Vec::new(),
            partitions: Vec::new(),
            runtime: 0
        };

        for line in file_content.lines() {
            result.processes.push(Rc::new(RefCell::new(Process::from(line))));
        }

        result.partitions.push(Partition::new_empty(0, result.size));

        Ok(result)
    }
    
    pub fn has_processes_waiting(&self) -> bool {
        !self.processes.is_empty()
    }

    pub fn update(&mut self, algorithm: Algorithm) {
        self.runtime += 1;
        
        for partition in &mut self.partitions {
            partition.update();
        }
        self.merge_partitions();
        self.partition_assignment(algorithm);   
        
        let mut i: usize = 0;
        while i < self.processes.len() {
            if self.processes[i].borrow().has_ended() {
                self.processes.remove(i);
                continue; // remove shifts vector elements
            }
            i += 1;
        }
        if let Err(e) = self.write_file() {
            println!("Something went wrong while writing the output to file");
            println!("{}", e);
        }
    }
    
    fn write_file(&self) -> std::io::Result<()> {
        use std::io::Write;
        let mut file = File::options().append(true).create(true).open(OUTPUT_FILE_NAME)?;
        file.write(format!("{}", self.runtime).as_bytes())?;
        for partition in &self.partitions {
            file.write(format!(" {partition}").as_bytes())?;
        }
        file.write(" Return\n".as_bytes())?;
        Ok(())
    }

    fn get_partition_position(&self, process: &Process, algorithm: &Algorithm) -> Option<usize> {
        let mut index_partition: Option<usize> = None;
        let mut size_partition;
        let algorithm_condition: Box<dyn Fn(u32, u32) -> bool>;
        
        match algorithm {
            Algorithm::BestFit => {
                size_partition = u32::MAX;
                algorithm_condition = Box::new(|current_size_partition: u32, best_size_partition: u32| -> bool { current_size_partition < best_size_partition }); 
            },
            Algorithm::WorstFit => {
                size_partition = u32::MIN;
                algorithm_condition = Box::new(|current_size_partition: u32, worst_size_partition: u32| -> bool { current_size_partition > worst_size_partition });
            }
        }
        
        for (index, partition) in self.partitions.iter().enumerate() {
            if partition.is_free() && algorithm_condition(partition.get_size(), size_partition) && partition.get_size() >= process.get_memory_required() {
                index_partition = Some(index);
                size_partition = partition.get_size();
            }
        }
        
        index_partition
    }

    fn partition_assignment(&mut self, algorithm: Algorithm) {
        let processes = self.processes.clone();
        for (i, process) in processes.iter().enumerate() {
            if !process.borrow().is_assigned() && self.runtime >= process.borrow().get_arrival_time() {
                let possible_position = self.get_partition_position(&process.borrow(), &algorithm);
                if let Some(index) = possible_position {
                    let partition = self.partitions.remove(index);
                    let (partition1, partition2) = partition.divide(&mut self.processes[i]);
                    self.partitions.insert(index, partition1);
                    if let Some(p) = partition2 {
                        self.partitions.insert(index + 1, p);
                    }
                }
            }
        }
    }

    fn merge_partitions(&mut self) {
        let mut i = 0;
        while i < self.partitions.len() {
            if self.partitions[i].is_free() {
                let j = i + 1;
                while j < self.partitions.len() && self.partitions[j].is_free() {
                    let p = self.partitions.remove(j);
                    self.partitions[i].merge(p);
                }
            }
            i += 1;
        }
    }

    pub fn get_partitions(&self) -> Vec<Partition> {
        self.partitions.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;

    #[test]
    fn test_new() {
        const FILE_NAME: &str = "input_prueba.txt";
        let result = Memory::new(FILE_NAME);
        match result {
            Ok(mem) => {
                assert_eq!(mem.size, INITIAL_MEMORY);
                assert_eq!(mem.runtime, 0);
                assert_eq!(mem.partitions.len(), 1);
                assert_eq!(mem.processes.len(), 2);
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
            Err(e) => println!("Correct: {e}")
        }
    }

    #[test]
    fn test_update() {
        let result = Memory::new("input_test_update.txt");
        match result {
            Ok(mut mem) => {
                // dbg!(&mem);
                assert_eq!(1, mem.partitions.len());
                assert_eq!(3, mem.processes.len());

                mem.update(Algorithm::BestFit);
                // dbg!(&mem);
                assert_eq!(3, mem.partitions.len());
                assert_eq!(3, mem.processes.len());

                mem.update(Algorithm::BestFit);
                // dbg!(&mem);
                assert_eq!(3, mem.partitions.len());
                assert_eq!(2, mem.processes.len());

                mem.update(Algorithm::BestFit);
                // dbg!(&mem);
                assert_eq!(1, mem.partitions.len());
                assert_eq!(0, mem.processes.len());
            },
            Err(e) => panic!("Could not complete test {}", e)
        }
        remove_file(OUTPUT_FILE_NAME).expect("Could not delete output file");
    }

    #[test]
    fn test_output() {
        remove_file(OUTPUT_FILE_NAME).unwrap_or_else(|_| {});
        let possible_mem = Memory::new("input_test_update.txt");
        match possible_mem {
            Ok(mut mem) => {
                while mem.has_processes_waiting() {
                    mem.update(Algorithm::BestFit);
                }

                let content = read_to_string(OUTPUT_FILE_NAME).expect("Could not read content of output file");
                let mut lines = content.lines();
                assert_eq!("1 [0 P1 100] [100 P3 100] [200 Hole 1800] Return", lines.next().unwrap());
                assert_eq!("2 [0 P1 100] [100 P2 100] [200 Hole 1800] Return", lines.next().unwrap());
                assert_eq!("3 [0 Hole 2000] Return", lines.next().unwrap());
                
            },
            Err(e) => panic!("Could not complete test {}", e)
        }
        remove_file(OUTPUT_FILE_NAME).expect("Could not delete output file");
    }
}