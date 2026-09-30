mod app;
mod spotify;
mod ui;

use app::AppState;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use librespot::core::spotify_uri::SpotifyUri;
use ratatui::{backend::CrosstermBackend, Terminal};
use spotify::auth::authenticate;
use spotify::player::SpotifyPlayer;
use std::env;
use std::io::stdout;
use std::time::Duration;
use ui::draw_ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = authenticate().await?;
    let username = session.username();
    let device_name = "crystalify".to_string();

    let mut player = SpotifyPlayer::new(session.clone())?;
    let initial_volume = player.volume();

    let track_arg = env::args()
        .nth(1)
        .unwrap_or_else(|| "spotify:track:5hVghJ4KaLM9bO4dM1yY6g".to_string());
    let track_uri = SpotifyUri::from_uri(&track_arg)?;

    let mut state = AppState::new(username, device_name, initial_volume);

    if let Ok(meta) = SpotifyPlayer::fetch_metadata(&session, &track_uri).await {
        state.set_track(meta);
    }

    player.load(track_uri);

    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
        default_panic(info);
    }));

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend_terminal = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend_terminal)?;

    'main_loop: loop {
        terminal.draw(|frame| draw_ui(frame, &state))?;

        tokio::select! {
            Some(event) = player.events.recv() => {
                state.handle_player_event(event);
            }
            _ = tokio::time::sleep(Duration::from_millis(30)) => {
                state.tick(30);

                while event::poll(Duration::from_millis(0))? {
                    if let Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Char('q') | KeyCode::Esc => {
                                    player.stop();
                                    break 'main_loop;
                                }
                                KeyCode::Char(' ') => {
                                    if state.is_playing {
                                        player.pause();
                                        state.is_playing = false;
                                    } else {
                                        player.play();
                                        state.is_playing = true;
                                    }
                                }
                                KeyCode::Right => {
                                    let new_pos = (state.position_ms + 5000).min(state.duration_ms);
                                    player.seek(new_pos);
                                    state.position_ms = new_pos;
                                }
                                KeyCode::Left => {
                                    let new_pos = state.position_ms.saturating_sub(5000);
                                    player.seek(new_pos);
                                    state.position_ms = new_pos;
                                }
                                KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::Up => {
                                    let new_vol = state.volume.saturating_add(3276);
                                    player.set_volume(new_vol);
                                    state.volume = new_vol;
                                }
                                KeyCode::Char('-') | KeyCode::Down => {
                                    let new_vol = state.volume.saturating_sub(3276);
                                    player.set_volume(new_vol);
                                    state.volume = new_vol;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
