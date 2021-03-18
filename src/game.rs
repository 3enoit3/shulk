
use std::cmp;
use array2d::Array2D;
use crate::rendering;
use crate::events;
use crossterm::{
    event::{KeyCode, KeyEvent},
};

// Board
#[derive(Clone)]
enum Tile {
    Empty,
    Inaccessible
}

struct Board {
    tiles: Array2D<Tile>,
}

impl Board {
    fn from_string(desc: &str) -> Board {
        let mut tiles = Vec::<Tile>::new();
        let (mut w, mut h) = (0, 0);
        for s in desc.split('\n') {
            w += 1;
            h = s.len();
            for c in s.chars() {
                if c == 'E' {
                    tiles.push(Tile::Empty{});
                }
                else {
                    tiles.push(Tile::Inaccessible{});
                }
            }
        }
        Board{tiles:Array2D::from_row_major(&tiles, w, h)}
    }

    fn get_visuals(&self) -> Vec<rendering::Visual> {
        let mut visuals = Vec::<rendering::Visual>::new();
        for (y, row_iter) in self.tiles.rows_iter().enumerate() {
            for (x, tile) in row_iter.enumerate() {
                let content = match tile {
                    Tile::Empty => 1,
                    Tile::Inaccessible => 0,
                };
                visuals.push(rendering::Visual{content:content, x:x as u32, y:y as u32});
            }
        }
        visuals
    }

    fn is_accessible(&self, x:u32, y:u32) -> bool {
        let tile = self.tiles.get(y as usize, x as usize);
        match tile {
            Some(Tile::Empty) => true,
            Some(Tile::Inaccessible) => false,
            None => false,
        }
    }
}

// Terminators
struct Terminator {
    pub x: u32,
    pub y: u32,
}

impl Terminator {
    fn new() -> Terminator {
        Terminator{x:0, y:0}
    }

    fn get_visual(&self) -> rendering::Visual {
        rendering::Visual{content:3, x:self.x, y:self.y}
    }
}

// World
struct World {
    board: Board,
    terminator: Terminator,
}

impl World {
    pub fn new() -> World {
        let board = Board::from_string("EEE\n E \nEEE");
        let terminator = Terminator::new();
        World{board:board, terminator:terminator}
    }

    pub fn get_simple_visuals(&self) -> Vec<rendering::Visual> {
        let mut visuals = self.board.get_visuals();
        visuals.push(self.terminator.get_visual());
        visuals
    }

    fn move_terminator_if_possible(&mut self, dx: i32, dy: i32) {
        let x = cmp::max(0i32, self.terminator.x as i32 + dx) as u32;
        let y = cmp::max(0i32, self.terminator.y as i32 + dy) as u32;

        if self.board.is_accessible(x, y) {
            self.terminator.x = x;
            self.terminator.y = y;
        }
    }
}

// Game
trait GameHandler {
    fn handle_event(&mut self, world: &mut World, event: events::Event<KeyEvent>) -> bool;
    fn get_visuals(&self, world: &World) -> Vec<rendering::Visual>;
}

pub struct Game {
    world: World,
    handler: Box<dyn GameHandler>,
}

impl Game {
    pub fn new() -> Game {
        let world = World::new();
        let handler = Box::new(MoveTerminatorHandler{});
        Game{world:world, handler:handler}
    }

    pub fn handle_event(&mut self, event: events::Event<KeyEvent>) -> bool {
        self.handler.handle_event(&mut self.world, event)
    }

    pub fn get_visuals(&self) -> Vec<rendering::Visual> {
        self.handler.get_visuals(&self.world)
    }
}

// Move
struct MoveTerminatorHandler {
}

impl GameHandler for MoveTerminatorHandler {
    fn handle_event(&mut self, world: &mut World, event: events::Event<KeyEvent>) -> bool {
        match event {
            events::Event::Input(key_event) => match key_event.code {
                KeyCode::Char('q') => {
                    return false;
                }
                KeyCode::Up => {
                    world.move_terminator_if_possible(0, -1);
                }
                KeyCode::Down => {
                    world.move_terminator_if_possible(0, 1);
                }
                KeyCode::Left => {
                    world.move_terminator_if_possible(-1, 0);
                }
                KeyCode::Right => {
                    world.move_terminator_if_possible(1, 0);
                }
                _ => {}
            },
            events::Event::Tick => {}
        }
        true
    }

    fn get_visuals(&self, world: &World) -> Vec<rendering::Visual> {
        world.get_simple_visuals()
    }
}


