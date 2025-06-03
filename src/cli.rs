use clap::{Parser, Subcommand};
use model::{keyboard_backlight::KeyboardBacklightProxy, power_profiles::PowerProfilesProxy};
use zbus::Connection;

pub(crate) mod model;

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// list, set, or get power profile with power-profiles-daemon
    #[command(subcommand)]
    PowerProfile(PowerProfileCommand),
    #[command(subcommand)]
    Brightness(BrightnessDevice),
}

#[derive(Subcommand)]
enum PowerProfileCommand {
    /// List available power profiles
    List,
    /// Set to previous power profile
    Previous,
    /// Set to next power profile
    Next,
    /// Set or get power profile
    Active {
        /// Set power profile
        #[arg(short, long)]
        profile: Option<String>,
    },
}

/// Set or get brightness of device
#[derive(Subcommand)]
enum BrightnessDevice {
    /// Set current or get current, max display brightness
    #[command(subcommand)]
    Display(BrightnessCommand),
    /// Set current or get current, max keyboard brightness
    #[command(subcommand)]
    Keyboard(BrightnessCommand),
}

/// Set or get brightness
#[derive(Subcommand)]
enum BrightnessCommand {
    /// Set or get current brightness
    Current {
        /// Set current brightness
        #[arg(short, long)]
        set: Option<i32>,
    },
    /// Get max brightness
    Max,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::PowerProfile(power_profile_command) => {
            let connection = Connection::system().await.unwrap();
            let proxy = PowerProfilesProxy::new(&connection).await.unwrap();
            match power_profile_command {
                PowerProfileCommand::List => {
                    let profiles = proxy.profiles().await.unwrap();
                    for profile in profiles {
                        if let Some(profile_name) = profile.get("Profile") {
                            println!("{}", profile_name);
                        }
                    }
                }
                PowerProfileCommand::Previous | PowerProfileCommand::Next => {
                    let is_first = matches!(power_profile_command, PowerProfileCommand::Next);
                    let profiles = proxy.profiles().await.unwrap();
                    let current_profile = proxy.active_profile().await.unwrap();

                    let next_profile = profiles
                        .iter()
                        .map(|p| p.get("Profile"))
                        .collect::<Option<Vec<_>>>()
                        .and_then(|names| {
                            names
                                .iter()
                                .position(|&name| name == &current_profile)
                                .and_then(|i| {
                                    names
                                        .get(if is_first { i + 1 } else { i - 1 })
                                        .cloned()
                                        .or_else(|| {
                                            if is_first {
                                                names.first()
                                            } else {
                                                names.last()
                                            }
                                            .cloned()
                                        })
                                })
                        });

                    if let Some(profile) = next_profile {
                        proxy.set_active_profile(profile.to_string()).await.unwrap();
                    }
                }
                PowerProfileCommand::Active { profile } => match profile {
                    Some(profile_name) => proxy.set_active_profile(profile_name).await.unwrap(),
                    None => println!("{}", proxy.active_profile().await.unwrap()),
                },
            }
        }
        Commands::Brightness(brightness) => {
            let connection = Connection::system().await.unwrap();
            let proxy = KeyboardBacklightProxy::new(&connection).await.unwrap();
            match brightness {
                BrightnessDevice::Display(_brightness_command) => todo!(),
                BrightnessDevice::Keyboard(brightness_command) => match brightness_command {
                    BrightnessCommand::Current { set } => match set {
                        Some(brightness) => {
                            proxy.set_brightness(brightness).await.unwrap();
                            todo!(
                                "Write to unix socket Event::KeyboardBrightness({})",
                                brightness
                            );
                        }
                        None => println!("{}", proxy.get_brightness().await.unwrap()),
                    },
                    BrightnessCommand::Max => {
                        println!("{}", proxy.get_max_brightness().await.unwrap())
                    }
                },
            }
        }
    }
}
