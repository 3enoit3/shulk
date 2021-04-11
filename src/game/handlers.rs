use crate::graphics;
use crate::events;
use crate::game::world;
use crossterm::event::KeyEvent;

pub enum EventHandling {
    Keep,
//    SetHandler(String),
    Quit,
}

pub struct EventUpdate {
    pub visuals: Vec<graphics::Visual>,
    pub texts: Vec<graphics::Text>,
    pub events: EventHandling,
}

impl EventUpdate {
    pub fn quit() -> EventUpdate {
        EventUpdate{visuals:vec![], texts:vec![], events:EventHandling::Quit}
    }
}

pub trait GameHandler {
    fn handle_event(&mut self, world: &mut world::World, event: events::Event<KeyEvent>) -> EventUpdate;
}
