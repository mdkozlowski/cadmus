use std::rc::Rc;
use cgmath::Vector2;
use crate::backend::agent::{Agent, AgentStats};

mod map;
mod entity;
mod agent;
mod gene;
mod engine;
mod engine_tests;

type Position = Vector2<i32>;
type Offset = Vector2<i32>;

use crate::backend::engine::{Engine, EngineConfig, MatchStats};
use crate::backend::gene::{Genome, GenomePool};

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

	pub fn get_agents(&self, count: usize) -> Vec<Agent> {
		let mut agents: Vec<Agent> = Vec::new();
		for idx in 0..count {
			let new_agent = Agent {
				stats: AgentStats::new(),
				genome: Rc::new(Genome::blank()),
				id: idx,
				position: Position::new(0,0),
				current_sense: None
			};
			agents.push(new_agent);
		}
		agents
	}

	pub fn start_matches(&mut self) {
		let initial_agents = self.get_agents(self.engine.config.agent_count);

		for idx in 0..config.agent_count {
			genepool.
		}


		let mut matches: Vec<MatchStats> = Vec::new();
		for i in 0..5 {
			println!("{}", i);
			let match_stats = self.engine.play_match();
			matches.push(match_stats);
		}
	}
}
