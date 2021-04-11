
use array2d::Array2D;

use crate::graphics;
use super::position;

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

    pub fn get_visuals(&self) -> Vec<graphics::Visual> {
        let mut visuals = Vec::<graphics::Visual>::new();
        for (y, row_iter) in self.tiles.rows_iter().enumerate() {
            for (x, tile) in row_iter.enumerate() {
                let content = match tile {
                    Tile::Empty => 1,
                    Tile::Inaccessible => 0,
                };
                visuals.push(graphics::Visual{content, x:x as u32, y:y as u32, id:None});
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
pub struct Terminator {
    pub name: String,
    pub id: u32,
    pub pos: position::Position,
    pub aps: u32,
}

impl Terminator {
    pub fn get_visual(&self) -> graphics::Visual {
        let content = match self.pos.dir {
            position::Direction::Up => 2,
            position::Direction::Down => 3,
            position::Direction::Right => 4,
            position::Direction::Left => 5,
        };
        graphics::Visual{content, x:self.pos.x, y:self.pos.y, id:None}
    }
}

// Names
/*
https://boardgamegeek.com/thread/1719850/space-marine-names-between-editions-and-video-game

Base game:
Brother Zael (squad leader, purple)
Brother Omnio (generic, purple)
Brother Claudio (squad leader, yellow)
Brother Goriel (generic, yellow)
Lexicanium Calistarius (squad leader, grey)
Brother Scipio (generic, grey)
Seargeant Lorenzo (squad leader, blue)
Brother Deino (generic, blue)
Sergeant Gideon (squad leader, green)
Brother Noctis (generic, green)
Brother Leon (squad leader, red)
Brother Valencio (generic, red)

Space Marine Pack 1:
Brother Adron (squad leader, orange)
Brother Raphael (generic, orange)
Chaplain Raziel (squad leader, black)
Brother Metraen (generic, black)

Deathwing Space Marine Pack:
Sergeant Zaltys (blue)
Brother Jericho (blue)
Interrogator-Chaplain Uriel (purple)
Brother Charon (purple)
Apothecary Nestor (green)
Brother Examinare (green)
Librarian Menelauis (grey)
Brother Boreas (grey)
Brother Hephaestus (red)
Brother Raphean (red)
Sergeant Arbalan (yellow)
Brother Gabriel (yellow)
*/

// World
pub struct World {
    pub board: Board,
    pub terminators: Vec<Terminator>,
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
        let terminators = vec![
            Terminator{name:"Brother Omnio".to_string(), id:0, pos:position::Position{x:0, y:11, dir:position::Direction::Right}, aps:50},
            Terminator{name:"Brother Claudio".to_string(), id:0, pos:position::Position{x:1, y:11, dir:position::Direction::Right}, aps:50},
            Terminator{name:"Brother Goriel".to_string(), id:0, pos:position::Position{x:2, y:11, dir:position::Direction::Right}, aps:50},
            Terminator{name:"Brother Zael".to_string(), id:0, pos:position::Position{x:3, y:11, dir:position::Direction::Right}, aps:50},
            Terminator{name:"Sergeant Lorenzo".to_string(), id:4, pos:position::Position{x:4, y:11, dir:position::Direction::Right}, aps:50},
        ];
        World{board:board, terminators:terminators}
    }

    pub fn get_simple_visuals(&self) -> Vec<graphics::Visual> {
        let mut visuals = self.board.get_visuals();
        for t in &self.terminators {
            visuals.push(t.get_visual());
        }
        visuals
    }

    pub fn can_move(&self, pos: &position::Position, dx: i32, dy: i32) -> bool {
        let x = pos.x as i32 + dx;
        let y = pos.y as i32 + dy;
        if x < 0 || y < 0 {
            return false;
        }
        self.board.is_accessible(x as u32, y as u32)
    }
}
