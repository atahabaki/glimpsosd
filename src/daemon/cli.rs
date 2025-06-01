use std::{error::Error, fs::File, io::Read, path::PathBuf};

use clap::{Parser, command};

use crate::{daemon::OSD_CSS, model::config::Configuration};

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

enum _Parameter {
    FromFile(String),
    Default,
}

impl Cli {
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
    fn _traverse_without_fear(
        path: Option<PathBuf>,
        no_fallback: bool,
        filename: &str,
    ) -> _Parameter {
        match path {
            Some(path) => match Cli::_read_file_contents(path.clone()) {
                Ok(contents) => _Parameter::FromFile(contents),
                Err(e) if no_fallback => panic!(
                    "Couldn't read file located at {:?} cause: {}",
                    path.clone(),
                    e
                ),
                Err(e) => {
                    eprintln!("Couldn't read file located at {:?} cause: {}", path, e);
                    match Cli::_find_xdg_config_home() {
                        Ok(path) => match path {
                            Some(path) => {
                                let path = path.join(filename);
                                match path.exists() {
                                    true => match Cli::_read_file_contents(path.clone()) {
                                        Ok(contents) => _Parameter::FromFile(contents),
                                        Err(e) => {
                                            eprintln!(
                                                "Couldn't read file located at {:?} cause: {}",
                                                path, e
                                            );
                                            _Parameter::Default
                                        }
                                    },
                                    false => _Parameter::Default,
                                }
                            }
                            None => _Parameter::Default,
                        },
                        Err(_) => _Parameter::Default,
                    }
                }
            },
            None => match Cli::_find_xdg_config_home() {
                Ok(path) => match path {
                    Some(path) => {
                        let path = path.join(filename);
                        match path.exists() {
                            true => match Cli::_read_file_contents(path.clone()) {
                                Ok(contents) => _Parameter::FromFile(contents),
                                Err(e) if no_fallback => {
                                    panic!("Couldn't read file located at {:?} cause: {}", path, e)
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Couldn't read file located at {:?} cause: {}",
                                        path, e
                                    );
                                    _Parameter::Default
                                }
                            },
                            false => _Parameter::Default,
                        }
                    }
                    None => _Parameter::Default,
                },
                Err(e) if no_fallback => panic!("{:?}", e),
                Err(_) => _Parameter::Default,
            },
        }
    }
    pub(crate) fn _get_style_from_cli(cli: &Cli) -> String {
        match Cli::_traverse_without_fear(cli.style.clone(), cli.no_fallback, "style.css") {
            _Parameter::FromFile(contents) => contents,
            _Parameter::Default => OSD_CSS.to_owned(),
        }
    }
    pub(crate) fn _get_config_from_cli(cli: &Cli) -> Configuration {
        match Cli::_traverse_without_fear(cli.style.clone(), cli.no_fallback, "config.ron") {
            _Parameter::FromFile(contents) => match ron::from_str::<Configuration>(&contents) {
                Ok(config) => config,
                Err(e) if cli.no_fallback => panic!("Failed to parse config file, reason: {e}"),
                Err(e) => {
                    eprintln!("Failed to parse config file, reason: {e}");
                    Configuration::default()
                }
            },
            _Parameter::Default => Configuration::default(),
        }
    }
}
