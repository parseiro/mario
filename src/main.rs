// #![allow(dead_code)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]
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

use std::cell::{Ref, RefCell};
use std::rc::Rc;

struct Player {
    state: RefCell<Rc<PlayerState>>,
    previous_state: RefCell<Rc<PlayerState>>,
}

struct PlayerBuilder {}

impl PlayerBuilder {
    fn build() -> Result<Player, String> {
        Ok(Player {
            state: RefCell::new(Rc::new(PlayerState::Small)),
            previous_state: RefCell::new(Rc::new(PlayerState::Dead)),
        })
    }
}

impl Player {
    // fn set_state<T: 'static>(&self, new_state: T)
    // where T: PlayerStateTrait
    fn set_state(&self, new_state: PlayerState)
    {
        let old_rc: Rc<PlayerState> =
            self.state.replace(Rc::new(new_state));
        let _ = self.previous_state.replace(old_rc);
    }

    fn extract_rc(&self) -> Rc<PlayerState> {
        let cell: Ref<Rc<PlayerState>> = RefCell::borrow(&self.state);
        let rc: Rc<PlayerState> = Rc::clone(&cell);
        rc
    }

    fn hit_player(&self) {
        let new_state: PlayerState =
            self.extract_rc()
            .hit();

        self.set_state(new_state);
    }

    fn mushroom_player(&self) {
        let new_state: PlayerState =
            self.extract_rc()
            .mushroom();

        self.set_state(new_state);
    }

    fn star(&self) {
        let new_state: PlayerState =
            self.extract_rc()
            .star();

        self.set_state(new_state);
    }
}

trait PlayerStateTrait {
    fn hit(&self) -> PlayerState;
    fn mushroom(&self) -> PlayerState;
    fn star(&self) -> PlayerState;
}

#[derive(Debug)]
enum PlayerState {
    Dead,
    Small,
    Large,
    Star,
}

impl PlayerStateTrait for PlayerState {
    fn hit(&self) -> PlayerState {
        match self {
            PlayerState::Small => {
                println!("small was hit, died");
                PlayerState::Dead
            }
            PlayerState::Large => {
                println!("I was large -> small again");
                PlayerState::Small
            }
            _ => {
                panic!("The dead don't need to change state")
            }
        }
    }

    fn mushroom(&self) -> PlayerState {
        match self {
            PlayerState::Small => {
                println!("small is growing");
                PlayerState::Large
            }
            PlayerState::Large => {
                println!("Can't grow anymore");
                PlayerState::Large
            }
            _ => {
                panic!("The dead don't need to change state")
            }
        }
    }

    fn star(&self) -> PlayerState {
        match self {
            PlayerState::Small => {
                println!("small got star");
                PlayerState::Star
            }
            PlayerState::Large => {
                println!("Large got star");
                PlayerState::Star
            }
            _ => {
                panic!("The dead don't need star")
            }
        }
    }
}

fn main() {
    {
        let mario = PlayerBuilder::build().expect("Error creating a player");
        println!("{:?}", mario.state.borrow());
        mario.hit_player();
        println!("{:?} -> {:?}", mario.previous_state.borrow(), mario.state.borrow());
        println!("----------------------")
    }

    {
        let mario = PlayerBuilder::build().expect("Error creating a player");
        println!("{:?}", mario.state.borrow());
        mario.mushroom_player();
        println!("{:?} -> {:?}", mario.previous_state.borrow(), mario.state.borrow());
        println!("----------------------")
    }

    {
        let mario = PlayerBuilder::build().expect("Error creating a player");
        mario.set_state(PlayerState::Large);
        println!("{:?}", mario.state.borrow());
        mario.hit_player();
        println!("{:?} -> {:?}", mario.previous_state.borrow(), mario.state.borrow());
        println!("----------------------")
    }

    {
        let mario = PlayerBuilder::build().expect("Error creating a player");
        mario.set_state(PlayerState::Large);
        println!("{:?}", mario.state.borrow());
        mario.mushroom_player();
        println!("{:?} -> {:?}", mario.previous_state.borrow(), mario.state.borrow());
        println!("----------------------")
    }

    {
        let mario = PlayerBuilder::build().expect("Error creating a player");
        println!("{:?}", mario.state.borrow());
        mario.star();
        println!("{:?} -> {:?}", mario.previous_state.borrow(), mario.state.borrow());
        println!("----------------------")
    }

    {
        let mario = PlayerBuilder::build().expect("Error creating a player");
        mario.set_state(PlayerState::Large);
        println!("{:?}", mario.state.borrow());
        mario.star();
        println!("{:?} -> {:?}", mario.previous_state.borrow(), mario.state.borrow());
        println!("----------------------")
    }
}

