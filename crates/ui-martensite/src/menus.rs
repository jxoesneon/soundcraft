//! DAW menus.

pub struct MenuCategory {
    pub title: &'static str,
    pub items: &'static [&'static str],
}

pub const MENUS: &[MenuCategory] = &[
    MenuCategory { title: "Track", items: &["track.new_audio"] },
    MenuCategory { title: "Transport", items: &["transport.play", "transport.record"] },
];
