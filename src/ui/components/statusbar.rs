use crate::app::AppState;
use crate::ui::theme::{ACCENT_CYAN, SPOTIFY_GREEN, TEXT_GRAY, TEXT_WHITE};
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_header(frame: &mut Frame, state: &AppState, area: Rect) {
    let header_text = vec![Line::from(vec![
        Span::styled(" Device: ", Style::default().fg(TEXT_GRAY)),
        Span::styled(
            &state.device_name,
            Style::default().fg(ACCENT_CYAN).add_modifier(Modifier::BOLD),
        ),
        Span::styled("  |  User: ", Style::default().fg(TEXT_GRAY)),
        Span::styled(
            &state.username,
            Style::default()
                .fg(TEXT_WHITE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  |  Audio Backend: ", Style::default().fg(TEXT_GRAY)),
        Span::styled("PipeWire (ALSA)", Style::default().fg(SPOTIFY_GREEN)),
    ])];

    let widget = Paragraph::new(header_text).block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(TEXT_GRAY)),
    );
    frame.render_widget(widget, area);
}

pub fn render_footer(frame: &mut Frame, area: Rect) {
    let footer_text = vec![Line::from(vec![
        Span::styled(
            " [Space] ",
            Style::default()
                .fg(SPOTIFY_GREEN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Play/Pause   ", Style::default().fg(TEXT_WHITE)),
        Span::styled(
            " [Left/Right] ",
            Style::default()
                .fg(SPOTIFY_GREEN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Seek +/-5s   ", Style::default().fg(TEXT_WHITE)),
        Span::styled(
            " [+/- or Up/Down] ",
            Style::default()
                .fg(SPOTIFY_GREEN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Volume   ", Style::default().fg(TEXT_WHITE)),
        Span::styled(
            " [Q/Esc] ",
            Style::default()
                .fg(SPOTIFY_GREEN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Quit", Style::default().fg(TEXT_WHITE)),
    ])];

    let widget = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(TEXT_GRAY)),
        );
    frame.render_widget(widget, area);
}
