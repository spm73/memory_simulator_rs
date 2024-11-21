use crate::process::Process;
use crate::partition::Partition;
use std::fs::read_to_string;
use std::error::Error;

const INITIAL_MEMORY: u32 = 2000;

pub struct Memory {
    size: u32,
    processes: Vec<Process>,
    partitions: Vec<Partition>, 
    runtime: u32
}

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
            result.processes.push(Process::from(line));
        }

        result.partitions.push(Partition::new_empty(0, result.size));

        Ok(result)
    }

    pub fn update(&mut self, algorithm: Algorithm) {
        match algorithm {
            Algorithm::BestFit => self.best_fit(),
            Algorithm::WorstFit => todo!()
        }

        for partition in &mut self.partitions {
            partition.update();
        }
        self.merge_partitions();
        self.runtime += 1;
    }

    fn get_best_fit_position(&self, process: &Process) -> Option<usize> {
        let mut index_best_fit_partition: Option<usize> = None;
        let mut size_best_fit_partition = u32::MAX;
        for (index, partition) in self.partitions.iter().enumerate() {
            if partition.is_free() && partition.get_size() < size_best_fit_partition && partition.get_size() >= process.get_memory_required() {
                index_best_fit_partition = Some(index);
                size_best_fit_partition = partition.get_size();
            }
        }
        index_best_fit_partition
    }

    fn best_fit(&mut self) {
        for process in &self.processes {
            if self.runtime >= process.get_arrival_time() {
                match self.get_best_fit_position(process) {
                    None => (),
                    Some(index) => {
                        let partition = self.partitions.remove(index);
                        let (partition1, partition2) = partition.divide(process.clone());
                        self.partitions.insert(index, partition1);
                        self.partitions.insert(index + 1, partition2);
                    }
                }
            }
        }
    }

    fn worst_fit(&mut self) {
        todo!()
    }

    fn merge_partitions(&mut self) {
        let mut i = 0;
        while i < self.partitions.len() {
            if self.partitions[i].is_free() {
                let j = i + 1;
                while self.partitions[j].is_free() {
                    let p = self.partitions.remove(j);
                    self.partitions[i].merge(p);
                }
            }
            i += 1;
        }
    }
}

fn contiguous_indexes(index1: usize, index2: usize) -> bool {
    index1 + 1 == index2 || index1 - 1 == index2
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
                assert_eq!(mem.size, INITIAL_MEMORY);
                assert_eq!(mem.runtime, 0);
                assert_eq!(mem.partitions.len(), 0);
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
            Err(_) => ()
        }
    }
}