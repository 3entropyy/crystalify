use crate::spotify::models::TrackMetadata;
use librespot::playback::player::PlayerEvent;

pub struct AppState {
    pub track_title: String,
    pub track_artist: String,
    pub track_album: String,
    pub duration_ms: u32,
    pub position_ms: u32,
    pub is_playing: bool,
    pub volume: u16,
    pub username: String,
    pub device_name: String,
}

impl AppState {
    pub fn new(username: String, device_name: String, volume: u16) -> Self {
        Self {
            track_title: "Unknown Track".to_string(),
            track_artist: "Unknown Artist".to_string(),
            track_album: "Unknown Album".to_string(),
            duration_ms: 0,
            position_ms: 0,
            is_playing: false,
            volume,
            username,
            device_name,
        }
    }

    pub fn set_track(&mut self, meta: TrackMetadata) {
        self.track_title = meta.title;
        self.track_artist = meta.artist;
        self.track_album = meta.album;
        self.duration_ms = meta.duration_ms;
        self.position_ms = 0;
        self.is_playing = true;
    }

    pub fn handle_player_event(&mut self, event: PlayerEvent) {
        match event {
            PlayerEvent::Playing { position_ms, .. } => {
                self.position_ms = position_ms;
                self.is_playing = true;
            }
            PlayerEvent::Paused { position_ms, .. } => {
                self.position_ms = position_ms;
                self.is_playing = false;
            }
            PlayerEvent::Stopped { .. } | PlayerEvent::EndOfTrack { .. } => {
                self.is_playing = false;
            }
            PlayerEvent::PositionChanged { position_ms, .. }
            | PlayerEvent::PositionCorrection { position_ms, .. }
            | PlayerEvent::Seeked { position_ms, .. } => {
                self.position_ms = position_ms;
            }
            PlayerEvent::VolumeChanged { volume } => {
                self.volume = volume;
            }
            _ => {}
        }
    }

    pub fn tick(&mut self, delta_ms: u32) {
        if self.is_playing && self.position_ms < self.duration_ms {
            self.position_ms = (self.position_ms + delta_ms).min(self.duration_ms);
        }
    }
}
