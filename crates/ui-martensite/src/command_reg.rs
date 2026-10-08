//! Decoupled command catalog and taxonomy for SoundCraft.
//!
//! Ids match the engine's command registry (`soundcraft_engine::command_specs`)
//! so every front-end dispatches the same commands by id.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandCategory {
    File,
    Edit,
    Clip,
    Track,
    Transport,
    Mix,
    View,
    Window,
    Help,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandSpec {
    pub id: &'static str,
    pub label: &'static str,
    pub category: CommandCategory,
    pub default_shortcut: Option<&'static str>,
    pub secondary_shortcut: Option<&'static str>,
}

pub const COMMAND_REGISTRY: &[CommandSpec] = &[
    // File
    CommandSpec {
        id: "session.new",
        label: "New Session…",
        category: CommandCategory::File,
        default_shortcut: Some("Cmd+N"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "session.open",
        label: "Open Session…",
        category: CommandCategory::File,
        default_shortcut: Some("Cmd+O"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "session.save",
        label: "Save Session",
        category: CommandCategory::File,
        default_shortcut: Some("Cmd+S"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "session.save_as",
        label: "Save Session As…",
        category: CommandCategory::File,
        default_shortcut: Some("Shift+Cmd+S"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "session.close",
        label: "Close Session",
        category: CommandCategory::File,
        default_shortcut: Some("Cmd+W"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "file.import_audio",
        label: "Import Audio…",
        category: CommandCategory::File,
        default_shortcut: Some("Shift+Cmd+I"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "file.import_midi", label: "Import MIDI…", category: CommandCategory::File, default_shortcut: None, secondary_shortcut: None
    },
    CommandSpec {
        id: "file.bounce_mix",
        label: "Bounce Mix…",
        category: CommandCategory::File,
        default_shortcut: Some("Alt+Cmd+B"),
        secondary_shortcut: None,
    },
    // Edit
    CommandSpec { id: "edit.undo", label: "Undo", category: CommandCategory::Edit, default_shortcut: Some("Cmd+Z"), secondary_shortcut: None },
    CommandSpec { id: "edit.redo", label: "Redo", category: CommandCategory::Edit, default_shortcut: Some("Shift+Cmd+Z"), secondary_shortcut: None },
    CommandSpec { id: "edit.cut", label: "Cut", category: CommandCategory::Edit, default_shortcut: Some("Cmd+X"), secondary_shortcut: None },
    CommandSpec { id: "edit.copy", label: "Copy", category: CommandCategory::Edit, default_shortcut: Some("Cmd+C"), secondary_shortcut: None },
    CommandSpec { id: "edit.paste", label: "Paste", category: CommandCategory::Edit, default_shortcut: Some("Cmd+V"), secondary_shortcut: None },
    CommandSpec { id: "edit.clear", label: "Clear", category: CommandCategory::Edit, default_shortcut: Some("Backspace"), secondary_shortcut: None },
    CommandSpec {
        id: "edit.duplicate",
        label: "Duplicate",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+D"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "edit.separate",
        label: "Separate Clip",
        category: CommandCategory::Edit,
        default_shortcut: Some("Cmd+E"),
        secondary_shortcut: None,
    },
    // Clip
    CommandSpec { id: "clip.rename", label: "Rename Clip…", category: CommandCategory::Clip, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "clip.gain", label: "Clip Gain…", category: CommandCategory::Clip, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "clip.group",
        label: "Group Clips",
        category: CommandCategory::Clip,
        default_shortcut: Some("Cmd+G"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "clip.ungroup",
        label: "Ungroup Clips",
        category: CommandCategory::Clip,
        default_shortcut: Some("Shift+Cmd+G"),
        secondary_shortcut: None,
    },
    // Track
    CommandSpec {
        id: "track.new",
        label: "New Track…",
        category: CommandCategory::Track,
        default_shortcut: Some("Shift+Cmd+N"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "track.duplicate",
        label: "Duplicate Track",
        category: CommandCategory::Track,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    CommandSpec { id: "track.delete", label: "Delete Track", category: CommandCategory::Track, default_shortcut: None, secondary_shortcut: None },
    CommandSpec { id: "mix.mute", label: "Mute", category: CommandCategory::Track, default_shortcut: Some("M"), secondary_shortcut: None },
    CommandSpec { id: "mix.solo", label: "Solo", category: CommandCategory::Track, default_shortcut: Some("S"), secondary_shortcut: None },
    CommandSpec {
        id: "mix.record_arm",
        label: "Record Arm",
        category: CommandCategory::Track,
        default_shortcut: Some("R"),
        secondary_shortcut: None,
    },
    // Transport
    CommandSpec {
        id: "transport.toggle",
        label: "Play / Stop",
        category: CommandCategory::Transport,
        default_shortcut: Some("Space"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "transport.record",
        label: "Record",
        category: CommandCategory::Transport,
        default_shortcut: Some("Cmd+Space"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "transport.rtz",
        label: "Return to Zero",
        category: CommandCategory::Transport,
        default_shortcut: Some("Enter"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "transport.go_to_end",
        label: "Go to End",
        category: CommandCategory::Transport,
        default_shortcut: Some("Alt+Enter"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "transport.rewind", label: "Rewind", category: CommandCategory::Transport, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "transport.fast_forward",
        label: "Fast Forward",
        category: CommandCategory::Transport,
        default_shortcut: None,
        secondary_shortcut: None,
    },
    // View
    CommandSpec {
        id: "view.zoom_in",
        label: "Zoom In",
        category: CommandCategory::View,
        default_shortcut: Some("Cmd+Plus"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "view.zoom_out",
        label: "Zoom Out",
        category: CommandCategory::View,
        default_shortcut: Some("Cmd+Minus"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "view.zoom_fit",
        label: "Zoom to Fit",
        category: CommandCategory::View,
        default_shortcut: Some("Cmd+0"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "view.ruler", label: "Rulers", category: CommandCategory::View, default_shortcut: Some("Alt+R"), secondary_shortcut: None },
    CommandSpec { id: "view.grid_lines", label: "Grid Lines", category: CommandCategory::View, default_shortcut: None, secondary_shortcut: None },
];

pub fn find_command(id: &str) -> Option<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().find(|cmd| cmd.id == id)
}

pub fn commands_by_category(category: CommandCategory) -> Vec<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().filter(|cmd| cmd.category == category).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_command_ids_are_unique() {
        let mut ids = HashSet::new();
        for cmd in COMMAND_REGISTRY {
            assert!(ids.insert(cmd.id), "Duplicate command ID detected: {}", cmd.id);
        }
    }

    #[test]
    fn test_lookup_finds_all_commands() {
        for cmd in COMMAND_REGISTRY {
            let found = find_command(cmd.id);
            assert!(found.is_some());
            assert_eq!(found.unwrap().label, cmd.label);
        }
    }

    #[test]
    fn test_categories_populated() {
        assert!(!commands_by_category(CommandCategory::File).is_empty());
        assert!(!commands_by_category(CommandCategory::Edit).is_empty());
        assert!(!commands_by_category(CommandCategory::Track).is_empty());
        assert!(!commands_by_category(CommandCategory::Transport).is_empty());
        assert!(!commands_by_category(CommandCategory::View).is_empty());
    }

    #[test]
    fn test_registry_ids_exist_in_engine() {
        for cmd in COMMAND_REGISTRY {
            assert!(soundcraft_engine::find_command(cmd.id).is_some(), "ui-martensite registry id not in engine: {}", cmd.id);
        }
    }
}
