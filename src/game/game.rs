use crate::graphics;
use crate::events;
use super::world;
use super::handlers;
use super::handler_move_terminator::MoveTerminatorHandler;
use crossterm::event::KeyEvent;

pub struct GameUpdate {
    pub visuals: Vec<graphics::Visual>,
    pub texts: Vec<graphics::Text>,
    pub quit: bool,
}

pub struct Game {
    world: world::World,
    handler: Box<dyn handlers::GameHandler>,
}

impl Game {
    pub fn new() -> Game {
        let world = world::World::new();
        let handler = Box::new(MoveTerminatorHandler{});
        Game{world:world, handler:handler}
    }

    pub fn handle_event(&mut self, event: events::Event<KeyEvent>) -> GameUpdate {
        let results = self.handler.handle_event(&mut self.world, event);
        let quit = match results.events {
            handlers::EventHandling::Quit => true,
            _ => false,
        };
        GameUpdate{visuals:results.visuals, texts:results.texts, quit:quit}
    }
}
