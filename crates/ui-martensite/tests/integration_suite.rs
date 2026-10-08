//! DAW integration tests.

use soundcraft_engine::{Engine, Tool};
use soundcraft_ui_martensite::SoundcraftApp;
use soundcraft_ui_martensite::command_reg::{CommandCategory, commands_by_category, find_command};
use soundcraft_ui_martensite::menus::generate_main_menu;

#[test]
fn test_daw_workflow() {
    let mut app = SoundcraftApp::new(Engine::default());
    assert_eq!(app.mixer.channels.len(), 1);

    app.mixer.set_channel_volume(0, -3.0);
    assert_eq!(app.mixer.channels[0].volume_db, -3.0);

    app.toggle_playback();
    assert!(app.is_playing);

    app.set_tool(Tool::Razor);
    assert_eq!(app.active_tool, Tool::Razor);
}

#[test]
fn test_registry_commands_resolve_in_engine() {
    let engine = Engine::default();
    let _ = &engine;
    for cmd in commands_by_category(CommandCategory::Transport) {
        assert!(find_command(cmd.id).is_some());
    }
}

#[test]
fn test_menu_tree_dispatches_engine_ids() {
    for cat in generate_main_menu() {
        for item in cat.items {
            if let Some(id) = item.command_id {
                assert!(soundcraft_engine::find_command(id).is_some(), "menu id {id} not in engine");
            }
        }
    }
}
