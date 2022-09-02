pub mod port;
pub mod spi;
pub mod stdio;
pub mod cpuint;
pub mod tcb;

pub trait InterruptSource {
    fn interrupt(&self, index: usize) -> bool {
        // This function should return the bitwise and of the 
        // corresponding intflag and intctrl registers of the peripheral
        false
    }
}

pub trait Clocked {
    fn tick(&mut self, time: usize) {

    }
}

