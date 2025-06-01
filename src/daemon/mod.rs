pub(crate) mod app;
pub(crate) mod cli;

pub(crate) const APP_ID: &str = "dev.atahabaki.glimpsosd";
/// I need to find a way to include this without relative paths
pub(crate) const OSD_CSS: &str = include_str!("../../examples/style.css");
