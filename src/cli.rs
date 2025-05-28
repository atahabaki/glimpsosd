use clap::{Args, Parser, Subcommand};
use model::power_profiles::PowerProfilesProxy;
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
    Brightness(Backlight),
}

#[derive(Subcommand)]
enum PowerProfileCommand {
    /// List available power profiles
    List,
    /// Set or get power profile
    Active(ActivePowerProfileArgs),
}

#[derive(Args)]
struct ActivePowerProfileArgs {
    profile: Option<String>,
}

/// Set or get brightness of device
#[derive(Subcommand)]
enum Backlight {
    /// Set or get display brightness
    Display {
        /// Sets display backlight brightness
        #[arg(short, long)]
        brightness: Option<usize>,
        /// Get or set brightness of this device
        #[arg(short, long)]
        device: Option<String>,
    },
    /// Set or get keyboard brightness
    Keyboard {
        /// Sets keyboard backlight brightness
        #[arg(short, long)]
        brightness: Option<usize>,
        /// Get or set brightness of this device
        #[arg(short, long)]
        device: Option<String>,
    },
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
                PowerProfileCommand::Active(active_power_profile_args) => {
                    if let Some(profile_name) = active_power_profile_args.profile {
                        proxy.set_active_profile(profile_name).await.unwrap();
                        // write to unix socket?
                        // no need, if glimpsosd is running then
                        // daemon listens for changes in power-profile
                        // removed todo!("write unix socket")
                        // So just println!
                        println!("Done");
                    } else {
                        let profile = proxy.active_profile().await.unwrap();
                        println!("{}", profile);
                    }
                }
            }
        }
        Commands::Brightness(backlight) => match backlight {
            Backlight::Display { brightness, device } => {
                if let Some(brightness) = brightness {
                    todo!("Set display brightness to {}", brightness);
                    todo!("write to unix socket");
                } else {
                    todo!("Get display brightness",);
                }
            }
            Backlight::Keyboard { brightness, device } => todo!(),
        },
    }
}
