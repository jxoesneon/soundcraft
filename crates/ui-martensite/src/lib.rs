//! Sovereign retained-mode DAW interface for SoundCraft built on Martensite.

pub mod command_reg;
pub mod menus;
pub mod mixer;
pub mod theme;

pub struct SoundcraftApp {
    pub mixer: mixer::MixerState,
    pub is_recording: bool,
    pub is_playing: bool,
    pub tempo_bpm: f32,
}

impl SoundcraftApp {
    pub fn new() -> Self {
        Self {
            mixer: mixer::MixerState::new_stereo(),
            is_recording: false,
            is_playing: false,
            tempo_bpm: 120.0,
        }
    }

    pub fn toggle_playback(&mut self) -> bool {
        self.is_playing = !self.is_playing;
        self.is_playing
    }

    pub fn set_tempo(&mut self, bpm: f32) {
        self.tempo_bpm = bpm.clamp(20.0, 999.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soundcraft_app() {
        let mut app = SoundcraftApp::new();
        assert!(!app.is_playing);
        assert!(app.toggle_playback());
        assert!(app.is_playing);
        app.set_tempo(140.0);
        assert_eq!(app.tempo_bpm, 140.0);
    }
}
