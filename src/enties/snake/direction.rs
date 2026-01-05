pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}
impl Direction {
    pub fn can_change_to(&self, other: Direction) -> bool {
        !matches!(
            (self, other),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
                | (Direction::Up, Direction::Up)
                | (Direction::Down, Direction::Down)
                | (Direction::Left, Direction::Left)
                | (Direction::Right, Direction::Right)
        )
    }
}
