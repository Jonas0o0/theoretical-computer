use crate::hardware::utils::Byte;

pub struct Register {
    value: Byte,
}

impl Register {
    pub fn new() -> Self {
        Register {
            value: Byte(false, false, false, false, false, false, false, false),
        }
    }

    pub fn clock_tick(&mut self, data_in: Byte, load: bool, reset: bool) {
        if reset {
            self.value = Byte(false, false, false, false, false, false, false, false);
        } else if load {
            self.value = data_in;
        }
    }

    pub fn read_output(&self) -> Byte {
        self.value
    }
}

pub struct Ram {
    memory: [Byte; 256],
}

impl Ram {
    pub fn new() -> Self {
        Ram{
            memory: [Byte(false, false, false, false, false, false, false, false); 256],
        }
    }

    pub fn read_output(&self, address: Byte) -> Byte {
        self.memory[usize::from(address)]
    }

    pub fn clock_tick(&mut self, address: Byte, data_in: Byte, load: bool) {
        if load {
            self.memory[usize::from(address)] = data_in;
        }
    }
}