use serde::{Deserialize, Serialize};

use super::event::Event;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Configuration {
    pub duration: u64,
    pub positioning: Positioning,
    pub osdtext: OsdText,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Positioning {
    pub anchor: Anchor,
    pub margin: Option<i32>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) enum Anchor {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

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
    pub battery_text: BatteryText,
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

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct BatteryText {
    pub present_charged_text: String,
    pub present_empty_text: String,
    pub present_charging_state_text: ([String; 10], String),
    pub present_discharging_state_text: ([String; 10], String),
    pub present_other_state_text: BatteryOtherStateText,
    pub removed_state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct BatteryOtherStateText {
    pub unknown: String,
    pub pending_charge: String,
    pub pending_discharge: String,
}

impl Default for BatteryOtherStateText {
    fn default() -> Self {
        BatteryOtherStateText {
            unknown: "Unknown".to_owned(),
            pending_charge: "Pending Charge".to_owned(),
            pending_discharge: "Pending Discharge".to_owned(),
        }
    }
}

impl Default for BatteryText {
    fn default() -> Self {
        let discharging_icons = [
            "󰁺 Discharging".to_string(),
            "󰁻 Discharging".to_string(),
            "󰁼 Discharging".to_string(),
            "󰁽 Discharging".to_string(),
            "󰁾 Discharging".to_string(),
            "󰁿 Discharging".to_string(),
            "󰂀 Discharging".to_string(),
            "󰂁 Discharging".to_string(),
            "󰂂 Discharging".to_string(),
            "󰁹 Discharging".to_string(),
        ];
        let charging_icons = [
            "󰢜  Charging".to_string(),
            "󰂆  Charging".to_string(),
            "󰂇  Charging".to_string(),
            "󰂈  Charging".to_string(),
            "󰢝  Charging".to_string(),
            "󰂉  Charging".to_string(),
            "󰢞  Charging".to_string(),
            "󰂊  Charging".to_string(),
            "󰂋  Charging".to_string(),
            "󰂅  Charging".to_string(),
        ];
        BatteryText {
            present_charged_text: "󰁹".to_owned(),
            present_empty_text: "󰁺".to_owned(),
            present_charging_state_text: (charging_icons, "󰂑 Charging".to_owned()),
            present_discharging_state_text: (discharging_icons, "󰂑 Discharging".to_owned()),
            present_other_state_text: BatteryOtherStateText::default(),
            removed_state: "󱟨".to_owned(),
        }
    }
}

impl BatteryText {
    pub(crate) fn _get_based_on_new_battery_status(&self, event: &Event) -> String {
        match event {
            Event::Battery {
                is_present,
                state,
                percentage,
            } => match is_present {
                true => match state {
                    1 => match percentage {
                        0_f64..10_f64 => self
                            .present_charging_state_text
                            .0
                            .get(0)
                            .unwrap()
                            .to_owned(),
                        10_f64..20_f64 => self
                            .present_charging_state_text
                            .0
                            .get(1)
                            .unwrap()
                            .to_owned(),
                        20_f64..30_f64 => self
                            .present_charging_state_text
                            .0
                            .get(2)
                            .unwrap()
                            .to_owned(),
                        30_f64..40_f64 => self
                            .present_charging_state_text
                            .0
                            .get(3)
                            .unwrap()
                            .to_owned(),
                        40_f64..50_f64 => self
                            .present_charging_state_text
                            .0
                            .get(4)
                            .unwrap()
                            .to_owned(),
                        50_f64..60_f64 => self
                            .present_charging_state_text
                            .0
                            .get(5)
                            .unwrap()
                            .to_owned(),
                        60_f64..70_f64 => self
                            .present_charging_state_text
                            .0
                            .get(6)
                            .unwrap()
                            .to_owned(),
                        70_f64..80_f64 => self
                            .present_charging_state_text
                            .0
                            .get(7)
                            .unwrap()
                            .to_owned(),
                        80_f64..90_f64 => self
                            .present_charging_state_text
                            .0
                            .get(8)
                            .unwrap()
                            .to_owned(),
                        90_f64..100_f64 => self
                            .present_charging_state_text
                            .0
                            .get(9)
                            .unwrap()
                            .to_owned(),
                        _ => self.present_charging_state_text.1.to_owned(),
                    },
                    2 => match percentage {
                        0_f64..10_f64 => self
                            .present_discharging_state_text
                            .0
                            .get(0)
                            .unwrap()
                            .to_owned(),
                        10_f64..20_f64 => self
                            .present_discharging_state_text
                            .0
                            .get(1)
                            .unwrap()
                            .to_owned(),
                        20_f64..30_f64 => self
                            .present_discharging_state_text
                            .0
                            .get(2)
                            .unwrap()
                            .to_owned(),
                        30_f64..40_f64 => self
                            .present_discharging_state_text
                            .0
                            .get(3)
                            .unwrap()
                            .to_owned(),
                        40_f64..50_f64 => self
                            .present_discharging_state_text
                            .0
                            .get(4)
                            .unwrap()
                            .to_owned(),
                        50_f64..60_f64 => self
                            .present_discharging_state_text
                            .0
                            .get(5)
                            .unwrap()
                            .to_owned(),
                        60_f64..70_f64 => self
                            .present_discharging_state_text
                            .0
                            .get(6)
                            .unwrap()
                            .to_owned(),
                        70_f64..80_f64 => self
                            .present_discharging_state_text
                            .0
                            .get(7)
                            .unwrap()
                            .to_owned(),
                        80_f64..90_f64 => self
                            .present_discharging_state_text
                            .0
                            .get(8)
                            .unwrap()
                            .to_owned(),
                        90_f64..100_f64 => self
                            .present_discharging_state_text
                            .0
                            .get(9)
                            .unwrap()
                            .to_owned(),
                        _ => self.present_discharging_state_text.1.to_owned(),
                    },
                    3 => self.present_empty_text.to_owned(),
                    4 => self.present_charged_text.to_owned(),
                    5 => self.present_other_state_text.pending_charge.to_owned(),
                    6 => self.present_other_state_text.pending_discharge.to_owned(),
                    _ => self.present_other_state_text.unknown.to_owned(),
                },
                false => self.removed_state.to_owned(),
            },
            _ => unreachable!("Only call on event Battery"),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            duration: 500,
            positioning: Positioning {
                anchor: Anchor::default(),
                margin: Some(50),
            },
            osdtext: OsdText {
                power_profile_text: PowerProfileText::default(),
                battery_text: BatteryText::default(),
            },
        }
    }
}
