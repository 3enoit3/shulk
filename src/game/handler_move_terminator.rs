use crate::events;
use crate::game::world;
use super::handlers;
use crossterm::{
    event::{KeyCode, KeyEvent},
};

pub struct MoveTerminatorHandler {
}

impl handlers::GameHandler for MoveTerminatorHandler {
    fn handle_event(&mut self, world: &mut world::World, event: events::Event<KeyEvent>) -> handlers::EventUpdate {
        match event {
            events::Event::Input(key_event) => match key_event.code {
                KeyCode::Char('q') => {
                    return handlers::EventUpdate{visuals:vec![], events:handlers::EventHandling::Quit};
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
        let visuals = world.get_simple_visuals();
        handlers::EventUpdate{visuals:visuals, events:handlers::EventHandling::Keep}
    }
}
