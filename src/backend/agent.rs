use std::rc::Rc;
use crate::backend::engine::Engine;
use crate::backend::gene::Genome;
use crate::backend::Position;

use crate::backend::map::Action;
use crate::backend::map::Action::Reproduce;

#[derive(Debug, Hash)]
pub struct Agent {
	pub id: u64,
	pub position: Position,
	pub stats: AgentStats,
	pub current_sense: Option<AgentSense>
}


#[derive(Debug, Clone, Copy, Hash)]
pub struct AgentSense {
	pub position: Position,
	pub map_tiles: [bool; Engine::DISTANCE_VISIBLE_BLOCKS]
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct AgentStats {
	pub food_eaten: usize,
	pub cumulative_food_eaten: usize,
	pub steps_taken: usize,
}

impl AgentStats {
	pub fn new() -> Self {
		Self {
			food_eaten: 0,
			cumulative_food_eaten: 0,
			steps_taken: 0
		}
	}
}

impl Agent {
	pub fn get_action(&mut self) -> Action {
		self.stats.steps_taken += 1;

		Reproduce
	}

	pub fn increment_food(&mut self) {
		self.stats.food_eaten += 1;
	}
}