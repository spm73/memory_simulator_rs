use core::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use crate::process::Process;

pub struct Partition {
    initial_adress: u32,
    size: u32,
    process: Option<Rc<RefCell<Process>>>
}

impl fmt::Display for Partition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_free() {
            write!(f, "[{} Hole {}]", self.initial_adress, self.size)
        } else {
            write!(f, "[{} {} {}]", self.initial_adress, self.process.as_ref().unwrap().borrow().clone(), self.size)
        }
    }
}

impl Partition {
    pub fn divide(self, process: &Rc<RefCell<Process>>) -> (Self, Option<Self>) {
        let memory_required = process.borrow().get_memory_required();
        process.borrow_mut().assign();
        if memory_required == self.size {
            return (Self {
                initial_adress: self.initial_adress,
                size: memory_required,
                process: Some(Rc::clone(process))
            }, None);
        }

        (Self {
            initial_adress: self.initial_adress,
            size: memory_required,
            process: Some(Rc::clone(process))
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
        if let Some(process) = &self.process {
            process.borrow_mut().update();
            if process.borrow().has_ended() {
                self.process = None;
            }
        }
    }

    pub fn is_free(&self) -> bool {
        self.process.is_none()
    }

    pub fn merge(&mut self, other: Self) {
        // add error handling
        if self.is_free() {
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