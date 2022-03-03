use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Index;
use poisson_diskus::bridson;
use crate::backend::agent::{Agent, AgentSense, AgentStats};
use crate::backend::entity::{Entity, EntityType};
use crate::backend::gene::{Genome, GenomePool};
use crate::backend::map::{Action, Direction};
use crate::backend::{Offset, Position};
use std::rc::Rc;
use rand::prelude::{IteratorRandom, SliceRandom};

#[derive(Debug)]
pub struct MatchStats {
	agent_stats: Vec<(AgentStats, Genome)>,
	duration: usize,
}

#[derive(Debug)]
pub struct Engine {
	pub config: EngineConfig,
	pub round_idx: usize,
	// pub entities: Vec<Entity>,
	pub entities: HashMap<Position, Entity>,
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
			// entities: Vec::new(),
			agents: Vec::new(),
			game_concluded: false,
			entities: HashMap::new(),
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

		let actions = self.process_agents();
		self.apply_actions(actions);

		self.round_idx += 1;
	}

	pub fn resolve_action(&self, agent: &Agent, action: &Action) -> Position {
		let current_pos = agent.position;
		match action {
			Action::Move(direction) => {
				match direction {
					Direction::Up => { current_pos + Offset::new(0, -1) }
					Direction::Down => { current_pos + Offset::new(0, 1) }
					Direction::Left => { current_pos + Offset::new(-1, 0) }
					Direction::Right => { current_pos + Offset::new(1, 0) }
				}
			}
			Action::Reproduce => { current_pos }
		}
	}

	fn resolve_target_position(&self, seen_positions: &mut HashSet<Position>, agent: &Agent, action: &Action) -> Position {
		let target_position = self.resolve_action(agent, &action);

		let duplicated_position = seen_positions.insert(target_position);
		return if duplicated_position {
			let new_direction_collection = match action {
				Action::Move(dir) => {
					[Direction::Up, Direction::Down, Direction::Left, Direction::Right]
						.iter()
						.filter(|a| **a != *dir)
						.map(|a| *a)
						.collect()
				}
				Action::Reproduce => {
					[Direction::Up, Direction::Down, Direction::Left, Direction::Right]
						.to_vec()
				}
			};
			let new_direction = new_direction_collection
				.choose(&mut rand::thread_rng())
				.unwrap()
				.clone();
			let new_action = Action::Move(new_direction);
			let new_position = self.resolve_target_position(seen_positions, agent, &new_action);

			new_position
		} else {
			target_position
		}
	}

	fn apply_actions(&mut self, actions: Vec<(usize, Action)>) {
		let mut seen_positions: HashSet<Position> = HashSet::new();
		let mut target_positions: Vec<(usize, Position)> = Vec::new();
		for (idx, action) in actions.iter() {
			let agent: &Agent = self.agents.get(*idx).unwrap();

			let new_position = self.resolve_target_position(&mut seen_positions, agent, action);
			target_positions.push((*idx, new_position));
		}

		// set new position
		for (idx, target) in target_positions.iter() {
			self.agents.get_mut(*idx).unwrap().position = *target
		}

		// consume food
		for agent in self.agents.iter_mut() {
			if self.entities.contains_key(&agent.position) {
				self.entities.remove(&agent.position);
				agent.increment_food();
			}
		}
	}

	fn process_agents(&mut self) -> Vec<(usize, Action)> {
		let mut actions: Vec<(usize, Action)> = Vec::new();
		self.collect_visions();
		for agent in self.agents.iter_mut() {
			let action = agent.get_action();
			actions.push((agent.id, action));
		}
		actions
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
					if self.entities.contains_key(&target) {
						agent_sense.map_tiles[tile_index] = true;
					}
				}
			}
			agent.current_sense = Some(agent_sense);
		}
	}

	fn reset(&mut self) {
		self.round_idx = 0;
		// self.entities = HashMap::new();
		self.entities = HashMap::new();
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
			// self.entities.push(Entity::new(coord, EntityType::Food));
			// self.entity_mask.insert(coord);
			self.entities.insert(coord, Entity::new(coord, EntityType::Food));
		}
	}

	fn place_agents(&mut self) {}
}