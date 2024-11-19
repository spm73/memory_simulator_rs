use crate::process::Process;

pub struct Partition<'a> {
    initial_adress: u32,
    size: u32,
    free: bool,
    process: Option<&'a mut Process>
}

impl<'a> Partition<'a> {
    fn new(initial_adress: u32, size: u32, process: Option<&'a mut Process>) -> Self {
        let mut result = Self {
            initial_adress: initial_adress,
            size: size,
            free: false,
            process: process
        };
        
        match result.process {
            Some(_) => result.free = false,
            None => result.free = true
        };

        result
    }

    pub fn update(&mut self) {
        match &mut self.process {
            Some(p) => {
                p.update();
                if p.has_ended() {
                    self.process = None;
                    self.free = true;
                }
            },
            None => ()
        }
    }
}