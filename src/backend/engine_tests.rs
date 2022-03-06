#[cfg(test)]
mod engine_tests {
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
			stats: AgentStats {
				steps_taken: 0,
				food_eaten: 0,
				cumulative_food_eaten: 0
			},
			position: Position::new(5,5),
			id: 0,
			genome: Genome {},
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
	fn move_oob() {
		let mut engine = get_engine();
		let mut agent1 = Agent {
			stats: AgentStats {
				steps_taken: 0,
				food_eaten: 0,
				cumulative_food_eaten: 0
			},
			position: Position::new(0,0),
			id: 0,
			genome: Genome {},
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
}