//! DAW integration tests.

use soundcraft_ui_martensite::SoundcraftApp;

#[test]
fn test_daw_workflow() {
    let mut app = SoundcraftApp::new();
    assert_eq!(app.mixer.channels.len(), 1);

    app.mixer.set_channel_volume(0, -3.0);
    assert_eq!(app.mixer.channels[0].volume_db, -3.0);

    app.toggle_playback();
    assert!(app.is_playing);
}
