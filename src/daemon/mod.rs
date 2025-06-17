pub mod app;
pub mod cli;

pub const APP_ID: &str = "dev.atahabaki.glimpsosd";
/// I need to find a way to include this without relative paths
pub const OSD_CSS: &str = include_str!("../../examples/style.css");
