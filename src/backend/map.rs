struct Map {

}

#[derive(PartialEq, Clone, Copy)]
pub enum Action {
	Move(Direction),
	Reproduce
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
	Up = 0,
	Down = 1,
	Left = 2,
	Right = 3
}