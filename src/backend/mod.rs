use cgmath::Vector2;

mod map;
mod entity;
mod agent;
mod gene;
mod engine;

type Position = Vector2<u32>;
type Offset = Vector2<i32>;

use crate::backend::engine::Engine;
use crate::backend::gene::GenomePool;

pub struct Orchestrator {
	engine: Engine,
	pub genepool: GenomePool
}

impl Orchestrator {
	pub fn new() -> Self {
		let engine = Engine::new();
		let genepool = GenomePool::new();

		Self {
			engine,
			genepool
		}
	}

	pub fn start_matches() {
		
	}
}
