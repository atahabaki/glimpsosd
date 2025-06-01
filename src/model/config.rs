use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Configuration {
    pub duration: u64,
    pub positioning: Positioning,
    pub osdtext: OsdText,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Positioning {
    pub anchor: Anchor,
    pub top_margin: Option<Margin>,
    pub bottom_margin: Option<Margin>,
    pub left_margin: Option<Margin>,
    pub right_margin: Option<Margin>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) enum Anchor {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

pub(crate) type Margin = i32;

impl From<Anchor> for gtkls::Edge {
    fn from(value: Anchor) -> Self {
        match value {
            Anchor::Top => gtkls::Edge::Top,
            Anchor::Bottom => gtkls::Edge::Bottom,
            Anchor::Left => gtkls::Edge::Left,
            Anchor::Right => gtkls::Edge::Right,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct OsdText {
    pub power_profile_text: PowerProfileText,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PowerProfileText {
    pub power_saver: String,
    pub balanced: String,
    pub performance: String,
}

impl PowerProfileText {
    pub(crate) fn _get_based_on_new_profile_text(&self, new_profile: &str) -> String {
        match new_profile {
            "power-saver" => self.power_saver.clone(),
            "balanced" => self.balanced.clone(),
            "performance" => self.performance.clone(),
            _ => "".into(),
        }
    }
}

impl Default for PowerProfileText {
    fn default() -> Self {
        PowerProfileText {
            power_saver: "  Power-Saver".to_owned(),
            balanced: "  Balanced".to_owned(),
            performance: " Performance".to_owned(),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            duration: 500,
            positioning: Positioning {
                anchor: Anchor::default(),
                top_margin: None,
                bottom_margin: Some(50),
                left_margin: None,
                right_margin: None,
            },
            osdtext: OsdText {
                power_profile_text: PowerProfileText::default(),
            },
        }
    }
}
