use core::fmt;

use crate::process::Process;

pub struct Partition<'a> {
    initial_adress: u32,
    size: u32,
    process: Option<&'a mut Process>
}

impl<'a> fmt::Display for Partition<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_free() {
            write!(f, "[{} Hole {}]", self.initial_adress, self.size)
        } else {
            write!(f, "[{} {} {}]", self.initial_adress, self.process.as_ref().unwrap().clone(), self.size)
        }
    }
}

impl<'a> Partition<'a> {
    pub fn divide(self, process: &'a mut Process) -> (Self, Option<Self>) {
        let memory_required = process.get_memory_required();
        process.assign();
        if memory_required == self.size {
            return (Self {
                initial_adress: self.initial_adress,
                size: memory_required,
                process: Some(process)
            }, None);
        }

        (Self {
            initial_adress: self.initial_adress,
            size: memory_required,
            process: Some(process)
        }, Some(Self {
            initial_adress: self.initial_adress + memory_required + 1,
            size: self.size - memory_required,
            process: None
        }))
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
        self.process.is_none()
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