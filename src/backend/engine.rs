use poisson_diskus::bridson;
use crate::backend::agent::Agent;
use crate::backend::entity::{Entity, EntityType};
use crate::backend::gene::GenomePool;
use crate::backend::Position;

#[derive(Debug)]
pub struct Engine {
	pub size: [f64; 2],
	pub round_idx: usize,
	pub entities: Vec<Entity>,
	pub agents: Vec<Agent>,
}

impl Engine {
	const FOOD_RATE: f32 = 5.0;

	pub fn new() -> Self {
		let mut engine = Self {
			size: [100.0f64, 100.0f64],
			round_idx: 0,
			entities: Vec::new(),
			agents: Vec::new(),
		};
		engine.initialise();

		engine
	}

	fn initialise(&mut self) {
		self.place_food();


		// self.place_agents()
	}

	fn place_food(&mut self) {
		let rmin = 5.0;
		let k = 30;
		let use_pbc = true;

		let coords: Vec<Position> = bridson(&self.size, rmin, k, use_pbc)
			.unwrap()
			.iter().map(|a| a.map(|x| x as u32))
			.map(|a| Position::from(a))
			.collect();

		// println!("{:?}", coords);
		// println!("{:?}", coords.len());
		for coord in coords {
			self.entities.push(Entity::new(coord, EntityType::Food))
		}
	}

	fn place_agents(&mut self) {

	}
}