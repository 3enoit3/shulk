use crate::events;
use crate::rendering;
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
                    return handlers::EventUpdate::quit();
                }
                KeyCode::Up => {
                    let (dx, dy) = world::move_frontward(&world.terminator.dir);
                    world.move_terminator_if_possible(dx, dy);
                }
                KeyCode::Down => {
                    let (dx, dy) = world::move_backward(&world.terminator.dir);
                    world.move_terminator_if_possible(dx, dy);
                }
                KeyCode::Left => {
                    world.terminator.dir = world::rotate_left(&world.terminator.dir);
                }
                KeyCode::Right => {
                    world.terminator.dir = world::rotate_right(&world.terminator.dir);
                }
                _ => {}
            },
            events::Event::Tick => {}
        }
        let visuals = world.get_simple_visuals();
        let annotations = vec![rendering::Annotation{text:world.terminator.name.clone(), anchor:rendering::Anchor::Position(10, 30)}];
        handlers::EventUpdate{visuals:visuals, events:handlers::EventHandling::Keep, annotations:annotations}
    }
}
