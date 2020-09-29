use std::sync::Arc;

struct Personagem {
    name: String,
    state: EstadoDoPersonagem,
    previous_state: EstadoDoPersonagem,
}

impl Personagem {
    fn new(name: String) -> Self {
        Personagem {
            name,
            state: EstadoDoPersonagem::Small,
            previous_state: EstadoDoPersonagem::Small,
        }
    }

    fn set_state(&mut self, new_state: EstadoDoPersonagem) {
        self.previous_state = self.state;
        self.state = new_state;
    }

    fn hit(&self) {
        self.state.hit();
    }

    fn trigger(&self) {
        self.state.trigger();
    }
}
#[derive(Copy,Clone,Debug)]
enum EstadoDoPersonagem {
    Small,
    Large,
    Invincible,
}

trait EstadoDoPersonagemTrait {
    fn hit(&self);
    fn trigger(&self);
}



impl EstadoDoPersonagemTrait for EstadoDoPersonagem {
    fn hit(&self) {
        println!("hit {:?}", self);
    }

    fn trigger(&self) {
        println!("trigger {:?}", self);
    }
}

fn main() {
    let mario = Personagem::new("Mario".to_string());

    mario.hit();
    mario.trigger();
}
