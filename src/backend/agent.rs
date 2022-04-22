use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use tch::Tensor;
use crate::backend::engine::Engine;
use crate::backend::gene::Genome;
use crate::backend::Position;

use crate::backend::map::{Action, Direction};
use crate::backend::map::Action::{Move, Reproduce};

#[derive(Debug)]
pub struct Agent {
	pub id: u64,
	pub position: Position,
	pub genome: Rc<RefCell<Genome>>,
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
	pub parent: Option<u64>,
	pub generation: usize
}

impl AgentStats {
	pub fn new() -> Self {
		Self {
			food_eaten: 0,
			cumulative_food_eaten: 0,
			steps_taken: 0,
			parent: None,
			generation: 0
		}
	}
}

impl Agent {
	pub fn get_action(&mut self) -> Action {
		self.genome.as_ref().borrow_mut().stats.steps_taken += 1;

		let direction_idx = self.genome.borrow().forward(&self.build_input_tensor());

		return match direction_idx {
			0 => Move(Direction::Up),
			1 => Move(Direction::Down),
			2 => Move(Direction::Left),
			3 => Move(Direction::Right),
			_ => {Reproduce}
		}
	}

	fn build_input_tensor(&self) -> Tensor {
		let t = Tensor::of_slice(&self.current_sense.unwrap().map_tiles)
			.f_internal_cast_float(false)
			.unwrap()
			.unsqueeze(0);
		t
	}

	pub fn increment_food(&mut self) {
		self.genome.as_ref().borrow_mut().stats.food_eaten += 1;
	}
}