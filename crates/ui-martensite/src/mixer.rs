//! Retained-mode mixer console channel strip.

#[derive(Clone, Debug, PartialEq)]
pub struct ChannelStrip {
    pub id: u64,
    pub name: String,
    pub volume_db: f32,  // -60.0 .. +12.0 dB
    pub pan: f32,        // -100.0 (L) .. +100.0 (R)
    pub muted: bool,
    pub solo: bool,
    pub arm_record: bool,
}

pub struct MixerState {
    pub channels: Vec<ChannelStrip>,
    pub master_volume_db: f32,
}

impl MixerState {
    pub fn new_stereo() -> Self {
        Self {
            channels: vec![
                ChannelStrip {
                    id: 1,
                    name: "Audio 1".to_string(),
                    volume_db: 0.0,
                    pan: 0.0,
                    muted: false,
                    solo: false,
                    arm_record: false,
                }
            ],
            master_volume_db: 0.0,
        }
    }

    pub fn set_channel_volume(&mut self, idx: usize, db: f32) {
        if let Some(ch) = self.channels.get_mut(idx) {
            ch.volume_db = db.clamp(-60.0, 12.0);
        }
    }

    pub fn toggle_mute(&mut self, idx: usize) {
        if let Some(ch) = self.channels.get_mut(idx) {
            ch.muted = !ch.muted;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixer_controls() {
        let mut mixer = MixerState::new_stereo();
        mixer.set_channel_volume(0, -6.5);
        assert_eq!(mixer.channels[0].volume_db, -6.5);

        assert!(!mixer.channels[0].muted);
        mixer.toggle_mute(0);
        assert!(mixer.channels[0].muted);
    }
}
