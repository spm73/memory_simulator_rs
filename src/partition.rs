use crate::process::Process;

#[derive(Clone, Copy)]
pub struct Partition {
    initial_adress: u32,
    size: u32,
    process: Option<Process>
}

impl Partition {
    pub fn divide(partition: Self, process: Process) -> (Self, Self) {
        let memory_required = process.get_memory_required();
        (Self {
            initial_adress: partition.initial_adress,
            size: memory_required,
            process: Some(process)
        }, Self {
            initial_adress: partition.initial_adress + memory_required + 1,
            size: partition.size - memory_required,
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