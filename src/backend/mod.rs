use std::rc::Rc;
use cgmath::Vector2;
use rand::{Rng, RngCore, thread_rng};
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

		let brain = Genome::blank(123);
		println!("{:?}", brain);
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

	pub fn get_agent_position(&mut self) -> Position {
		let mut position_found : bool = false;
		let mut candidate_position: Position = Vector2 { x: 0, y: 0 };
		while !position_found {
			let new_x = thread_rng().gen_range(0..=(self.engine.config.size[0] as i32));
			let new_y = thread_rng().gen_range(0..=(self.engine.config.size[1] as i32));
			candidate_position = Position::new(new_x, new_y);

			for (idx, agent) in self.engine.agents.iter() {
				if agent.position == candidate_position {
					position_found = false;
				}
				position_found = true;
			}
		}
		candidate_position
	}

	pub fn start_matches(&mut self) {

		for idx in 0..self.engine.config.agent_count {
			let agent_id = thread_rng().next_u64();
			let genome = Rc::new(Genome::blank(agent_id));
			self.genepool.add_genome(agent_id, genome);

			let new_agent = Agent {
				stats: AgentStats::new(),
				id: agent_id,
				position: self.get_agent_position(),
				current_sense: None
			};
			self.engine.agents.insert(agent_id, new_agent);
		}

		let mut matches: Vec<MatchStats> = Vec::new();
		for i in 0..5 {
			println!("{}", i);
			let match_stats = self.engine.play_match();
			matches.push(match_stats);
		}
	}
}
