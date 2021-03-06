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
	pub agent_stats: HashMap<u64, AgentStats>,
	pub duration: usize,
}

#[derive(Debug)]
pub struct Engine {
	pub config: EngineConfig,
	pub round_idx: usize,
	// pub entities: Vec<Entity>,
	pub entities: HashMap<Position, Entity>,
	pub agents: HashMap<u64, Agent>,
	pub game_concluded: bool,
	pub round: usize
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
			agents: HashMap::new(),
			game_concluded: false,
			entities: HashMap::new(),
			round: 0
		};
		engine
	}


	pub fn play_match(&mut self, agents: HashMap<u64, Agent>, round: usize) -> MatchStats {
		self.reset();
		self.agents = agents;
		self.initialise();
		self.round = round;

		while !self.game_concluded {
			self.step()
		}

		let stats = MatchStats {
			agent_stats: self.agents.iter()
				.map(|(idx, x)| (*idx, x.genome.borrow().stats.clone()))
				.collect::<HashMap<u64, AgentStats>>(),
			duration: self.round_idx.clone(),
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
		let mut target_pos = match action {
			Action::Move(direction) => {
				match direction {
					Direction::Up => { current_pos + Offset::new(0, -1) }
					Direction::Down => { current_pos + Offset::new(0, 1) }
					Direction::Left => { current_pos + Offset::new(-1, 0) }
					Direction::Right => { current_pos + Offset::new(1, 0) }
				}
			}
			Action::Reproduce => { current_pos }
		};
		let bounds = (*self.config.size.get(0).unwrap() as i32,
					  *self.config.size.get(1).unwrap() as i32);

		if target_pos.x > bounds.0 {
			target_pos.x = bounds.0;
		}
		if target_pos.x < 0 {
			target_pos.x = 0;
		}
		if target_pos.y > bounds.1 {
			target_pos.y = bounds.1;
		}
		if target_pos.y < 0 {
			target_pos.y = 0;
		}

		target_pos
	}

	fn resolve_target_position(&self, seen_positions: &mut HashSet<Position>, agent: &Agent, action: &Action) -> Position {
		let target_position = self.resolve_action(agent, &action);

		let duplicated_position = !seen_positions.insert(target_position);
		return if duplicated_position {
			let possible_directions = match action {
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
			let new_direction = possible_directions
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

	pub fn apply_actions(&mut self, actions: HashMap<u64, Action>) {
		let mut seen_positions: HashSet<Position> = HashSet::new();
		let mut target_positions: HashMap<u64, Position> = HashMap::new();
		for (idx, action) in actions.iter() {
			let agent: &Agent = self.agents.get(idx).unwrap();

			let new_position = self.resolve_target_position(&mut seen_positions, agent, action);
			target_positions.insert(*idx, new_position);
		}

		// set new position
		for (idx, target) in target_positions.iter() {
			self.agents.get_mut(idx).unwrap().position = *target
		}

		// consume food
		for (idx, agent) in self.agents.iter_mut() {
			if self.entities.contains_key(&agent.position) {
				self.entities.remove(&agent.position);
				agent.increment_food();
			}
		}
	}

	pub fn process_agents(&mut self) -> HashMap<u64, Action> {
		let mut actions: HashMap<u64, Action> = HashMap::new();
		self.collect_visions();
		for (idx, agent) in self.agents.iter_mut() {
			let action = agent.get_action();
			actions.insert(agent.id, action);
		}
		actions
	}

	pub const DISTANCE_VISIBLE_SIDE : i32 = 3;
	pub const DISTANCE_VISIBLE_LENGTH : usize = (Engine::DISTANCE_VISIBLE_SIDE as usize * 2) + 1;
	pub const DISTANCE_VISIBLE_BLOCKS : usize = (Engine::DISTANCE_VISIBLE_LENGTH as usize).pow(2);

	pub fn collect_visions(&mut self) {
		for (idx, agent) in &mut self.agents {
			let mut agent_sense = AgentSense {
				position: agent.position,
				map_tiles: [false; Engine::DISTANCE_VISIBLE_BLOCKS]
			};
			for (x_idx,x) in (-Engine::DISTANCE_VISIBLE_SIDE..Engine::DISTANCE_VISIBLE_SIDE).enumerate() {
				for (y_idx, y) in (-Engine::DISTANCE_VISIBLE_SIDE..Engine::DISTANCE_VISIBLE_SIDE).enumerate() {
					let target = agent.position + Position::new(x as i32, y as i32);
					let tile_index = x_idx + (y_idx * Engine::DISTANCE_VISIBLE_LENGTH);
					if self.entities.contains_key(&target) {
						agent_sense.map_tiles[tile_index] = true;
					}
				}
			}
			agent.current_sense = Some(agent_sense);
		}
	}

	pub fn reset(&mut self) {
		self.round_idx = 0;
		// self.entities = HashMap::new();
		self.entities = HashMap::new();
		self.agents = HashMap::new();
		self.game_concluded = false;
	}

	pub(crate) fn initialise(&mut self) {
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
		for coord in coords {
			// self.entities.push(Entity::new(coord, EntityType::Food));
			// self.entity_mask.insert(coord);
			self.entities.insert(coord, Entity::new(coord, EntityType::Food));
		}
	}

	fn place_agents(&mut self) {}
}