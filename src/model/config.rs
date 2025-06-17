use serde::Deserialize;

use super::event::Event;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Configuration {
    pub duration: u64,
    pub positioning: Positioning,
    pub osd: OsdText,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Positioning {
    pub anchor: Anchor,
    pub margin: Option<i32>,
}

#[derive(Debug, Default, Deserialize)]
pub enum Anchor {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

impl From<Anchor> for gtkls::Edge {
    fn from(value: Anchor) -> Self {
        match value {
            Anchor::Top => Self::Top,
            Anchor::Bottom => Self::Bottom,
            Anchor::Left => Self::Left,
            Anchor::Right => Self::Right,
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct OsdText {
    pub power_profile: PowerProfileText,
    pub battery: BatteryText,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct PowerProfileText {
    pub power_saver: String,
    pub balanced: String,
    pub performance: String,
}

impl PowerProfileText {
    #[allow(dead_code)]
    pub(crate) fn get_based_on_new_profile(&self, new_profile: &str) -> String {
        match new_profile {
            "power-saver" => self.power_saver.clone(),
            "balanced" => self.balanced.clone(),
            "performance" => self.performance.clone(),
            _ => String::new(),
        }
    }
}

impl Default for PowerProfileText {
    fn default() -> Self {
        Self {
            power_saver: "  Power-Saver".to_owned(),
            balanced: "  Balanced".to_owned(),
            performance: " Performance".to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct BatteryText {
    pub present_charged: String,
    pub present_empty: String,
    pub present_charging: ([String; 10], String),
    pub present_discharging: ([String; 10], String),
    pub present_pending_charge: ([String; 10], String),
    pub present_pending_discharge: ([String; 10], String),
    pub removed: String,
    pub unknown: String,
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
        Self {
            present_charged: "󰁹".to_owned(),
            present_empty: "󰁺".to_owned(),
            present_charging: (charging_icons, "󰂑 Charging".to_owned()),
            present_discharging: (discharging_icons, "󰂑 Discharging".to_owned()),
            present_pending_charge: (pending_charge_icons, "󰂑 Pending Charge".to_owned()),
            present_pending_discharge: (pending_discharge_icons, "󰂑 Pending Discharge".to_owned()),
            removed: "󱟨".to_owned(),
            unknown: "󰂑".to_owned(),
        }
    }
}

impl BatteryText {
    #[allow(dead_code)]
    pub(crate) fn get_based_on_new_battery_status(&self, event: &Event) -> String {
        match event {
            Event::Battery {
                is_present,
                state,
                percentage,
            } => match is_present {
                true => match state {
                    1 | 2 | 5 | 6 => {
                        let number = percentage.map_or(None, |p| {
                            if p <= 0_f64 || p >= 100_f64 {
                                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                                return Some((p / 100_f64) as usize);
                            }
                            None
                        });
                        match number {
                            Some(number) if state == &1 => {
                                self.present_charging.0.get(number).unwrap().to_owned()
                            }
                            Some(number) if state == &2 => {
                                self.present_discharging.0.get(number).unwrap().to_owned()
                            }
                            Some(number) if state == &5 => self
                                .present_pending_charge
                                .0
                                .get(number)
                                .unwrap()
                                .to_owned(),
                            Some(number) => self
                                .present_pending_discharge
                                .0
                                .get(number)
                                .unwrap()
                                .to_owned(),
                            None if state == &1 => self.present_charging.1.clone(),
                            None if state == &2 => self.present_discharging.1.clone(),
                            None if state == &5 => self.present_pending_charge.1.clone(),
                            None => self.present_pending_discharge.1.clone(),
                        }
                    }
                    3 => self.present_empty.clone(),
                    4 => self.present_charged.clone(),
                    _ => self.unknown.clone(),
                },
                false => self.removed.clone(),
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
            osd: OsdText {
                power_profile: PowerProfileText::default(),
                battery: BatteryText::default(),
            },
        }
    }
}
