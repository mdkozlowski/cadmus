use crate::backend::Orchestrator;

#[allow(dead_code)]

mod interface;
mod backend;

fn main() {
    let orchestrator = Orchestrator::new();
    orchestrator.start()
    // println!("{:}", eng.id)
}
