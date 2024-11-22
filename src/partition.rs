use core::fmt;

use crate::process::Process;

pub struct Partition {
    initial_adress: u32,
    size: u32,
    process: Option<Process>
}

impl fmt::Display for Partition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_free() {
            write!(f, "[{} Hole {}]", self.initial_adress, self.size)
        } else {
            write!(f, "[{} {} {}]", self.initial_adress, self.process.unwrap(), self.size)
        }
    }
}

impl Partition {
    pub fn divide(self, process: Process) -> (Self, Self) {
        let memory_required = process.get_memory_required();
        (Self {
            initial_adress: self.initial_adress,
            size: memory_required,
            process: Some(process)
        }, Self {
            initial_adress: self.initial_adress + memory_required + 1,
            size: self.size - memory_required,
            process: None
        })
    }

    pub fn new_empty(initial_adress: u32, size: u32) -> Self {
        Self {
            initial_adress: initial_adress,
            size: size,
            process: None
        }
    }

    pub fn update(&mut self) {
        match &mut self.process {
            Some(p) => {
                p.update();
                if p.has_ended() {
                    self.process = None;
                }
            },
            None => ()
        }
    }

    pub fn is_free(&self) -> bool {
        match &self.process {
            Some(_) => false,
            None => true
        }
    }

    pub fn merge(&mut self, other: Self) {
        // add error handling
        if self.process.is_none() {
            self.size += other.size;
        }
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}