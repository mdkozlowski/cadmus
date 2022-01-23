use crate::engine::Engine;

#[allow(dead_code)]

mod interface;
mod engine;

fn main() {
    let eng = Engine::new();
    // println!("{:}", eng.id)
}
