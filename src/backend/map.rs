struct Map {

}

#[derive(PartialEq, Clone, Copy)]
pub enum Action {
	Move(Direction),
	Reproduce
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right
}