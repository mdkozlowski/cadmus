use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use cgmath::Vector2;
use rand::{Rng, RngCore, thread_rng};
use crate::backend::agent::{Agent, AgentStats};

mod map;
mod entity;
mod agent;
mod gene;
mod engine;
mod engine_tests;

type Position = Vector2<i32>;
type Offset = Vector2<i32>;

use crate::backend::engine::{Engine, EngineConfig, MatchStats};
use crate::backend::gene::{Genome, GenomePool};

pub struct Orchestrator {
	engine: Engine,
	pub genepool: GenomePool
}

impl Orchestrator {
	pub fn new() -> Self {

		// let mut brain = Genome::blank(123);
		// brain.test();
		// println!("{:?}", brain);

		let config = EngineConfig {
			food_spread: 2.5f64,
			size: [100f64, 100f64],
			round_max: 500,
			agent_count: 10
		};

		let engine = Engine::new(config);
		let genepool = GenomePool::new();

		Self {
			engine,
			genepool
		}
	}

	pub fn get_agent_position(&self, seen_positions: &mut HashSet<Position> ) -> Position {
		let mut position_found : bool = false;
		let mut candidate_position: Position = Vector2 { x: 0, y: 0 };
		while !position_found {
			let new_x = thread_rng().gen_range(0..=(self.engine.config.size[0] as i32));
			let new_y = thread_rng().gen_range(0..=(self.engine.config.size[1] as i32));
			candidate_position = Position::new(new_x, new_y);

			position_found = seen_positions.insert(candidate_position);
		}
		candidate_position
	}

	pub fn start_matches(&mut self) {
		let mut round:  usize = 0;

		let mut initial_agents: HashMap<u64, Agent> = HashMap::new();
		let mut seen_positions = HashSet::new();
		for idx in 0..self.engine.config.agent_count {
			let agent_id = thread_rng().next_u64();
			let genome = Rc::new(RefCell::new(Genome::blank(agent_id)));
			self.genepool.add_genome(agent_id, genome.clone());

			let new_agent = Agent {
				id: agent_id,
				position: self.get_agent_position(&mut seen_positions),
				genome: genome.clone(),
				current_sense: None
			};
			initial_agents.insert(agent_id, new_agent);
		}
		let mut match_stats = self.engine.play_match(initial_agents, round);

		for i in 0..100 {
			round += 1;
			let mut agents: HashMap<u64, Agent> = HashMap::new();
			seen_positions.clear();
			println!("{}", i);

			let new_genes = self.select_new_genes(&match_stats);

			for (id, gene) in new_genes.iter() {
				let new_agent = Agent {
					id: *id,
					position: self.get_agent_position(&mut seen_positions),
					genome: gene.clone(),
					current_sense: None
				};
				agents.insert(gene.borrow().id, new_agent);
			}
			match_stats = self.engine.play_match(agents, round);
			// matches.push(match_stats);
		}
	}

	pub fn select_new_genes(&mut self, match_stats: &MatchStats) -> HashMap<u64, Rc<RefCell<Genome>>> {
		let mut new_genes: HashMap<u64, Rc<RefCell<Genome>>> = HashMap::new();

		// println!("{:?}", match_stats);

		let mut agent_score = match_stats.agent_stats
			.iter()
			.collect::<Vec<(&u64, &AgentStats)>>();

		println!("{}", agent_score.iter().map(|(idx, stats)| stats.food_eaten).max().unwrap());

		agent_score.sort_by_key(|(id, stats)| stats.food_eaten);
		let best = agent_score
			.iter()
			.rev()
			.take(4)
			.map(|(id, stats)| (*id, *stats))
			.collect::<Vec<(&u64, &AgentStats)>>();

		// println!("{:?}", best);

		for (id, stats) in &match_stats.agent_stats {
			let mut gene = (self.genepool.get_genome(*id)).as_ref().borrow_mut();
			gene.stats.food_eaten = 0;
			gene.stats.cumulative_food_eaten += stats.food_eaten;
			gene.stats.steps_taken = 0;
		}

		for (id, score) in best.iter() {
			let gene = self.genepool.get_genome(**id);
			new_genes.insert(gene.borrow().id, gene.clone());
		}

		for (id, score) in best.iter() {
			let gene = self.genepool.get_genome(**id);

			let mut mutated_gene = gene.borrow().copy();
			mutated_gene.mutate();
			mutated_gene.stats.food_eaten = 0;
			mutated_gene.stats.cumulative_food_eaten += score.food_eaten;
			mutated_gene.stats.parent = Some(gene.borrow().id.clone());
			mutated_gene.stats.generation += 1;

			new_genes.insert(mutated_gene.id, Rc::new(RefCell::new(mutated_gene)));
		}

		for idx in 0..2 {
			let gene_id = thread_rng().next_u64();
			let gene = Rc::new(RefCell::new(Genome::blank(gene_id)));

			new_genes.insert(gene_id, gene);
		}

		for (id, gene) in &new_genes {
			self.genepool.add_genome(*id, gene.clone());
		}

		new_genes
	}
}
