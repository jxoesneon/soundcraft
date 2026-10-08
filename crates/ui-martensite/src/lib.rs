//! Sovereign retained-mode interface for SoundCraft built on the Martensite GUI engine.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

pub mod command_reg;
pub mod menus;
pub mod mixer;
pub mod shortcuts;
pub mod theme;

use soundcraft_engine::{Engine, Tool};
use std::sync::{Arc, Mutex};

/// Application state container managing the Martensite GUI pipeline.
pub struct SoundcraftApp {
    pub engine: Arc<Mutex<Engine>>,
    pub theme: theme::CraftTheme,
    pub keyboard: shortcuts::KeyboardEngine,
    pub mixer: mixer::MixerState,
    pub active_tool: Tool,
    /// Timeline horizontal zoom (pixels-per-second scale factor).
    pub zoom_level: f32,
    /// Timeline/track scroll offset in pixels.
    pub scroll_offset: [f32; 2],
    pub is_playing: bool,
    pub is_recording: bool,
    pub tempo_bpm: f32,
    /// Playhead position in samples at the session rate.
    pub playhead_samples: u64,
    pub loop_enabled: bool,
    pub rulers_visible: bool,
    pub is_dirty: bool,
}

impl SoundcraftApp {
    pub fn new(engine: Engine) -> Self {
        Self {
            engine: Arc::new(Mutex::new(engine)),
            theme: theme::CraftTheme::console_dark(),
            keyboard: shortcuts::KeyboardEngine::new(),
            mixer: mixer::MixerState::new_stereo(),
            active_tool: Tool::Select,
            zoom_level: 1.0,
            scroll_offset: [0.0, 0.0],
            is_playing: false,
            is_recording: false,
            tempo_bpm: 120.0,
            playhead_samples: 0,
            loop_enabled: false,
            rulers_visible: true,
            is_dirty: false,
        }
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.active_tool = tool;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom_level = zoom.clamp(0.01, 256.0);
    }

    pub fn scroll_by(&mut self, dx: f32, dy: f32) {
        self.scroll_offset[0] += dx;
        self.scroll_offset[1] += dy;
    }

    pub fn reset_view(&mut self) {
        self.zoom_level = 1.0;
        self.scroll_offset = [0.0, 0.0];
    }

    /// Transport toggle (the Space key dispatches the `transport.toggle`
    /// command above this layer; this mirrors the resulting state).
    pub fn toggle_playback(&mut self) -> bool {
        self.is_playing = !self.is_playing;
        self.is_playing
    }

    pub fn toggle_record(&mut self) -> bool {
        self.is_recording = !self.is_recording;
        self.is_recording
    }

    pub fn toggle_loop(&mut self) -> bool {
        self.loop_enabled = !self.loop_enabled;
        self.loop_enabled
    }

    pub fn set_tempo(&mut self, bpm: f32) {
        self.tempo_bpm = bpm.clamp(20.0, 999.0);
    }

    pub fn set_playhead_samples(&mut self, samples: u64) {
        self.playhead_samples = samples;
    }

    pub fn toggle_rulers(&mut self) -> bool {
        self.rulers_visible = !self.rulers_visible;
        self.rulers_visible
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> SoundcraftApp {
        SoundcraftApp::new(Engine::default())
    }

    #[test]
    fn test_app_initialization() {
        let app = app();
        assert_eq!(app.active_tool, Tool::Select);
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.scroll_offset, [0.0, 0.0]);
        assert!(!app.is_playing);
        assert!(!app.is_recording);
        assert_eq!(app.tempo_bpm, 120.0);
        assert_eq!(app.playhead_samples, 0);
        assert!(app.rulers_visible);
        assert!(!app.is_dirty);
    }

    #[test]
    fn test_zoom_clamping() {
        let mut app = app();
        app.set_zoom(4.0);
        assert_eq!(app.zoom_level, 4.0);
        app.set_zoom(0.0001);
        assert_eq!(app.zoom_level, 0.01);
        app.set_zoom(1000.0);
        assert_eq!(app.zoom_level, 256.0);
    }

    #[test]
    fn test_scroll_and_reset() {
        let mut app = app();
        app.scroll_by(320.0, -48.0);
        assert_eq!(app.scroll_offset, [320.0, -48.0]);
        app.set_zoom(8.0);
        app.reset_view();
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.scroll_offset, [0.0, 0.0]);
    }

    #[test]
    fn test_transport_toggles() {
        let mut app = app();
        assert!(app.toggle_playback());
        assert!(app.is_playing);
        assert!(!app.toggle_playback());
        assert!(app.toggle_record());
        assert!(app.is_recording);
        assert!(app.toggle_loop());
        assert!(app.loop_enabled);
    }

    #[test]
    fn test_tempo_and_playhead() {
        let mut app = app();
        app.set_tempo(140.0);
        assert_eq!(app.tempo_bpm, 140.0);
        app.set_tempo(5.0);
        assert_eq!(app.tempo_bpm, 20.0);
        app.set_playhead_samples(96000);
        assert_eq!(app.playhead_samples, 96000);
    }
}
