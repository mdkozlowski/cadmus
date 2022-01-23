use crate::backend::gene::Genome;
use crate::backend::Position;

#[derive(Debug)]
pub struct Agent {
	position: Position,
	stats: AgentStats,
	genome: Genome
}

#[derive(Debug)]
pub struct AgentStats {
	pub food_eaten: usize,
	pub steps_taken: usize
}

