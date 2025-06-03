use serde::Deserialize;

use super::event::Event;

#[derive(Debug, Deserialize)]
pub(crate) struct Configuration {
    pub _duration: u64,
    pub _positioning: Positioning,
    pub _osdtext: OsdText,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Positioning {
    pub _anchor: Anchor,
    pub _margin: Option<i32>,
}

#[derive(Debug, Default, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub(crate) struct OsdText {
    pub _power_profile_text: PowerProfileText,
    pub _battery_text: BatteryText,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PowerProfileText {
    pub _power_saver: String,
    pub _balanced: String,
    pub _performance: String,
}

impl PowerProfileText {
    pub(crate) fn _get_based_on_new_profile_text(&self, new_profile: &str) -> String {
        match new_profile {
            "power-saver" => self._power_saver.clone(),
            "balanced" => self._balanced.clone(),
            "performance" => self._performance.clone(),
            _ => "".into(),
        }
    }
}

impl Default for PowerProfileText {
    fn default() -> Self {
        PowerProfileText {
            _power_saver: "  Power-Saver".to_owned(),
            _balanced: "  Balanced".to_owned(),
            _performance: " Performance".to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct BatteryText {
    pub _present_charged_text: String,
    pub _present_empty_text: String,
    pub _present_charging_state_text: ([String; 10], String),
    pub _present_discharging_state_text: ([String; 10], String),
    pub _present_pending_charge_state_text: ([String; 10], String),
    pub _present_pending_discharge_state_text: ([String; 10], String),
    pub _removed_state_text: String,
    pub _unknown_state_text: String,
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
        let pending_charge_icons = [
            "󰁺 Pending Charge".to_string(),
            "󰁻 Pending Charge".to_string(),
            "󰁼 Pending Charge".to_string(),
            "󰁽 Pending Charge".to_string(),
            "󰁾 Pending Charge".to_string(),
            "󰁿 Pending Charge".to_string(),
            "󰂀 Pending Charge".to_string(),
            "󰂁 Pending Charge".to_string(),
            "󰂂 Pending Charge".to_string(),
            "󰁹 Pending Charge".to_string(),
        ];
        let pending_discharge_icons = [
            "󰁺 Pending Discharge".to_string(),
            "󰁻 Pending Discharge".to_string(),
            "󰁼 Pending Discharge".to_string(),
            "󰁽 Pending Discharge".to_string(),
            "󰁾 Pending Discharge".to_string(),
            "󰁿 Pending Discharge".to_string(),
            "󰂀 Pending Discharge".to_string(),
            "󰂁 Pending Discharge".to_string(),
            "󰂂 Pending Discharge".to_string(),
            "󰁹 Pending Discharge".to_string(),
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
            _present_charged_text: "󰁹".to_owned(),
            _present_empty_text: "󰁺".to_owned(),
            _present_charging_state_text: (charging_icons, "󰂑 Charging".to_owned()),
            _present_discharging_state_text: (discharging_icons, "󰂑 Discharging".to_owned()),
            _present_pending_charge_state_text: (
                pending_charge_icons,
                "󰂑 Pending Charge".to_owned(),
            ),
            _present_pending_discharge_state_text: (
                pending_discharge_icons,
                "󰂑 Pending Discharge".to_owned(),
            ),
            _removed_state_text: "󱟨".to_owned(),
            _unknown_state_text: "󰂑".to_owned(),
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
                    1 | 2 | 5 | 6 => {
                        let number = match percentage {
                            0_f64..10_f64 => Some(0),
                            10_f64..20_f64 => Some(1),
                            20_f64..30_f64 => Some(2),
                            30_f64..40_f64 => Some(3),
                            40_f64..50_f64 => Some(4),
                            50_f64..60_f64 => Some(5),
                            60_f64..70_f64 => Some(6),
                            70_f64..80_f64 => Some(7),
                            80_f64..90_f64 => Some(8),
                            90_f64..100_f64 => Some(9),
                            _ => None,
                        };
                        match number {
                            Some(number) if state == &1 => self
                                ._present_charging_state_text
                                .0
                                .get(number)
                                .unwrap()
                                .to_owned(),
                            Some(number) if state == &2 => self
                                ._present_discharging_state_text
                                .0
                                .get(number)
                                .unwrap()
                                .to_owned(),
                            Some(number) if state == &5 => self
                                ._present_pending_charge_state_text
                                .0
                                .get(number)
                                .unwrap()
                                .to_owned(),
                            Some(number) => self
                                ._present_pending_discharge_state_text
                                .0
                                .get(number)
                                .unwrap()
                                .to_owned(),
                            None if state == &1 => self._present_charging_state_text.1.to_owned(),
                            None if state == &2 => {
                                self._present_discharging_state_text.1.to_owned()
                            }
                            None if state == &5 => {
                                self._present_pending_charge_state_text.1.to_owned()
                            }
                            None => self._present_pending_discharge_state_text.1.to_owned(),
                        }
                    }
                    3 => self._present_empty_text.to_owned(),
                    4 => self._present_charged_text.to_owned(),
                    _ => self._unknown_state_text.to_owned(),
                },
                false => self._removed_state_text.to_owned(),
            },
            _ => unreachable!("Only call on event Battery"),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            _duration: 500,
            _positioning: Positioning {
                _anchor: Anchor::default(),
                _margin: Some(50),
            },
            _osdtext: OsdText {
                _power_profile_text: PowerProfileText::default(),
                _battery_text: BatteryText::default(),
            },
        }
    }
}
