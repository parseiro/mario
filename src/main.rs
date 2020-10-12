// #![no_std]

// #![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![warn(anonymous_parameters)]
#![warn(box_pointers)]
//#![warn(missing_docs)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_results)]
// #![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![warn(clippy::cast_possible_truncation,clippy::cast_possible_wrap,
clippy::cast_precision_loss,clippy::cast_sign_loss,clippy::integer_arithmetic)]
#![warn(clippy::fallible_impl_from)]
#![warn(clippy::filter_map,clippy::filter_map_next)]
#![warn(clippy::if_not_else,clippy::nonminimal_bool,clippy::single_match_else)]
#![warn(clippy::int_plus_one)]
#![warn(clippy::similar_names)]
#![warn(clippy::mutex_integer)]
//#![warn(clippy::print_stdout,clippy::use_debug)]
#![warn(clippy::unwrap_used,clippy::map_unwrap_or)]
//#![warn(clippy::unwrap_in_result)]

extern crate alloc;

/*#[macro_use]
extern crate lazy_static;*/

// use alloc::sync::{Arc, Weak};
// use alloc::borrow::BorrowMut;
use alloc::fmt::{Display, Formatter, Result};
use alloc::fmt;
use core::cell::RefCell;

/*macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({

    })
}*/

// const FAN_OFF_STATE: FanOffState = FanOffState {};
// const FAN_LOW_STATE: FanLowState = FanLowState {};
// const FAN_MED_STATE: FanMedState = FanMedState {};
// const FAN_HIGH_STATE: FanHighState = FanHighState {};



fn main() {
    let fan_off_state = FanOffState {};

    let mut fan: Fan = Fan {
        //fan_off_state: FAN_OFF_STATE,
        fan_off_state: fan_off_state.clone(),
        // fan_low_state: FAN_LOW_STATE,
        fan_low_state: FanLowState {},
        // fan_med_state: FAN_MED_STATE,
        fan_med_state: FanMedState {},
        // fan_high_state: FAN_HIGH_STATE,
        fan_high_state: FanHighState {},
        current_state: RefCell::new(&fan_off_state),
        // current_pointer: fan_off_state as *mut FanState,
    };

    let state = fan.current_state.borrow().clone();

    println!("Estado atual: {}", state);

    state.handle_request(&mut fan);

    let state = fan.current_state.borrow().clone();

    println!("Estado atual: {}", state);

    state.handle_request(&mut fan);

    let state = fan.current_state.borrow().clone();

    println!("Estado atual: {}", state);

    state.handle_request(&mut fan);

    let state = fan.current_state.borrow().clone();

    println!("Estado atual: {}", state);

    state.handle_request(&mut fan);

    let state = fan.current_state.borrow().clone();

    println!("Estado atual: {}", state);

    state.handle_request(&mut fan);







    // arc_ventilador = arc.borrow_mut();


    // let crone = arc_ventilador.clone();
    // let state = FanOffState::new();
    // let arc_state = Arc::new(state);
    // arc_ventilador.fanOffState.set(Some(arc_state));
}

struct Fan<'a> {
    fan_off_state: FanOffState,
    fan_low_state: FanLowState,
    fan_med_state: FanMedState,
    fan_high_state: FanHighState,

    current_state: RefCell<&'a dyn FanState>,
    // current_pointer: *mut dyn FanState,
}

impl<'a> Fan<'a> {
    /*fn new() -> Fan {
        let fan_off_state = FanOffState::new();
        let fan_low_state = FanLowState::new();
        let fan_med_state = FanMedState::new();
        let fan_high_state = FanHighState::new();


        let current_state = RefCell::new(&fan_off_state);

        let fan = Fan {
            fan_off_state: fan_off_state,
            fan_low_state: fan_low_state,
            fan_med_state: fan_med_state,
            fan_high_state: fan_high_state,
            current_state: current_state,
        };

        // fan_off_state.set_fan(fan.clone());
        // fan_low_state.set_fan(fan.clone());
        // fan_med_state.set_fan(fan.clone());
        // fan_high_state.set_fan(fan.clone());

        fan
    }*/

    fn set_current_state(&self, state: &'a dyn FanState) {
        let _ = self.current_state.replace(state);
    }
}

trait FanState: Display {
    fn handle_request(&self, fan: &mut Fan);
    // fn set_fan(&self, a: Fan);
}

#[derive(Clone,Copy)]
struct FanOffState {
    // fan: RefCell<Option<Fan>>,
}

/*impl FanOffState {
    fn new() -> Self {
        FanOffState {
            // fan: RefCell::new(None),
        }
    }
}*/

impl Display for FanOffState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FanOffState")
    }
}

impl FanState for FanOffState {
    fn handle_request<'a>(&self, fan: &'a mut Fan) {
        println!("Turning fan off to low");

        let state = fan.fan_low_state.clone();

        fan.set_current_state(&state);
    }

/*    fn set_fan(&self, arc: Arc<Fan>) {
        let weak = Arc::downgrade(&arc);
        let _ = self.fan.replace(Some(weak));
    }*/
}

#[derive(Clone,Copy)]
struct FanLowState {
    // fan: RefCell<Option<Weak<Fan>>>,
}

impl FanLowState {
/*    fn new() -> Self {
        FanLowState {
            // fan: RefCell::new(None),
        }
    }*/
}

impl Display for FanLowState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FanLowState")
    }
}

impl FanState for FanLowState {
    fn handle_request(&self, fan: &mut Fan) {
        println!("Turning fan low to med");

        // fan.set_current_state(&FAN_MED_STATE);

    }

/*    fn set_fan(&self, arc: Arc<Fan>) {
        let weak = Arc::downgrade(&arc);
        let _ = self.fan.replace(Some(weak));
    }*/
}

#[derive(Clone,Copy)]
struct FanMedState {
    // fan: RefCell<Option<Fan>>,
}

impl FanMedState {
/*    fn new() -> Self {
        FanMedState {
            // fan: RefCell::new(None),
        }
    }*/
}

impl Display for FanMedState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FanMedState")
    }
}

impl FanState for FanMedState {
    fn handle_request(&self, fan: &mut Fan) {
        println!("Turning fan med to high");

        // fan.set_current_state(&FAN_HIGH_STATE);
    }

/*    fn set_fan(&self, arc: Arc<Fan>) {
        let weak = Arc::downgrade(&arc);
        let _ = self.fan.replace(Some(weak));
    }*/
}

#[derive(Clone,Copy)]
struct FanHighState {
    // fan: RefCell<Option<Weak<Fan>>>,
}

impl FanHighState {
/*    fn new() -> Self {
        FanHighState {
            // fan: RefCell::new(None),
        }
    }*/
}

impl Display for FanHighState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FanHighState")
    }
}

impl FanState for FanHighState {
    fn handle_request(&self, fan: &mut Fan) {
        println!("Turning fan high to off");

        fan.set_current_state(&fan.fan_off_state);
    }

/*    fn set_fan(&self, arc: Arc<Fan>) {
        let weak = Arc::downgrade(&arc);
        let _ = self.fan.replace(Some(weak));
    }*/
}

