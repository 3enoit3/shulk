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
        let mut texts = vec![rendering::Text::ItemAnnotation(world.terminator.id, world.terminator.name.clone())];
        match event {
            events::Event::Input(key_event) => match key_event.code {
                KeyCode::Char('q') => {
                    return handlers::EventUpdate::quit();
                }
                KeyCode::Up => {
                    let (dx, dy) = world::move_frontward(&world.terminator.dir);
                    if world.terminator.aps > 0 && world.can_move(&world.terminator, dx, dy) {
                        world.terminator.x = (world.terminator.x as i32 + dx) as u32;
                        world.terminator.y = (world.terminator.y as i32 + dy) as u32;
                        world.terminator.aps -= 1;
                    }
                }
                KeyCode::Down => {
                    let (dx, dy) = world::move_backward(&world.terminator.dir);
                    if world.terminator.aps > 0 && world.can_move(&world.terminator, dx, dy) {
                        world.terminator.x = (world.terminator.x as i32 + dx) as u32;
                        world.terminator.y = (world.terminator.y as i32 + dy) as u32;
                        world.terminator.aps -= 1;
                    }
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
        handlers::EventUpdate{visuals:visuals, texts:texts, events:handlers::EventHandling::Keep}
    }
}
