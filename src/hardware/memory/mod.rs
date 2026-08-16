use crate::hardware::alu::{au16};
use crate::hardware::gates::{mux16};
use crate::hardware::utils::{Byte, U16};

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
        Ram {
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

pub struct Rom {
    memory: [Byte; 65_536],
}

impl Rom {
    pub fn new() -> Self {
        Rom {
            memory: [Byte(false, false, false, false, false, false, false, false); 65_536],
        }
    }

    pub fn read_output(&self, address: U16) -> Byte {
        self.memory[usize::from(address)]
    }

    pub fn write(&mut self, address: usize, data_in: Byte) {
        self.memory[address] = data_in;
    }
}

pub struct Register16 {
    value: U16,
}

impl Register16 {
    pub fn new() -> Self {
        Register16 {
            value: U16(
                false, false, false, false,
                false, false, false, false,
                false, false, false, false,
                false, false, false, false,
            ),
        }
    }

    pub fn clock_tick(&mut self, data_in: U16, load: bool, reset: bool) {
        if reset {
            self.value = U16(
                false, false, false, false,
                false, false, false, false,
                false, false, false, false,
                false, false, false, false,
            );
        } else if load {
            self.value = data_in;
        }
    }

    pub fn read_output(&self) -> U16 {
        self.value
    }
}

pub struct PC {
    register: Register16,
}

impl PC {
    pub fn new() -> Self {
        PC {
            register: Register16::new(),
        }
    }

    pub fn clock_tick(&mut self, jump_address: U16, load: bool, reset: bool) {
        let current_pc = self.register.read_output();
        let base_address = mux16(current_pc, jump_address, load);
        let one = U16(true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false);
        let zero = U16(false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false);
        let increment = mux16(one, zero, load);
        let next_pc = au16(base_address, increment, (false, false, false)).0;
        self.register.clock_tick(next_pc, true, reset);
    }

    pub fn read_output(&self) -> U16 {
        self.register.read_output()
    }
}

#[cfg(test)]
mod tests;