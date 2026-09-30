use crate::app::AppState;
use crate::ui::theme::{ACCENT_CYAN, ACCENT_YELLOW, SPOTIFY_GREEN, TEXT_GRAY, TEXT_WHITE};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Gauge, Paragraph};
use ratatui::Frame;

pub fn render_now_playing(frame: &mut Frame, state: &AppState, area: Rect) {
    let status_badge = if state.is_playing {
        Span::styled(
            " [PLAYING] ",
            Style::default()
                .bg(SPOTIFY_GREEN)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled(
            " [PAUSED] ",
            Style::default()
                .bg(ACCENT_YELLOW)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
    };

    let card_text = vec![
        Line::from(vec![status_badge]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Title:  ", Style::default().fg(TEXT_GRAY)),
            Span::styled(
                &state.track_title,
                Style::default().fg(TEXT_WHITE).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Artist: ", Style::default().fg(TEXT_GRAY)),
            Span::styled(&state.track_artist, Style::default().fg(ACCENT_CYAN)),
        ]),
        Line::from(vec![
            Span::styled("Album:  ", Style::default().fg(TEXT_GRAY)),
            Span::styled(&state.track_album, Style::default().fg(Color::Gray)),
        ]),
    ];

    let widget = Paragraph::new(card_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Now Playing "),
    );
    frame.render_widget(widget, area);
}

pub fn render_progress(frame: &mut Frame, state: &AppState, area: Rect) {
    let elapsed_sec = state.position_ms / 1000;
    let total_sec = state.duration_ms / 1000;
    let ratio = if state.duration_ms > 0 {
        (state.position_ms as f64 / state.duration_ms as f64).clamp(0.0, 1.0)
    } else {
        0.0
    };

    let label = format!(
        "{:02}:{:02} / {:02}:{:02}",
        elapsed_sec / 60,
        elapsed_sec % 60,
        total_sec / 60,
        total_sec % 60
    );

    let widget = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Progress "),
        )
        .gauge_style(Style::default().fg(SPOTIFY_GREEN).bg(TEXT_GRAY))
        .ratio(ratio)
        .label(label);
    frame.render_widget(widget, area);
}

pub fn render_details(frame: &mut Frame, state: &AppState, area: Rect) {
    let vol_percent = ((state.volume as f64 / u16::MAX as f64) * 100.0).round() as u8;
    let text = vec![
        Line::from(vec![
            Span::styled("Audio Pipeline: ", Style::default().fg(TEXT_GRAY)),
            Span::styled("librespot -> PipeWire", Style::default().fg(TEXT_WHITE)),
        ]),
        Line::from(vec![
            Span::styled("Format:         ", Style::default().fg(TEXT_GRAY)),
            Span::styled("44.1 kHz / 16-bit", Style::default().fg(TEXT_WHITE)),
        ]),
        Line::from(vec![
            Span::styled("Bitrate:        ", Style::default().fg(TEXT_GRAY)),
            Span::styled("320 kbps (Vorbis)", Style::default().fg(TEXT_WHITE)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Volume: ", Style::default().fg(TEXT_GRAY)),
            Span::styled(
                format!("{}%", vol_percent),
                Style::default()
                    .fg(ACCENT_YELLOW)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let widget = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Playback Details "),
    );
    frame.render_widget(widget, area);
}
