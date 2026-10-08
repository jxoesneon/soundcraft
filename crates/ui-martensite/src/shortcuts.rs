//! Keystroke state machine for edit-window tool switching.
//!
//! Single keys select tools; `Z` is a spring-loaded Zoom. `Space` is *not* a
//! tool key in a DAW — the front-end dispatches `transport.toggle` for it.

use soundcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub cmd: bool,
}

impl KeyModifiers {
    pub const fn empty() -> Self {
        Self { shift: false, ctrl: false, alt: false, cmd: false }
    }
}

pub struct KeyboardEngine {
    pub prior_tool: Option<Tool>,
    pub z_held: bool,
    pub alt_held: bool,
}

impl KeyboardEngine {
    pub fn new() -> Self {
        Self { prior_tool: None, z_held: false, alt_held: false }
    }

    pub fn on_key_down(&mut self, key: &str, current: Tool) -> Option<Tool> {
        match key {
            "z" | "Z" if !self.z_held => {
                self.z_held = true;
                self.prior_tool = Some(current);
                Some(Tool::Zoom)
            }
            "Alt" => {
                self.alt_held = true;
                None
            }
            // Single-key edit tools.
            "v" | "V" => Some(Tool::Select),
            "g" | "G" => Some(Tool::Grab),
            "t" | "T" => Some(Tool::Trim),
            "f" | "F" => Some(Tool::Fade),
            "l" | "L" => Some(Tool::Slip),
            "b" | "B" => Some(Tool::Razor),
            "p" | "P" => Some(Tool::Draw),
            "m" | "M" => Some(Tool::Mute),
            "s" | "S" => Some(Tool::Scrub),
            "h" | "H" => Some(Tool::Hand),
            "x" | "X" => Some(Tool::Smart),
            _ => None,
        }
    }

    pub fn on_key_up(&mut self, key: &str) -> Option<Tool> {
        match key {
            "z" | "Z" if self.z_held => {
                self.z_held = false;
                self.prior_tool.take()
            }
            "Alt" => {
                self.alt_held = false;
                None
            }
            _ => None,
        }
    }
}

impl Default for KeyboardEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_key_tool_switching() {
        let mut k = KeyboardEngine::new();
        assert_eq!(k.on_key_down("v", Tool::Draw), Some(Tool::Select));
        assert_eq!(k.on_key_down("B", Tool::Select), Some(Tool::Razor));
        assert_eq!(k.on_key_down("t", Tool::Razor), Some(Tool::Trim));
        assert_eq!(k.on_key_down("f", Tool::Trim), Some(Tool::Fade));
        assert_eq!(k.on_key_down("g", Tool::Fade), Some(Tool::Grab));
        assert_eq!(k.on_key_down("l", Tool::Grab), Some(Tool::Slip));
        assert_eq!(k.on_key_down("p", Tool::Slip), Some(Tool::Draw));
        assert_eq!(k.on_key_down("m", Tool::Draw), Some(Tool::Mute));
        assert_eq!(k.on_key_down("s", Tool::Mute), Some(Tool::Scrub));
        assert_eq!(k.on_key_down("h", Tool::Scrub), Some(Tool::Hand));
        assert_eq!(k.on_key_down("x", Tool::Hand), Some(Tool::Smart));
    }

    #[test]
    fn test_spring_loaded_zoom_tool() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::Select;

        assert_eq!(k.on_key_down("z", initial), Some(Tool::Zoom));
        assert!(k.z_held);

        // Repeat key-down events don't clobber the saved tool.
        assert_eq!(k.on_key_down("z", Tool::Zoom), None);

        assert_eq!(k.on_key_up("z"), Some(initial));
        assert!(!k.z_held);
    }

    #[test]
    fn test_space_is_not_a_tool_key() {
        let mut k = KeyboardEngine::new();
        assert_eq!(k.on_key_down("Space", Tool::Select), None);
        assert_eq!(k.on_key_up("Space"), None);
    }
}
