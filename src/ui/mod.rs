pub mod components;
pub mod theme;

use crate::app::AppState;
use crate::ui::components::player::{render_details, render_now_playing, render_progress};
use crate::ui::components::statusbar::{render_footer, render_header};
use crate::ui::theme::{SPOTIFY_GREEN, TEXT_WHITE};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders};
use ratatui::Frame;

pub fn draw_ui(frame: &mut Frame, state: &AppState) {
    let size = frame.area();

    let root_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(SPOTIFY_GREEN))
        .title(" crystalify | Spotify Terminal Player ")
        .title_style(Style::default().fg(TEXT_WHITE).add_modifier(Modifier::BOLD));

    let inner_area = root_block.inner(size);
    frame.render_widget(root_block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(inner_area);

    render_header(frame, state, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(chunks[1]);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(6),
            Constraint::Length(3),
        ])
        .split(main_chunks[0]);

    render_now_playing(frame, state, left_chunks[0]);
    render_progress(frame, state, left_chunks[1]);
    render_details(frame, state, main_chunks[1]);

    render_footer(frame, chunks[2]);
}
