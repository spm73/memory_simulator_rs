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

    fn best_fit(&mut self) {
        let mut processes_to_remove: Vec<usize> = Vec::new();
        for (index, process) in self.processes.iter_mut().enumerate() {
            if self.runtime >= process.get_arrival_time() {
                let mut new_memory_arrangement: Vec<Partition> = Vec::new();
                for partition in &self.partitions {
                    let memory_required = process.get_memory_required();
                    if partition.is_free() && partition.get_size() >= memory_required {
                        let (partition1, partition2) = Partition::divide(partition.clone(), process.clone());
                        new_memory_arrangement.push(partition1);
                        new_memory_arrangement.push(partition2);
                        processes_to_remove.push(index);
                    } else {
                        new_memory_arrangement.push(partition.clone());
                    } 
                }
                self.partitions = new_memory_arrangement;
            }
        }
        
        for index in processes_to_remove {
            self.processes.remove(index);
        }
    }

    fn get_free_partitions_index(&self) -> Vec<usize> {
        let mut free_partitions_index: Vec<usize> = Vec::new();
        for (index, partition) in self.partitions.iter().enumerate() {
            if partition.is_free() {
                free_partitions_index.push(index);
            }
        }

        free_partitions_index
    }

    fn merge_partitions(&mut self) {
        // let mut free_partitions_index: Vec<usize> = self.get_free_partitions_index();

        // let mut is_merged = false;

        // while !is_merged {
        //     let mut skip_next_iteration = false;
        //     for i in 0..free_partitions_index.len() - 1 {
        //         if !skip_next_iteration && contiguous_indexes(free_partitions_index[i], free_partitions_index[i + 1]) {
        //             let (left, right)  = self.partitions.split_at_mut(free_partitions_index[i + 1]);
        //             let partition1 = left.last_mut().unwrap();
        //             let partition2 = right.first().unwrap();
        //             partition1.merge(partition2);
        //             self.partitions.remove(free_partitions_index[i + 1]);
        //             skip_next_iteration = true;
        //         } else if skip_next_iteration {
        //             skip_next_iteration = false;
        //         }
        //     }

        //     let new_free_partitions_index = self.get_free_partitions_index();
        //     if free_partitions_index == new_free_partitions_index {
        //         is_merged = true;
        //     }

        //     free_partitions_index = new_free_partitions_index;
        //}

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