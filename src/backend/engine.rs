use std::collections::HashMap;
use poisson_diskus::bridson;
use crate::backend::agent::{Agent, AgentSense, AgentStats};
use crate::backend::entity::{Entity, EntityType};
use crate::backend::gene::{Genome, GenomePool};
use crate::backend::Position;

#[derive(Debug)]
pub struct MatchStats {
	agent_stats: Vec<(AgentStats, Genome)>,
	duration: usize,
}

#[derive(Debug)]
pub struct Engine {
	pub config: EngineConfig,
	pub round_idx: usize,
	pub entities: Vec<Entity>,
	pub entity_mask: HashMap<Position, bool>,
	pub agents: Vec<Agent>,
	pub game_concluded: bool,
}

#[derive(Debug)]
pub struct EngineConfig {
	pub size: [f64; 2],
	pub round_max: usize,
	pub food_spread: f64,
	pub agent_count: usize,
}

impl Engine {
	const FOOD_RATE: f32 = 5.0;

	pub fn new(config: EngineConfig) -> Self {
		let mut engine = Self {
			config,
			round_idx: 0,
			entities: Vec::new(),
			agents: Vec::new(),
			game_concluded: false,
			entity_mask: HashMap::new(),
		};
		engine
	}

	pub fn play_match(&mut self) -> MatchStats {
		self.reset();
		self.initialise();

		while !self.game_concluded {
			self.step()
		}

		let stats = MatchStats {
			agent_stats: self.agents.iter()
				.map(|x| (x.stats.clone(), x.genome.clone()))
				.collect(),
			duration: self.round_idx,
		};

		stats
	}

	fn step(&mut self) {
		if self.round_idx > self.config.round_max {
			self.game_concluded = true;
		}

		self.process_agents();

		self.round_idx += 1;
	}

	fn process_agents(&mut self) {
		self.collect_visions()

	}

	fn collect_visions(&mut self) {
		for agent in &mut self.agents {
			let mut agent_sense = AgentSense {
				position: agent.position,
				map_tiles: [false; 10]
			};
			for (x_idx,x) in (-1..1).enumerate() {
				for (y_idx, y) in (-1..1).enumerate() {
					let target = agent.position + Position::new(x, y);
					let tile_index = x_idx + (y_idx * 3);
					if self.entity_mask.contains_key(&target) {
						agent_sense.map_tiles[tile_index] = true;
					}
				}
			}
			agent.current_sense = Some(agent_sense);
		}
	}

	fn reset(&mut self) {
		self.round_idx = 0;
		self.entities = Vec::new();
		self.agents = Vec::new();
	}

	fn initialise(&mut self) {
		self.place_food();


		// self.place_agents()
	}

	fn place_food(&mut self) {
		let rmin = self.config.food_spread;
		let k = 10;
		let use_pbc = false;

		let coords: Vec<Position> = bridson(&self.config.size, rmin, k, use_pbc)
			.unwrap()
			.iter().map(|a| a.map(|x| x as i32))
			.map(|a| Position::from(a))
			.collect();

		// println!("{:?}", coords);
		println!("{:?}", coords.len());
		for coord in coords {
			self.entities.push(Entity::new(coord, EntityType::Food));
			self.entity_mask.insert(coord, true);
		}
	}

	fn place_agents(&mut self) {}
}