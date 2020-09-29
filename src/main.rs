#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use]
extern crate derive_builder;

use std::borrow::BorrowMut;
use std::cell::{Cell, Ref, RefCell};
use std::rc::Rc;

// #[derive(Builder)]
struct Personagem {
    // name: String,
    state: RefCell<Rc<EstadoDoPersonagem>>,
    previous_state: RefCell<Rc<EstadoDoPersonagem>>,
}

struct PersonagemBuilder {}

impl PersonagemBuilder {
    fn build() -> Result<Personagem, String> {
        Ok(Personagem {
            state: RefCell::new(Rc::new(EstadoDoPersonagem::Small)),
            previous_state: RefCell::new(Rc::new(EstadoDoPersonagem::Dead))
        })
    }
}

impl Personagem {
    // fn set_state<T: 'static>(&self, new_state: T)
    // where T: EstadoDoPersonagemTrait
    fn set_state(&self, new_state: EstadoDoPersonagem)
    {
        let old_rc: Rc<EstadoDoPersonagem> = self.state.replace(Rc::new(new_state));
        self.previous_state.replace(old_rc);
    }

    fn extract_rc(&self) -> Rc<EstadoDoPersonagem> {
        let celula: Ref<Rc<EstadoDoPersonagem>> = RefCell::borrow(&self.state);
        let rc: Rc<EstadoDoPersonagem> = Rc::clone(&celula);
        rc
    }

    fn hit(&self) {
        let new_state: EstadoDoPersonagem = self.extract_rc()
            .hit();

        self.set_state(new_state);
    }

    fn mushroom(&self) {
        let new_state: EstadoDoPersonagem = self.extract_rc()
            .mushroom();

        self.set_state(new_state);
    }

    fn star(&self) {
        let new_state: EstadoDoPersonagem = self.extract_rc()
            .star();

        self.set_state(new_state);
    }
}

/*#[derive(Copy, Clone, Debug)]
enum EstadoDoPersonagem {
    Small,
    Large,
    Invincible,
}*/

trait EstadoDoPersonagemTrait {
    fn hit(&self) -> EstadoDoPersonagem;
    fn mushroom(&self) -> EstadoDoPersonagem;
    fn star(&self) -> EstadoDoPersonagem;
}

#[derive(Debug)]
enum EstadoDoPersonagem {
    Dead,
    Small,
    Large,
    Star,
}

impl EstadoDoPersonagemTrait for EstadoDoPersonagem {
    fn hit(&self) -> EstadoDoPersonagem {
        match self {
            EstadoDoPersonagem::Small => {
                println!("small was hit, died");
                EstadoDoPersonagem::Dead
            }
            EstadoDoPersonagem::Large => {
                println!("I was large -> small again");
                EstadoDoPersonagem::Small
            }
            _ => {
                panic!("The dead don't need to change state")
            }
        }
    }

    fn mushroom(&self) -> EstadoDoPersonagem {
        match self {
            EstadoDoPersonagem::Small => {
                println!("small is growing");
                EstadoDoPersonagem::Large
            }
            EstadoDoPersonagem::Large => {
                println!("Can't grow anymore");
                EstadoDoPersonagem::Large
            }
            _ => {
                panic!("The dead don't need to change state")
            }
        }
    }

    fn star(&self) -> EstadoDoPersonagem {
        match self {
            EstadoDoPersonagem::Small => {
                println!("small got star");
                EstadoDoPersonagem::Star
            }
            EstadoDoPersonagem::Large => {
                println!("Large got star");
                EstadoDoPersonagem::Star
            }
            _ => {
                panic!("The dead don't need star")
            }
        }
    }
}

/*impl EstadoDoPersonagemTrait for LargeState {
    fn hit(&self) -> EstadoDoPersonagem {

    }

    fn mushroom(&self) -> EstadoDoPersonagem {
        println!("already large");

        // keeps in the same state
        EstadoDoPersonagem::Large
    }
}*/

fn main() {
    {
        let mario = PersonagemBuilder::build().unwrap();
        println!("{:?}", mario.state.borrow());
        mario.hit();
        println!("{:?}", mario.state.borrow());
        println!("----------------------")
    }

    {
        let mario = PersonagemBuilder::build().unwrap();
        println!("{:?}", mario.state.borrow());
        mario.mushroom();
        println!("{:?}", mario.state.borrow());
        println!("----------------------")
    }

    {
        let mario = PersonagemBuilder::build().unwrap();
        mario.set_state(EstadoDoPersonagem::Large);
        println!("{:?}", mario.state.borrow());
        mario.hit();
        println!("{:?}", mario.state.borrow());
        println!("----------------------")
    }

    {
        let mario = PersonagemBuilder::build().unwrap();
        mario.set_state(EstadoDoPersonagem::Large);
        println!("{:?}", mario.state.borrow());
        mario.mushroom();
        println!("{:?}", mario.state.borrow());
        println!("----------------------")
    }

    {
        let mario = PersonagemBuilder::build().unwrap();
        println!("{:?}", mario.state.borrow());
        mario.star();
        println!("{:?}", mario.state.borrow());
        println!("----------------------")
    }

    {
        let mario = PersonagemBuilder::build().unwrap();
        mario.set_state(EstadoDoPersonagem::Large);
        println!("{:?}", mario.state.borrow());
        mario.star();
        println!("{:?}", mario.state.borrow());
        println!("----------------------")
    }

}

