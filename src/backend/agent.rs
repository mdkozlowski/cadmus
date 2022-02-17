use crate::backend::gene::Genome;
use crate::backend::Position;

use crate::backend::map::Action;
use crate::backend::map::Action::Reproduce;

#[derive(Debug, Clone, Copy)]
pub struct Agent {
	pub position: Position,
	pub stats: AgentStats,
	pub genome: Genome,
	pub current_sense: Option<AgentSense>
}


#[derive(Debug, Clone, Copy)]
pub struct AgentSense {
	pub position: Position,
	pub map_tiles: [bool; 10]
}

#[derive(Debug, Clone, Copy)]
pub struct AgentStats {
	pub food_eaten: usize,
	pub cumulative_food_eaten: usize,
	pub steps_taken: usize,
}

impl Agent {
	pub fn get_move() -> Action {

		Reproduce
	}
}