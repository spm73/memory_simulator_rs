use crate::process::Process;

pub struct Partition<'a> {
    initial_adress: u32,
    size: u32,
    process: Option<&'a mut Process>
}

impl<'a> Partition<'a> {
    fn new(initial_adress: u32, size: u32, process: Option<&'a mut Process>) -> Self {
        Self {
            initial_adress: initial_adress,
            size: size,
            process: process
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

    pub fn merge(&mut self, other: &Self) {
        // add error handling
        if self.process.is_none() {
            self.size += other.size;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_free1() {
        let partition = Partition::new(0, 1, None);
        assert_eq!(partition.is_free(), true);
    }

    #[test]
    fn is_free2() {
        let mut process = Process::from("1 0 100 50");
        let partition = Partition::new(0, 1, Some(&mut process));

        assert_eq!(partition.is_free(), false);
    }
}