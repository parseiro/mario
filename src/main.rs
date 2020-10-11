// #![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![warn(anonymous_parameters)]
#![warn(box_pointers)]
//#![warn(missing_docs)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_results)]
#![warn(unused_qualifications)]
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

use std::sync::{Arc, Weak};
use std::cell::{RefCell};
use std::borrow::BorrowMut;
use std::fmt::{Display, Formatter};
use std::fmt;

fn main() {
    // let arc_ventilador;


    let mut fan = Arc::new(Fan::new());

    let state = fan.current_state.borrow().upgrade().expect("Ué");

    println!("Estado atual: {}", state);

    state.handle_request();

    let state = fan.current_state.borrow().upgrade().expect("Ué");

    println!("Estado atual: {}", state);

    state.handle_request();

    let state = fan.current_state.borrow().upgrade().expect("Ué");

    println!("Estado atual: {}", state);

    state.handle_request();

    let state = fan.current_state.borrow().upgrade().expect("Ué");

    println!("Estado atual: {}", state);

    state.handle_request();

    let state = fan.current_state.borrow().upgrade().expect("Ué");

    println!("Estado atual: {}", state);

    state.handle_request();







    // arc_ventilador = arc.borrow_mut();


    // let crone = arc_ventilador.clone();
    // let state = FanOffState::new();
    // let arc_state = Arc::new(state);
    // arc_ventilador.fanOffState.set(Some(arc_state));
}

struct Fan {
    fan_off_state: Arc<FanOffState>,
    fan_low_state: Arc<FanLowState>,
    fan_med_state: Arc<FanMedState>,
    fan_high_state: Arc<FanHighState>,

    current_state: RefCell<Weak<dyn FanState>>,
}

impl Fan {
    fn new() -> Arc<Fan> {
        let fan_off_state = Arc::from(FanOffState::new());
        let fan_low_state = Arc::from(FanLowState::new());
        let fan_med_state = Arc::from(FanMedState::new());
        let fan_high_state = Arc::from(FanHighState::new());


        let current_state = RefCell::new(Arc::downgrade(&fan_off_state));

        let fan = Fan {
            fan_off_state: fan_off_state.clone(),
            fan_low_state: fan_low_state.clone(),
            fan_med_state: fan_med_state.clone(),
            fan_high_state: fan_high_state.clone(),
            current_state: current_state.clone(),
        };

        let fan = Arc::from(fan);

        fan_off_state.set_fan(fan.clone());
        fan_low_state.set_fan(fan.clone());
        fan_med_state.set_fan(fan.clone());
        fan_high_state.set_fan(fan.clone());


        // let a: Option<&mut FanOffState> = Arc::get_mut(&mut fan_off_state);
        // let _b = a.expect("There must be something");

        // .set_fan(arc_fan.clone());

        fan
    }

    fn set_current_state(&self, state: Weak<dyn FanState>) {
        let _ = self.current_state.replace(state);
    }

    /*    fn getFanLowState(&self) -> Box<&dyn State> {
            //return Box::new(&self.fanLowState);
            return &self.fanOffState;
        }*/
}

trait FanState: Display {
    fn handle_request(&self);
    fn set_fan(&self, a: Arc<Fan>);
}


struct FanOffState {
    fan: RefCell<Option<Weak<Fan>>>,
}

impl FanOffState {
    fn new() -> Self {
        FanOffState {
            fan: RefCell::new(None),
        }
    }
}

impl Display for FanOffState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FanOffState")
    }
}

impl FanState for FanOffState {
    fn handle_request(&self) {
        println!("Turning fan off to low");
        let fan = self.fan
            .borrow()
            .as_ref()
            .expect("Ué")
            .upgrade()
            .expect("Ué2");

        let new_state = fan.fan_low_state.clone();
        let new_state: Weak<_> = Arc::downgrade(&new_state);

        fan.set_current_state(new_state);
    }

    fn set_fan(&self, arc: Arc<Fan>) {
        let weak = Arc::downgrade(&arc);
        let _ = self.fan.replace(Some(weak));
    }
}

struct FanLowState {
    fan: RefCell<Option<Weak<Fan>>>,
}

impl FanLowState {
    fn new() -> Self {
        FanLowState {
            fan: RefCell::new(None),
        }
    }
}

impl Display for FanLowState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FanLowState")
    }
}

impl FanState for FanLowState {
    fn handle_request(&self) {
        println!("Turning fan low to med");
        let fan = self.fan
            .borrow()
            .as_ref()
            .expect("Ué")
            .upgrade()
            .expect("Ué2");

        let new_state = fan.fan_med_state.clone();
        let new_state: Weak<_> = Arc::downgrade(&new_state);

        fan.set_current_state(new_state);

    }

    fn set_fan(&self, arc: Arc<Fan>) {
        let weak = Arc::downgrade(&arc);
        let _ = self.fan.replace(Some(weak));
    }
}

struct FanMedState {
    fan: RefCell<Option<Weak<Fan>>>,
}

impl FanMedState {
    fn new() -> Self {
        FanMedState {
            fan: RefCell::new(None),
        }
    }
}

impl Display for FanMedState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FanMedState")
    }
}

impl FanState for FanMedState {
    fn handle_request(&self) {
        println!("Turning fan med to high");
        let fan = self.fan
            .borrow()
            .as_ref()
            .expect("Ué")
            .upgrade()
            .expect("Ué2");

        let new_state = fan.fan_high_state.clone();
        let new_state: Weak<_> = Arc::downgrade(&new_state);

        fan.set_current_state(new_state);
    }

    fn set_fan(&self, arc: Arc<Fan>) {
        let weak = Arc::downgrade(&arc);
        let _ = self.fan.replace(Some(weak));
    }
}

struct FanHighState {
    fan: RefCell<Option<Weak<Fan>>>,
}

impl FanHighState {
    fn new() -> Self {
        FanHighState {
            fan: RefCell::new(None),
        }
    }
}

impl Display for FanHighState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FanHighState")
    }
}

impl FanState for FanHighState {
    fn handle_request(&self) {
        println!("Turning fan high to off");
        let fan = self.fan
            .borrow()
            .as_ref()
            .expect("Ué")
            .upgrade()
            .expect("Ué2");

        let new_state = fan.fan_off_state.clone();
        let new_state: Weak<_> = Arc::downgrade(&new_state);

        fan.set_current_state(new_state);
    }

    fn set_fan(&self, arc: Arc<Fan>) {
        let weak = Arc::downgrade(&arc);
        let _ = self.fan.replace(Some(weak));
    }
}
