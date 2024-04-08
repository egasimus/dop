use clap::Parser;
use microxdg::XdgApp;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs::create_dir_all;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The name of the action to track.
    action: Option<String>,

    /// Mark action as desirable.
    #[arg(long="good", short='g')]
    good: bool,

    /// Mark action as undesirable.
    #[arg(long="bad", short='b')]
    bad: bool,
}

fn main () -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("{cli:?}");
    let xdg = XdgApp::new("dop")?;
    println!("{xdg:?}");
    let config_dir = xdg.app_config()?;
    if !Path::new(&config_dir).exists() {
        println!("Creating {config_dir:?}");
        create_dir_all(&config_dir)?;
    }
    let data_dir = xdg.app_data()?;
    if !Path::new(&data_dir).exists() {
        println!("Creating {data_dir:?}");
        create_dir_all(&data_dir)?;
    }
    if let Some(action) = cli.action {
        track_action(&config_dir, &data_dir, &action)
    } else {
        display_status(&config_dir, &data_dir)
    }
}

fn track_action (config_dir: &PathBuf, data_dir: &PathBuf, action: &str)
    -> Result<(), Box<dyn Error>>
{
    unimplemented!();
}

fn display_status (config_dir: &PathBuf, data_dir: &PathBuf)
    -> Result<(), Box<dyn Error>>
{
    unimplemented!();
}
