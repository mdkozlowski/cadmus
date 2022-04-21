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
	var_store: Option<VarStore>,
	module: Sequential,
	id: u64
}

impl Genome {
	pub fn blank(id: u64) -> Self {
		let (net, var_store) = Network::get_network(&mut None);

		let data = Tensor::rand(&[1,49], kind::FLOAT_CPU).set_requires_grad(false);
		let out1 = net.forward(&data);
		out1.print();
		// println!("{:?}", out1.mean(kind::Kind::Float));


		for (name, mut var) in var_store.variables() {
			var.set_requires_grad(false);
			let new_var = var.shallow_clone() + Tensor::rand(var.size().as_slice(), kind::FLOAT_CPU);
			var.copy_(&new_var);
		}

		let (net, var_store) = Network::get_network(&mut Some(var_store));
		let out2 = net.forward(&data);
		out2.print();
		// println!("{:?}", out2.mean(kind::Kind::Float));

		Genome {
			var_store: Some(var_store),
			module: net,
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

	pub fn get_network(vs: &mut Option<nn::VarStore>) -> (Sequential, VarStore) {
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
			}
		}

		(default_module, new_var_store)
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