use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use rand::prelude::SliceRandom;
use rand::{RngCore, thread_rng};
use tch::{CModule, Device, nn, Tensor};
use tch::nn::{Module, Sequential, VarStore};
use test::RunIgnored::No;
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
	pub fn blank() -> Self {
		Genome {
			var_store: None,
			id: thread_rng().next_u64()
		}
	}
}

pub struct Brain {
	genome: Genome,
	module: Sequential
}

impl Brain {

	const HIDDEN_NODES: i64 = 32;

	pub fn get_net(genome: &Genome) -> Sequential {
		let vs = &genome.var_store.as_ref().unwrap().root();
		let default_module = nn::seq()
			.add(nn::linear(
				vs / "layer1",
				Engine::DISTANCE_VISIBLE_BLOCKS as i64,
				Brain::HIDDEN_NODES,
				Default::default(),
			))
			.add_fn(|xs| xs.relu())
			.add(nn::linear(vs, Brain::HIDDEN_NODES, 4, Default::default()));

		default_module
	}

	pub fn new(genome: Genome) -> Self {
		if genome.var_store.is_none() {
			return Brain {
				genome,
				module: nn::seq()
			}
		}

		Brain {
			module: Brain::get_net(&genome),
			genome,
		}
	}
}

#[derive(Debug)]
pub struct GenomePool {
	pool: HashMap<u64, Rc<RefCell<Genome>>>,
	stats: HashMap<u64, AgentStats>
}

impl GenomePool {
	pub fn new() -> Self {
		Self {
			pool: HashMap::new(),
			stats: HashMap::new()
		}
	}

	pub fn add_genome(&mut self, id: u64, genome: Genome) {
		self.pool.insert(id, Rc::new(RefCell::new(genome)));
		self.stats.insert(id, AgentStats::new());
	}

	pub fn get_genome(&self, id: u64) -> &Rc<RefCell<Genome>> {
		self.pool.get(&id).unwrap()
	}

	pub fn update_stats(&mut self, id: u64, new_stats: AgentStats) {
		let mut agent_stats = *self.stats.get(&id).unwrap();

		agent_stats.cumulative_food_eaten += new_stats.cumulative_food_eaten;
		agent_stats.steps_taken += new_stats.steps_taken;
		agent_stats.food_eaten += new_stats.food_eaten;
	}
}