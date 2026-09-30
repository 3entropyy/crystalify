use crate::spotify::models::TrackMetadata;
use librespot::core::session::Session;
use librespot::core::spotify_uri::SpotifyUri;
use librespot::metadata::{Metadata, Track};
use librespot::playback::audio_backend;
use librespot::playback::config::{AudioFormat, PlayerConfig};
use librespot::playback::mixer::softmixer::SoftMixer;
use librespot::playback::mixer::{Mixer, MixerConfig};
use librespot::playback::player::{Player, PlayerEvent};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct SpotifyPlayer {
    player: Arc<Player>,
    mixer: SoftMixer,
    pub events: UnboundedReceiver<PlayerEvent>,
}

impl SpotifyPlayer {
    pub fn new(session: Session) -> Result<Self, Box<dyn std::error::Error>> {
        let backend = audio_backend::find(None).expect("Failed to find audio backend");

        let mixer = SoftMixer::open(MixerConfig::default())?;
        let initial_volume = 45000;
        mixer.set_volume(initial_volume);
        let volume_getter = mixer.get_soft_volume();

        let mut player_config = PlayerConfig::default();
        player_config.position_update_interval = Some(Duration::from_millis(250));

        let player = Player::new(
            player_config,
            session,
            volume_getter,
            move || backend(None, AudioFormat::F32),
        );
        let events = player.get_player_event_channel();

        Ok(Self {
            player,
            mixer,
            events,
        })
    }

    pub async fn fetch_metadata(
        session: &Session,
        uri: &SpotifyUri,
    ) -> Result<TrackMetadata, Box<dyn std::error::Error>> {
        let track = Track::get(session, uri).await?;
        let artist = track
            .artists
            .iter()
            .map(|a| a.name.clone())
            .collect::<Vec<_>>()
            .join(", ");

        Ok(TrackMetadata {
            title: track.name,
            artist,
            album: track.album.name,
            duration_ms: track.duration as u32,
        })
    }

    pub fn load(&self, uri: SpotifyUri) {
        self.player.load(uri, true, 0);
    }

    pub fn play(&self) {
        self.player.play();
    }

    pub fn pause(&self) {
        self.player.pause();
    }

    pub fn stop(&self) {
        self.player.stop();
    }

    pub fn seek(&self, position_ms: u32) {
        self.player.seek(position_ms);
    }

    pub fn volume(&self) -> u16 {
        self.mixer.volume()
    }

    pub fn set_volume(&mut self, volume: u16) {
        self.mixer.set_volume(volume);
        self.player.emit_volume_changed_event(volume);
    }
}
