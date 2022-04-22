use std::cell::RefCell;
use std::collections::HashMap;
use std::env::var;
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
	var_store: VarStore,
	module: Sequential,
	pub id: u64,
	pub stats: AgentStats
}

impl Genome {

	const MUTATION_STRENGTH: f64 = 0.5f64;

	pub fn blank(id: u64) -> Self {
		let (net, var_store) = Network::get_network(None);
		Genome {
			var_store,
			module: net,
			id,
			stats: AgentStats::new()
		}
	}

	pub fn test(&mut self) {
		let data = Tensor::rand(&[1,49], kind::FLOAT_CPU).set_requires_grad(false);
		let out1 = self.module.forward(&data);
		out1.print();

		self.mutate();

		let out2 = self.module.forward(&data);
		out2.print();

		let dist = out2.multinomial(1, false).int64_value(&[0,0]);
		println!("{}", dist);
		// dist.print();
	}

	pub fn mutate(&mut self) {

		for (_, mut var) in self.var_store.variables() {
			var.set_requires_grad(false);
			let new_var = var.shallow_clone() + (Tensor::rand(var.size().as_slice(), kind::FLOAT_CPU) * Genome::MUTATION_STRENGTH);
			var.copy_(&new_var);
		}
		let (module, var_store) = Network::get_network(Some(&self.var_store));

		self.module = module;
		self.var_store = var_store;
	}

	pub fn copy(&self) -> Genome {
		let (module, var_store) = Network::get_network(Some(&self.var_store));
		return Genome {
			id: thread_rng().next_u64(),
			module,
			var_store,
			stats: self.stats.clone()
		}
	}

	pub fn forward(&self, data: &Tensor) -> i64 {
		return self.module.forward(data).multinomial(1, false).int64_value(&[0,0]);
	}
}

pub struct Network {
	genome: Genome,
	module: Sequential
}

impl Network {

	const HIDDEN_NODES: i64 = 32;

	pub fn get_network(vs: Option<&nn::VarStore>) -> (Sequential, VarStore) {
		let mut new_var_store = VarStore::new(Device::Cpu);
		new_var_store.freeze();

		let path = &new_var_store.root();
		let default_module = nn::seq()
			.add(nn::linear(
				path / "layer1",
				Engine::DISTANCE_VISIBLE_BLOCKS as i64,
				Network::HIDDEN_NODES,
				Default::default(),
			))
			.add_fn(|xs| xs.tanh())
			.add(nn::linear(path / "final", Network::HIDDEN_NODES, 4, Default::default()))
			.add_fn(|xs| xs.softmax(1, kind::Kind::Float));

		match vs {
			None => {}
			Some(new_weights) => {
				new_var_store.copy(new_weights);
				new_var_store.freeze();
			}
		}

		(default_module, new_var_store)
	}
}

#[derive(Debug)]
pub struct GenomePool {
	pool: HashMap<u64, Rc<RefCell<Genome>>>,
}

impl GenomePool {
	pub fn new() -> Self {
		Self {
			pool: HashMap::new(),
		}
	}

	pub fn add_genome(&mut self, id: u64, genome: Rc<RefCell<Genome>>) {
		if !self.pool.contains_key(&id) {
			self.pool.insert(id, genome);
		}
	}

	pub fn get_genome(&self, id: u64) -> &Rc<RefCell<Genome>> {
		self.pool.get(&id).unwrap()
	}

	// pub fn update_stats(&mut self, id: u64, new_stats: AgentStats) {
	// 	let mut agent_stats = *self.stats.get(&id).unwrap();
	//
	// 	agent_stats.cumulative_food_eaten += new_stats.cumulative_food_eaten;
	// 	agent_stats.steps_taken += new_stats.steps_taken;
	// 	agent_stats.food_eaten += new_stats.food_eaten;
	// }
}