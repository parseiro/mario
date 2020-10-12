#![feature(lang_items, start)]
#![no_std]

#![allow(dead_code)]
// #![allow(unused_imports)]
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

// use core::cell::Cell;

/*macro_rules! println {
    ($($arg:tt)*) => ({

    })
}*/



#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[start]
fn main(argc: isize, argv: *const *const u8) -> isize {
//fn main() {
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
        // current_state: Cell::new(&fan_off_state),
        // current_state: &fan_off_state,
    };

/*    for i in 0..=9 {
        // let state = fan.current_state.get();
        let state= fan.current_state;

        println!("Estado atual: {}", state);

        state.handle_request(&fan);
    }*/

    0
}

// struct Fan<'a> {
struct Fan {
    fan_off_state: FanOffState,
    fan_low_state: FanLowState,
    fan_med_state: FanMedState,
    fan_high_state: FanHighState,

    // current_state: Cell<&'a dyn FanState>,
    // current_state: &'a dyn FanState,
}

// impl<'a> Fan<'a> {
impl Fan {
    // fn set_current_state<'b: 'a>(&self, state: &'b dyn FanState) {
    fn set_current_state<'b>(&self, state: &'b dyn FanState) {
        // let _ = self.current_state.replace(state);
        // self.current_state = state;
    }
}

trait FanState {
    // fn handle_request<'a>(&self, fan: &'a Fan<'a>);
    fn handle_request<'a>(&self, fan: &'a Fan);
}

#[derive(Clone,Copy)]
struct FanOffState { }

/*impl Display for FanOffState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FanOffState")
    }
}*/

impl FanState for FanOffState {
    // fn handle_request<'a>(&self, fan: &'a Fan<'a>) {
    fn handle_request<'a>(&self, fan: &'a Fan) {
        // println!("Turning fan off to low");

        fan.set_current_state(&fan.fan_low_state);
    }
}

#[derive(Clone,Copy)]
struct FanLowState { }

impl FanState for FanLowState {
    // fn handle_request<'a>(&self, fan: &'a Fan<'a>) {
    fn handle_request<'a>(&self, fan: &'a Fan) {
        // println!("Turning fan low to med");

        // fan.set_current_state(&fan.fan_med_state);
    }
}

#[derive(Clone,Copy)]
struct FanMedState { }

/*impl Display for FanMedState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FanMedState")
    }
}*/

impl FanState for FanMedState {
    // fn handle_request<'a>(&self, fan: &'a Fan<'a>) {
    fn handle_request<'a>(&self, fan: &'a Fan) {
        // println!("Turning fan med to high");

        // fan.set_current_state(&fan.fan_high_state);
    }
}

#[derive(Clone,Copy)]
struct FanHighState { }

/*impl Display for FanHighState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FanHighState")
    }
}*/

impl FanState for FanHighState {
    // fn handle_request<'a>(&self, fan: &'a Fan<'a>) {
    fn handle_request<'a>(&self, fan: &'a Fan) {
        // println!("Turning fan high to off");

        // fan.set_current_state(&fan.fan_off_state);
    }
}
