#[derive(Default)]
pub struct SettingsInteraction {
    pub folding_enabled: bool,
    pub folding_depth: usize,
    pub selection_depth: i32,
}

pub struct SettingsNavigation {
    pub zoom_and_pan_enabled: bool,
    pub screen_padding: f32,
    pub zoom_speed: f32,
}

impl Default for SettingsNavigation {
    fn default() -> Self {
        Self {
            screen_padding: 0.3,
            zoom_speed: 0.1,
            zoom_and_pan_enabled: true,
        }
    }
}

pub struct SettingsStyle {
    pub edge_radius_weight: f32,
    pub folded_node_radius_weight: f32,
    pub labels_always: bool,
}

impl Default for SettingsStyle {
    fn default() -> Self {
        Self {
            edge_radius_weight: 1.,
            folded_node_radius_weight: 2.,
            labels_always: false,
        }
    }
}
