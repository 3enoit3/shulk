use crate::events;
use crate::graphics;
use crate::game::world;
use super::handlers;
use crossterm::{
    event::{KeyCode, KeyEvent},
};

pub struct MoveTerminatorHandler {
}

impl handlers::GameHandler for MoveTerminatorHandler {
    fn handle_event(&mut self, world: &mut world::World, event: events::Event<KeyEvent>) -> handlers::EventUpdate {
        let mut texts = Vec::<graphics::Text>::new();
        for t in &world.terminators {
            let text = graphics::Text::ItemAnnotation(t.id, t.name.clone());
            texts.push(text);
        }

        let selected = 4;
        let mut pos = world.terminators[selected].pos.clone();
        let mut aps = world.terminators[selected].aps;
        match event {
            events::Event::Input(key_event) => match key_event.code {
                KeyCode::Char('q') => {
                    return handlers::EventUpdate::quit();
                }
                KeyCode::Up => {
                    let (dx, dy) = pos.dir.move_frontward();
                    if aps > 0 && world.can_move(&pos, dx, dy) {
                        pos.move_by(dx, dy);
                        aps -= 1;
                    }
                }
                KeyCode::Down => {
                    let (dx, dy) = pos.dir.move_backward();
                    if aps > 0 && world.can_move(&pos, dx, dy) {
                        pos.move_by(dx, dy);
                        aps -= 1;
                    }
                }
                KeyCode::Left => {
                    pos.rotate_left();
                }
                KeyCode::Right => {
                    pos.rotate_right();
                }
                _ => {}
            },
            events::Event::Tick => {}
        }
        world.terminators[selected].pos = pos;
        world.terminators[selected].aps = aps;

        let visuals = world.get_simple_visuals();
        handlers::EventUpdate{visuals:visuals, texts:texts, events:handlers::EventHandling::Keep}
    }
}

// self.a = build_value(self.a)
// self.a.value()
// self.a = self.build_value(a);
// ! self.a = self.build_value(self.a) -> ?
// ! self.change_value(self.a) -> ?
// ! let &mut S = &mut self -> It cannot prove self is still valid when you use S
