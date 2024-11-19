use std::convert::From;

pub struct Process {
    id: u32,
    arrival_time: u32,
    memory_required: u32,
    runtime: u32
}

impl From<&str> for Process {
    fn from(value: &str) -> Self {
        let mut buff = value.split_whitespace();
        Self {
            id: buff.next().unwrap().parse::<u32>().unwrap(),
            arrival_time: buff.next().unwrap().parse::<u32>().unwrap(),
            memory_required: buff.next().unwrap().parse::<u32>().unwrap(),
            runtime: buff.next().unwrap().parse::<u32>().unwrap()
        }
    }
}

impl Process {
    pub fn update(&mut self) {
        self.runtime -= 1;
    }

    pub fn has_ended(&self) -> bool {
        self.runtime <= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_impl() {
        let input = "1 0 100 50";
        let result = Process::from(input);
    
        assert_eq!(result.id, 1);
        assert_eq!(result.arrival_time, 0);
        assert_eq!(result.memory_required, 100);
        assert_eq!(result.runtime, 50);
    }
}