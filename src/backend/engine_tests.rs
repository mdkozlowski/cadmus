#[cfg(test)]
mod engine_tests {
	use std::cell::RefCell;
	use std::collections::{HashMap, HashSet};
	use std::rc::Rc;
	use crate::backend::agent::{Agent, AgentStats};
	use crate::backend::engine::{Engine, EngineConfig};
	use crate::backend::gene::Genome;
	use crate::backend::map::{Action, Direction};
	use crate::backend::Position;

	fn get_engine() -> Engine {
		let engine_config = EngineConfig {
			food_spread: 2.5f64,
			size: [100f64, 100f64],
			round_max: 1000,
			agent_count: 10
		};
		let mut engine = Engine::new(engine_config);
		engine.reset();
		engine
	}

	#[test]
	fn move_normal() {
		let mut engine = get_engine();
		let agent1 = Agent {
			position: Position::new(5,5),
			id: 0,
			genome: Rc::new(RefCell::new(Genome::blank(1))),
			current_sense: None
		};
		let action = Action::Move(Direction::Up);
		let target_position = engine.resolve_action(&agent1, &action);
		assert_eq!(target_position, Position::new(5, 4));

		let action = Action::Move(Direction::Left);
		let target_position = engine.resolve_action(&agent1, &action);
		assert_eq!(target_position, Position::new(4, 5));

		let action = Action::Move(Direction::Down);
		let target_position = engine.resolve_action(&agent1, &action);
		assert_eq!(target_position, Position::new(5, 6));

		let action = Action::Move(Direction::Right);
		let target_position = engine.resolve_action(&agent1, &action);
		assert_eq!(target_position, Position::new(6, 5));
	}

	#[test]
	fn move_collision() {
		let mut engine = get_engine();
		let agent1 = Agent {
			position: Position::new(5,4),
			id: 1,
			genome: Rc::new(RefCell::new(Genome::blank(1))),
			current_sense: None
		};
		let agent2 = Agent {
			position: Position::new(5,6),
			genome: Rc::new(RefCell::new(Genome::blank(2))),
			id: 2,
			current_sense: None
		};

		engine.agents.insert(1, agent1);
		engine.agents.insert(2, agent2);

		let action1 = Action::Move(Direction::Down);
		let action2 = Action::Move(Direction::Up);
		let mut actions = HashMap::new();

		actions.insert(1, action1);
		actions.insert(2, action2);

		engine.apply_actions(actions);

		let mut seen_positions: HashSet<Position> = HashSet::new();
		for (idx, agent) in engine.agents.iter() {
			seen_positions.insert(agent.position.clone());
		}
		assert_eq!(seen_positions.len(), 2);
	}

	#[test]
	fn move_oob() {
		let mut engine = get_engine();
		let mut agent1 = Agent {
			position: Position::new(0,0),
			id: 0,
			genome: Rc::new(RefCell::new(Genome::blank(0))),
			current_sense: None
		};

		let action = Action::Move(Direction::Up);
		let target_position = engine.resolve_action(&agent1, &action);
		assert_eq!(target_position, Position::new(0, 0));

		let action = Action::Move(Direction::Left);
		let target_position = engine.resolve_action(&agent1, &action);
		assert_eq!(target_position, Position::new(0, 0));

		agent1.position = Position::new(100, 100);

		let action = Action::Move(Direction::Down);
		let target_position = engine.resolve_action(&agent1, &action);
		assert_eq!(target_position, Position::new(100, 100));

		let action = Action::Move(Direction::Right);
		let target_position = engine.resolve_action(&agent1, &action);
		assert_eq!(target_position, Position::new(100, 100));
	}

	#[test]
	fn genome_vision() {
		let mut engine = get_engine();
		engine.reset();
		engine.initialise();

		let mut agent1 = Agent {
			position: Position::new(10,10),
			id: 0,
			genome: Rc::new(RefCell::new(Genome::blank(0))),
			current_sense: None
		};
		engine.agents.insert(0, agent1);
		engine.process_agents();

		println!("{:?}", engine.agents);
		assert_eq!(1,1);
	}
}