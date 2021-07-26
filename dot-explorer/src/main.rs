mod node;
mod state;

use crate::state::State;
use std::{env, io};
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, ListState},
    Terminal,
};

fn main() -> anyhow::Result<()> {
    let mut args = env::args();
    let initial_node_label = args.nth(1).unwrap();

    let state = {
        let nodes = vec!["a", "b", "c"];
        let edges = vec![(0, 1), (0, 2), (1, 2)];
        let initial_node_id = nodes.iter().position(|n| n == &initial_node_label).unwrap();
        State::new(edges, nodes, initial_node_id)
    };

    let stdin = io::stdin();
    stdin.lock();
    let mut keys = stdin.keys();
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut list_state = ListState::default();
    list_state.select(None);
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Main block with round corners")
                .border_type(BorderType::Rounded);
            f.render_widget(block, size);
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Percentage(40),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let block = Block::default().title("With borders").borders(Borders::ALL);
            f.render_widget(block, chunks[0]);
            let block = Block::default()
                .title(state.selected().to_string())
                .borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
            let block = Block::default().title("With borders").borders(Borders::ALL);
            f.render_widget(block, chunks[2]);
        })?;

        let key = keys.next().unwrap().unwrap();
        if key == Key::Char('q') {
            break;
        }
    }

    Ok(())
}

fn render(state: &State) {
    println!("{}", state.selected());
    for node in state.from() {
        println!("{} -> {}", state.selected(), node);
    }
    for node in state.to() {
        println!("{} -> {}", node, state.selected());
    }
}
