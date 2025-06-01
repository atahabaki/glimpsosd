use std::{error::Error, fs::File, io::Read, path::PathBuf};

use clap::{Parser, command};

use crate::{GlimpsOSD, daemon::OSD_CSS, model::config::Configuration};

#[derive(Parser)]
#[command(about, version)]
pub(crate) struct Cli {
    /// Use this style.css file instead.
    /// By default, glimpsosd uses XDG_CONFIG_HOME/glimpsosd/style.css
    #[arg(short, long, value_name = "CSS_FILE")]
    pub style: Option<PathBuf>,
    /// Use this config.ron file instead.
    /// By default, glimpsosd uses XDG_CONFIG_HOME/glimpsosd/config.ron
    #[arg(short, long, value_name = "RON_FILE")]
    pub config: Option<PathBuf>,
    /// Useful for no default styling or configuration
    /// By default when error occured when reading style/config use builtins.
    /// This argument disables that.
    #[arg(short, long)]
    pub no_fallback: bool,
}

/// Temporary Config Holder
enum _ConfigCup {
    FromFile(String),
    Default,
}

impl GlimpsOSD {
    fn _read_file_contents(path: PathBuf) -> Result<String, Box<dyn Error>> {
        let mut buf = String::new();
        // file get it? XD
        let mut phile = File::open(path)?;
        phile.read_to_string(&mut buf)?;
        Ok(buf)
    }
    fn _find_xdg_config_home() -> Result<Option<PathBuf>, Box<dyn Error>> {
        let use_builtin = Ok(None);
        match std::env::var("XDG_CONFIG_HOME") {
            Ok(xdg_env_path)
                if PathBuf::from(xdg_env_path.clone())
                    .join("glimpsosd")
                    .exists() =>
            {
                Ok(Some(PathBuf::from(xdg_env_path)))
            }
            Ok(_) => match std::env::var("HOME") {
                Ok(home_path) => {
                    let xdg_config_home_path =
                        PathBuf::from(home_path).join(".config").join("glimpsosd");
                    match xdg_config_home_path.exists() {
                        true => Ok(Some(xdg_config_home_path)),
                        false => use_builtin,
                    }
                }
                Err(_) => use_builtin,
            },
            Err(_) => use_builtin,
        }
    }
    fn _get_style_from_cli(cli: &Cli) -> String {
        match &cli.style {
            Some(path) => match GlimpsOSD::_read_file_contents(path.clone()) {
                Ok(contents) => contents,
                Err(e) if cli.no_fallback => panic!(
                    "Couldn't read file located at {:?} cause: {}",
                    path.clone(),
                    e
                ),
                Err(e) => {
                    eprintln!("Couldn't read file located at {:?} cause: {}", path, e);
                    match GlimpsOSD::_find_xdg_config_home() {
                        Ok(path) => match path {
                            Some(path) => {
                                let path = path.join("style.css");
                                match path.exists() {
                                    true => match GlimpsOSD::_read_file_contents(path.clone()) {
                                        Ok(contents) => contents,
                                        Err(e) => {
                                            eprintln!(
                                                "Couldn't read file located at {:?} cause: {}",
                                                path, e
                                            );
                                            OSD_CSS.to_owned()
                                        }
                                    },
                                    false => OSD_CSS.to_owned(),
                                }
                            }
                            None => OSD_CSS.to_owned(),
                        },
                        Err(_) => OSD_CSS.to_owned(),
                    }
                }
            },
            None => match GlimpsOSD::_find_xdg_config_home() {
                Ok(path) => match path {
                    Some(path) => {
                        let path = path.join("style.css");
                        match path.exists() {
                            true => match GlimpsOSD::_read_file_contents(path.clone()) {
                                Ok(contents) => contents,
                                Err(e) if cli.no_fallback => {
                                    panic!("Couldn't read file located at {:?} cause: {}", path, e)
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Couldn't read file located at {:?} cause: {}",
                                        path, e
                                    );
                                    OSD_CSS.to_owned()
                                }
                            },
                            false => OSD_CSS.to_owned(),
                        }
                    }
                    None => OSD_CSS.to_owned(),
                },
                Err(e) if cli.no_fallback => panic!("{:?}", e),
                Err(_) => OSD_CSS.to_owned(),
            },
        }
    }
    fn _get_config_from_cli(cli: &Cli) -> _ConfigCup {
        match &cli.config {
            Some(path) => match GlimpsOSD::_read_file_contents(path.clone()) {
                Ok(contents) => _ConfigCup::FromFile(contents),
                Err(e) if cli.no_fallback => panic!(
                    "Couldn't read file located at {:?} cause: {}",
                    path.clone(),
                    e
                ),
                Err(e) => {
                    eprintln!("Couldn't read file located at {:?} cause: {}", path, e);
                    match GlimpsOSD::_find_xdg_config_home() {
                        Ok(path) => match path {
                            Some(path) => {
                                let path = path.join("config.ron");
                                match path.exists() {
                                    true => match GlimpsOSD::_read_file_contents(path.clone()) {
                                        Ok(contents) => _ConfigCup::FromFile(contents),
                                        Err(e) => {
                                            eprintln!(
                                                "Couldn't read file located at {:?} cause: {}",
                                                path, e
                                            );
                                            _ConfigCup::Default
                                        }
                                    },
                                    false => _ConfigCup::Default,
                                }
                            }
                            None => _ConfigCup::Default,
                        },
                        Err(_) => _ConfigCup::Default,
                    }
                }
            },
            None => match GlimpsOSD::_find_xdg_config_home() {
                Ok(path) => match path {
                    Some(path) => {
                        let path = path.join("config.ron");
                        match path.exists() {
                            true => match GlimpsOSD::_read_file_contents(path.clone()) {
                                Ok(contents) => _ConfigCup::FromFile(contents),
                                Err(e) if cli.no_fallback => {
                                    panic!("Couldn't read file located at {:?} cause: {}", path, e)
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Couldn't read file located at {:?} cause: {}",
                                        path, e
                                    );
                                    _ConfigCup::Default
                                }
                            },
                            false => _ConfigCup::Default,
                        }
                    }
                    None => _ConfigCup::Default,
                },
                Err(e) if cli.no_fallback => panic!("{:?}", e),
                Err(_) => _ConfigCup::Default,
            },
        }
    }
    pub(crate) fn from_cli(cli: Cli) -> Self {
        let style = GlimpsOSD::_get_style_from_cli(&cli);
        let config = GlimpsOSD::_get_config_from_cli(&cli);
        let config = match config {
            _ConfigCup::FromFile(contents) => match ron::from_str::<Configuration>(&contents) {
                Ok(config) => config,
                Err(e) if cli.no_fallback => panic!("Parsing failed by ron: {e}"),
                Err(e) => {
                    eprintln!("Parsing failed by ron: {e}");
                    Configuration::default()
                }
            },
            _ConfigCup::Default => Configuration::default(),
        };
        GlimpsOSD { style, config }
    }
}
