
use crossterm::{
    event::{self, Event as CEvent, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
    text::Text,
};

mod rendering;
mod game;
mod events;

fn event_loop(events_tx: mpsc::Sender<events::Event<KeyEvent>>) {
    let tick_rate = Duration::from_millis(30);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    events_tx.send(events::Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = events_tx.send(events::Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });
}

/// Main loop
fn game_loop<B>(terminal: &mut Terminal<B>, events_rx: mpsc::Receiver<events::Event<KeyEvent>>) -> Result<(), Box<dyn std::error::Error>>
where B : Backend
{
    let mut game = game::Game::new();

    loop {
        // Wait
        let event = events_rx.recv()?;

        // Update
        let keep_looping = (&mut game).handle_event(event);
        if !keep_looping {
            break;
        }

        // Draw
        let visuals = &game.get_visuals();
        terminal.draw(|frame| {
	    let size = frame.size();
            let content = rendering::render(&visuals);
            let board = Paragraph::new(Text::from(content))
                .style(Style::default().fg(Color::LightCyan))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Board")
                        .border_type(BorderType::Plain),
                );
            frame.render_widget(board, Rect::new(0, 0, size.width, size.height - 10));
        })?;

    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let (tx, rx) = mpsc::channel();
    event_loop(tx);
    let game_exit_status = game_loop(&mut terminal, rx);

    disable_raw_mode()?;
    terminal.show_cursor()?;
    println!();

    game_exit_status
}
