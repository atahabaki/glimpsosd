use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Configuration {
    duration: u64,
    positioning: Positioning,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Positioning {
    anchor: Anchor,
    top_margin: Margin,
    bottom_margin: Margin,
    left_margin: Margin,
    right_margin: Margin,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Anchor {
    Top,
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
