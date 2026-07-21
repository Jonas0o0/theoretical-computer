use crate::hardware::utils::Byte;

pub struct Register {
    value: Byte,
}

impl Register {
    pub fn new() -> Self {
        Register {
            value: (false, false, false, false, false, false, false, false),
        }
    }

    pub fn clock_tick(&mut self, data_in: Byte, load: bool, reset: bool) {
        if reset {
            self.value = (false, false, false, false, false, false, false, false);
        } else if load {
            self.value = data_in;
        }
    }

    pub fn read_output(&self) -> Byte {
        self.value
    }
}