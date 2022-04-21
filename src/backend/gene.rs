use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use rand::prelude::SliceRandom;
use rand::{RngCore, thread_rng};
use tch::{CModule, Device, nn, Tensor, kind};
use tch::nn::{Module, Sequential, VarStore};
use crate::backend::agent::{Agent, AgentStats};
use crate::backend::engine::Engine;
use crate::backend::Position;
// use tch::nn::CModule;

#[derive(Debug)]
pub struct Genome {
	var_store: Option<VarStore>,
	id: u64
}

impl Genome {
	pub fn blank(id: u64) -> Self {
		let var_store = VarStore::new(Device::Cpu);
		Network::get_random_net(&var_store);

		Genome {
			var_store: Some(var_store),
			id
		}
	}
}

pub struct Network {
	genome: Genome,
	module: Sequential
}

impl Network {

	const HIDDEN_NODES: i64 = 32;

	pub fn get_random_net(var_store: &VarStore) -> Sequential {
		let vs = &var_store.root();
		let default_module = nn::seq()
			.add(nn::linear(
				vs / "layer1",
				Engine::DISTANCE_VISIBLE_BLOCKS as i64,
				Network::HIDDEN_NODES,
				Default::default(),
			))
			.add_fn(|xs| xs.tanh())
			.add(nn::linear(vs / "final", Network::HIDDEN_NODES, 4, Default::default()));

		default_module
	}
}

#[derive(Debug)]
pub struct GenomePool {
	pool: HashMap<u64, Rc<Genome>>,
	stats: HashMap<u64, AgentStats>
}

impl GenomePool {
	pub fn new() -> Self {
		Self {
			pool: HashMap::new(),
			stats: HashMap::new()
		}
	}

	pub fn add_genome(&mut self, id: u64, genome: Rc<Genome>) {
		self.pool.insert(id, genome);
		self.stats.insert(id, AgentStats::new());
	}

	pub fn get_genome(&self, id: u64) -> &Rc<Genome> {
		self.pool.get(&id).unwrap()
	}

	pub fn update_stats(&mut self, id: u64, new_stats: AgentStats) {
		let mut agent_stats = *self.stats.get(&id).unwrap();

		agent_stats.cumulative_food_eaten += new_stats.cumulative_food_eaten;
		agent_stats.steps_taken += new_stats.steps_taken;
		agent_stats.food_eaten += new_stats.food_eaten;
	}
}