use cgmath::Vector2;
use crate::engine::entity::EntityType::Food;
use crate::engine::Position;

#[derive(Debug)]
pub struct Entity {
	position: Position,
	entity_type: EntityType
}

#[derive(Debug)]
pub enum EntityType {
	Food
}

impl Entity {
	pub fn new(pos: Position, entity_type: EntityType) -> Self {
		Self {
			position: pos,
			entity_type
		}
	}
}