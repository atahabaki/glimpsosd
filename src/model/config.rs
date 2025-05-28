use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Configuration {
    duration: u64,
    positioning: Positioning,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Positioning {
    anchor: Anchor,
    top_margin: Option<Margin>,
    bottom_margin: Option<Margin>,
    left_margin: Option<Margin>,
    right_margin: Option<Margin>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) enum Anchor {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

pub(crate) type Margin = (Anchor, i32);

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

impl Default for Configuration {
    fn default() -> Self {
        Self {
            duration: 500,
            positioning: Positioning {
                anchor: Anchor::default(),
                top_margin: None,
                bottom_margin: Some((Anchor::default(), 50)),
                left_margin: None,
                right_margin: None,
            },
        }
    }
}
