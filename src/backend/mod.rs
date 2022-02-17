use cgmath::Vector2;

mod map;
mod entity;
mod agent;
mod gene;
mod engine;

type Position = Vector2<i32>;
type Offset = Vector2<i32>;

use crate::backend::engine::{Engine, EngineConfig, MatchStats};
use crate::backend::gene::GenomePool;

pub struct Orchestrator {
	engine: Engine,
	pub genepool: GenomePool
}

impl Orchestrator {
	pub fn new() -> Self {
		let config = EngineConfig {
			food_spread: 2.5f64,
			size: [100f64, 100f64],
			round_max: 1000,
			agent_count: 10
		};

		let engine = Engine::new(config);
		let genepool = GenomePool::new();

		Self {
			engine,
			genepool
		}
	}

	pub fn start_matches(&mut self) {
		let mut matches: Vec<MatchStats> = Vec::new();
		for i in 0..5 {
			println!("{}", i);
			let match_stats = self.engine.play_match();
			matches.push(match_stats);
		}
	}
}
