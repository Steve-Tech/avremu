use std::rc::Rc;
use std::cell::RefCell;

use super::Hardware;
use crate::nets::{PinState, Net, NetState};


#[derive(Debug)]
#[derive(PartialEq)]
enum SinkPwmState {
    High, 
    Low,
    Undefined
}

pub struct SinkPwm {
    name: String,
    pin: Rc<RefCell<PinState>>,
    net: Rc<RefCell<Net>>,
    state: SinkPwmState,
    t_last: usize,
    t_rise_last: usize,
    t_fall_last: usize,
    cycle_valid: bool,
    is_dc: bool
}

impl SinkPwm {
    pub fn new(name: String, net: Rc<RefCell<Net>>, pinstate: PinState) -> Self {
        let mut sink = SinkPwm { 
            name,
            pin: Rc::new(RefCell::new(pinstate)),
            net,
            state: SinkPwmState::Undefined,
            t_last: 0,
            t_rise_last: 0,
            t_fall_last: 0,
            cycle_valid: false,
            is_dc: true
        };
        sink.net.borrow_mut().connect(Rc::downgrade(&sink.pin));
        sink.update(0);
        sink
    }
}

impl Hardware for SinkPwm {
    fn update(&mut self, time: usize) {
        let new_state: SinkPwmState;
        match self.net.borrow().state {
            NetState::High => new_state = SinkPwmState::High,
            NetState::Low => new_state = SinkPwmState::Low,
            _ => new_state = SinkPwmState::Undefined
        }
        if !self.state.eq(&new_state) {
            self.t_last = time;
            self.is_dc = false;
        } else {
            if time - self.t_last > 166667 {
                // Greater than 50ms elapsed, assume DC
                if !self.is_dc {
                    self.is_dc = true;
                    match self.state {
                        SinkPwmState::Low => println!("[@{:08X}] PWM|{}: {:.0} Hz, {:.1} % duty cycle", time, self.name, 0.0, 0.0),
                        SinkPwmState::High => println!("[@{:08X}] PWM|{}: {:.0} Hz, {:.1} % duty cycle", time, self.name, 0.0, 100.0),
                        SinkPwmState::Undefined => println!("[@{:08X}] PWM|{}: Undefined", time, self.name),
                    }
                }
            }
        }
        match self.state {
            SinkPwmState::Low => {
                if new_state.eq(&SinkPwmState::High) {
                    let diff = time - self.t_rise_last;
                    let ppw = self.t_fall_last - self.t_rise_last;
                    let f = 3333333.333/f64::from(diff as i32);
                    let duty = 100.0*f64::from(ppw as i32)/f64::from(diff as i32);
                    if self.cycle_valid & (f > 20.0) & (f < 20000.0) {
                        println!("[@{:08X}] PWM|{}: {:.0} Hz, {:.1} % duty cycle", time, self.name, f, duty);
                    }
                    self.t_rise_last = time;
                    if !self.cycle_valid {
                        self.cycle_valid = true;
                    }
                }
                
            },
            SinkPwmState::High => {
                if new_state.eq(&SinkPwmState::Low) {
                    self.t_fall_last = time
                }
            },
            _ => {self.cycle_valid = false}
        }
        self.state = new_state;
    }
}