struct Map {

}

pub enum Action {
	Move(Direction),
	Reproduce
}

enum Direction {
	Up,
	Down,
	Left,
	Right
}