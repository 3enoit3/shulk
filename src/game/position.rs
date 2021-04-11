#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn move_frontward(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        }
    }

    pub fn move_backward(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Right => (-1, 0),
            Direction::Left => (1, 0),
        }
    }
}

#[derive(Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
    pub dir: Direction,
}

impl Position {
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x = (self.x as i32 + dx) as u32;
        self.y = (self.y as i32 + dy) as u32;
    }

    pub fn rotate_left(&mut self) {
        self.dir = self.dir.rotate_left();
    }

    pub fn rotate_right(&mut self) {
        self.dir = self.dir.rotate_right();
    }
}
