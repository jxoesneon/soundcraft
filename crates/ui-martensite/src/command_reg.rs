//! DAW command registry.

pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
}

pub const COMMANDS: &[Command] = &[
    Command { id: "transport.play", label: "Play / Stop", shortcut: Some("Space") },
    Command { id: "transport.record", label: "Record", shortcut: Some("Cmd+Space") },
    Command { id: "track.new_audio", label: "New Audio Track…", shortcut: Some("Shift+Cmd+N") },
];
