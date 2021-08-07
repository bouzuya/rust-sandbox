mod dot;
mod node;
mod state;

use crate::{dot::parse, state::State};
use std::{env, fs, io};
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Terminal,
};

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let content = fs::read_to_string(args.get(1).unwrap())?;
    let initial_node_label = args.get(2).unwrap();

    let mut state = {
        let graph = parse(&content)?;
        let nodes = graph.nodes();
        let edges = graph
            .edges()
            .into_iter()
            .map(|(l, r, _)| {
                (
                    nodes.iter().position(|x| x == &l).unwrap(),
                    nodes.iter().position(|x| x == &r).unwrap(),
                )
            })
            .collect();
        let initial_node_id = nodes.iter().position(|n| n == initial_node_label).unwrap();
        State::new(edges, nodes.into_iter().collect(), initial_node_id)
    };

    let stdin = io::stdin();
    stdin.lock();
    let mut keys = stdin.keys();
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut list_state_left = ListState::default();
    list_state_left.select(None);
    let mut list_state_right = ListState::default();
    list_state_right.select(None);
    let mut pos = None;
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

            let highlight_style = Style::default()
                .bg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD);
            let items = state
                .from()
                .into_iter()
                .map(|node| ListItem::new(node.to_string()))
                .collect::<Vec<ListItem>>();
            let items = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("From"))
                .highlight_style(highlight_style);
            f.render_stateful_widget(items, chunks[0], &mut list_state_left);
            let block = Block::default()
                .title(state.selected().to_string())
                .borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
            let items = state
                .to()
                .into_iter()
                .map(|node| ListItem::new(node.to_string()))
                .collect::<Vec<ListItem>>();
            let items = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("To"))
                .highlight_style(highlight_style);
            f.render_stateful_widget(items, chunks[2], &mut list_state_right);
        })?;

        let key = keys.next().unwrap().unwrap();
        match key {
            Key::Char('q') => {
                break;
            }
            Key::Char('h') | Key::Left => match pos {
                Some(false) => {
                    if let Some(i) = list_state_left.selected() {
                        let id = state.from().get(i).unwrap().id();
                        state.select(id).unwrap();
                        pos = None;
                        list_state_left.select(None);
                        list_state_right.select(None);
                    } else if !state.from().is_empty() {
                        list_state_left.select(Some(0));
                    }
                }
                None | Some(true) => {
                    pos = Some(false);
                    list_state_right.select(None);
                    if !state.from().is_empty() {
                        list_state_left.select(Some(0));
                    }
                }
            },
            Key::Char('l') | Key::Right => match pos {
                Some(true) => {
                    if let Some(i) = list_state_right.selected() {
                        let id = state.to().get(i).unwrap().id();
                        state.select(id).unwrap();
                        pos = None;
                        list_state_left.select(None);
                        list_state_right.select(None);
                    } else if !state.to().is_empty() {
                        list_state_right.select(Some(0));
                    }
                }
                None | Some(false) => {
                    pos = Some(true);
                    list_state_left.select(None);
                    if !state.to().is_empty() {
                        list_state_right.select(Some(0));
                    }
                }
            },
            Key::Char('j') | Key::Down => match pos {
                None => {}
                Some(false) => match list_state_left.selected() {
                    Some(i) => {
                        if i + 1 < state.from().len() {
                            list_state_left.select(Some(i + 1));
                        }
                    }
                    None => {
                        if !state.from().is_empty() {
                            list_state_left.select(Some(0));
                        }
                    }
                },
                Some(true) => match list_state_right.selected() {
                    Some(i) => {
                        if i + 1 < state.to().len() {
                            list_state_right.select(Some(i + 1));
                        }
                    }
                    None => {
                        if !state.to().is_empty() {
                            list_state_right.select(Some(0));
                        }
                    }
                },
            },
            Key::Char('k') | Key::Up => match pos {
                None => {}
                Some(false) => match list_state_left.selected() {
                    Some(i) => {
                        if i > 0 {
                            list_state_left.select(Some(i - 1));
                        }
                    }
                    None => {
                        if !state.from().is_empty() {
                            list_state_left.select(Some(state.from().len() - 1));
                        }
                    }
                },
                Some(true) => match list_state_right.selected() {
                    Some(i) => {
                        if i > 0 {
                            list_state_right.select(Some(i - 1));
                        }
                    }
                    None => {
                        if !state.to().is_empty() {
                            list_state_right.select(Some(state.to().len() - 1));
                        }
                    }
                },
            },
            _ => {}
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
