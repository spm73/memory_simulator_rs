use core::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use crate::process::Process;

#[derive(Debug, Clone)]
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
            write!(f, "[{} {} {}]", self.initial_adress, self.process.clone().unwrap().borrow().clone(), self.size)
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
            initial_adress: self.initial_adress + memory_required,
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
        if self.is_free() && other.is_free() {
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

    #[test]
    fn test_divide() {
        let initial_partition = Partition::new_empty(0, 100);
        let process = Rc::new(RefCell::new(Process::from("1 0 100 2")));
        let result = initial_partition.divide(&process);

        assert_eq!(result.0.initial_adress, 0);
        assert_eq!(result.0.size, 100);
        assert_eq!(result.0.is_free(), false);
        assert_eq!(result.1.is_none(), true);
    }

    #[test]
    fn test_divide2() {
        let initial_partition = Partition::new_empty(0, 200);
        let process = Rc::new(RefCell::new(Process::from("1 0 100 2")));
        let result = initial_partition.divide(&process);

        assert_eq!(result.0.initial_adress, 0);
        assert_eq!(result.0.size, 100);
        assert_eq!(result.0.is_free(), false);
        assert_eq!(result.1.as_ref().unwrap().initial_adress, 100);
        assert_eq!(result.1.as_ref().unwrap().size, 100);
        assert_eq!(result.1.unwrap().is_free(), true);
    }

    #[test]
    fn test_representation() {
        let initial_partition = Partition::new_empty(0, 200);
        let process = Rc::new(RefCell::new(Process::from("1 0 100 2")));
        let result = initial_partition.divide(&process);

        let p0 = format!("{}", result.0);
        let p1 = format!("{}", result.1.unwrap());

        assert_eq!("[0 P1 100]", p0);
        assert_eq!("[100 Hole 100]", p1);
    }

    #[test]
    fn test_merge() {
        let mut p1 = Partition::new_empty(0, 100);
        let p2 = Partition::new_empty(101, 100);

        p1.merge(p2);
        assert_eq!(p1.initial_adress, 0);
        assert_eq!(p1.size, 200);
    }

    #[test]
    fn test_update() {
        let initial_partition = Partition::new_empty(0, 200);
        let process = Rc::new(RefCell::new(Process::from("1 0 100 2")));
        let mut result = initial_partition.divide(&process);

        result.0.update();
        assert_eq!(result.0.process.clone().unwrap().borrow().has_ended(), false);
        
        result.0.update();
        assert_eq!(result.0.is_free(), true);
    }
}