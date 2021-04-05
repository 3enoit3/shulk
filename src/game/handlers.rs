use crate::rendering;
use crate::events;
use crate::game::world;
use crossterm::event::KeyEvent;

pub enum EventHandling {
    Keep,
//    SetHandler(String),
    Quit,
}

pub struct EventUpdate {
    pub visuals: Vec<rendering::Visual>,
    pub texts: Vec<rendering::Text>,
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
