use std::cmp;
use array2d::Array2D;
use crate::rendering;

// Board
#[derive(Clone)]
enum Tile {
    Empty,
    Inaccessible
}

pub struct Board {
    tiles: Array2D<Tile>,
}

impl Board {
    pub fn from_string(desc: &str) -> Board {
        let mut tiles = Vec::<Tile>::new();
        let (mut w, mut h) = (0, 0);
        for s in desc.split('\n') {
            w += 1;
            h = s.len();
            for c in s.chars() {
                if c == ' ' {
                    tiles.push(Tile::Inaccessible{});
                }
                else {
                    tiles.push(Tile::Empty{});
                }
            }
        }
        Board{tiles:Array2D::from_row_major(&tiles, w, h)}
    }

    pub fn get_visuals(&self) -> Vec<rendering::Visual> {
        let mut visuals = Vec::<rendering::Visual>::new();
        for (y, row_iter) in self.tiles.rows_iter().enumerate() {
            for (x, tile) in row_iter.enumerate() {
                let content = match tile {
                    Tile::Empty => 1,
                    Tile::Inaccessible => 0,
                };
                visuals.push(rendering::Visual{content:content, x:x as u32, y:y as u32, id:None});
            }
        }
        visuals
    }

    pub fn is_accessible(&self, x:u32, y:u32) -> bool {
        let tile = self.tiles.get(y as usize, x as usize);
        match tile {
            Some(Tile::Empty) => true,
            Some(Tile::Inaccessible) => false,
            None => false,
        }
    }
}

// Terminators
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn rotate_left(dir: &Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
    }
}

pub fn rotate_right(dir: &Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

pub fn move_frontward(dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Right => (1, 0),
        Direction::Left => (-1, 0),
    }
}

pub fn move_backward(dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (0, 1),
        Direction::Down => (0, -1),
        Direction::Right => (-1, 0),
        Direction::Left => (1, 0),
    }
}

pub struct Terminator {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub dir: Direction,
    pub aps: u32,
}

impl Terminator {
    pub fn get_visual(&self) -> rendering::Visual {
        let content = match self.dir {
            Direction::Up => 2,
            Direction::Down => 3,
            Direction::Right => 4,
            Direction::Left => 5,
        };
        rendering::Visual{content:content, x:self.x, y:self.y, id:None}
    }
}

// World
pub struct World {
    pub board: Board,
    pub terminator: Terminator,
}

impl World {
    pub fn new() -> World {
        let board_desc =
"                   EEE     
                   EEE     
                   EEE     
                    D      
                    E      
                    E      
                    E     I
                    E     E
              EDEEEEEEEEEEE
              E     E     E
      EEE     E     E     E
SSSSSDEEEDEEEEEEEEEEEEEEEEE
      EEE           E     I
       D            E      
       E            E      
       E            E      
       E            E      
       E            E      
       D   I      IEEEI    
      EEE  E               
      EEEEDE               
      EEE  E               
           I               ";
        let board = Board::from_string(board_desc);
        let terminator = Terminator{name:"Brother Omnio".to_string(), x:0, y:11, dir:Direction::Right, aps:4};
        World{board:board, terminator:terminator}
    }

    pub fn get_simple_visuals(&self) -> Vec<rendering::Visual> {
        let mut visuals = self.board.get_visuals();
        visuals.push(self.terminator.get_visual());
        visuals
    }

    pub fn move_terminator_if_possible(&mut self, dx: i32, dy: i32) {
        let x = cmp::max(0i32, self.terminator.x as i32 + dx) as u32;
        let y = cmp::max(0i32, self.terminator.y as i32 + dy) as u32;

        if self.board.is_accessible(x, y) {
            self.terminator.x = x;
            self.terminator.y = y;
        }
    }
}

