use crate::process::Process;
use crate::partition::Partition;
use std::fs::read_to_string;
use std::error::Error;

const INITIAL_MEMORY: u32 = 2000;

pub struct Memory<'a> {
    size: u32,
    processes: Vec<Process>,
    partitions: Vec<Partition<'a>>, 
    runtime: u32
}

impl<'a> Memory<'a> {
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

    pub fn update(&mut self) {
        for partition in self.partitions.iter_mut() {
            partition.update();
        }
        self.merge_partitions();
        self.runtime += 1;
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
        let mut free_partitions_index: Vec<usize> = self.get_free_partitions_index();

        let mut is_merged = false;

        while (!is_merged) {
            let mut skip_next_iteration = false;
            for i in 0..free_partitions_index.len() - 1 {
                if !skip_next_iteration && contiguous_indexes(free_partitions_index[i], free_partitions_index[i + 1]) {
                    let (left, right)  = self.partitions.split_at_mut(free_partitions_index[i + 1]);
                    let partition1 = left.last_mut().unwrap();
                    let partition2 = right.first().unwrap();
                    partition1.merge(partition2);
                    self.partitions.remove(free_partitions_index[i + 1]);
                    skip_next_iteration = true;
                } else if skip_next_iteration {
                    skip_next_iteration = false;
                }
            }

            let new_free_partitions_index = self.get_free_partitions_index();
            if free_partitions_index == new_free_partitions_index {
                is_merged = true;
            }

            free_partitions_index = new_free_partitions_index;
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