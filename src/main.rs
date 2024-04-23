use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs::{File, create_dir_all, read_to_string, write, OpenOptions};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use toml::{Table, map::Map, Value, to_string};
use clap::Parser;
use microxdg::XdgApp;

type Maybe<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct App {
    /// State
    xdg: XdgApp,
    /// Command
    cli: Cli,
}

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

fn main () -> Maybe<()> {
    let app = App {
        xdg: XdgApp::new("dop")?,
        cli: Cli::parse(),
    };
    create_dirs(&app)?;
    if app.cli.action.is_some() {
        track_action(&app)
    } else {
        show_status(&app)
    }
}

fn create_dirs (app: &App) -> Maybe<()> {
    let config_dir = app.xdg.app_config()?;
    if !Path::new(&config_dir).exists() {
        println!("Creating {config_dir:?}");
        create_dir_all(&config_dir)?;
    }
    let config_path = config_dir.join("dop.toml");
    if !Path::new(&config_path).exists() {
        println!("Creating {config_path:?}");
        File::create_new(&config_path)?;
    }
    let data_dir = app.xdg.app_data()?;
    if !Path::new(&data_dir).exists() {
        println!("Creating {data_dir:?}");
        create_dir_all(&data_dir)?;
    }
    Ok(())
}

fn define_action (app: &App) -> Maybe<Map<String, Value>> {
    // Load config
    let config_path = app.xdg.app_config()?.join("dop.toml");
    let config_text = read_to_string(&config_path)?;
    let mut update_config = false;
    let mut config = config_text.parse::<Table>()?;
    // Define action
    if !config.contains_key("actions") {
        config.insert("actions".into(), Table::new().into());
        update_config = true;
    }
    let actions = config.get_mut("actions").unwrap().as_table_mut().unwrap();
    let action_name = app.cli.action.clone().unwrap();
    if !actions.contains_key(&action_name) {
        println!("Registering new action: \"{action_name}\"");
        actions.insert(action_name.clone().into(), Table::new().into());
        update_config = true;
    }
    // Set action as good
    if app.cli.good {
        println!("Setting valence of \"{action_name}\" to \"good\". Yay!");
        let action = actions.get_mut(&action_name).unwrap().as_table_mut().unwrap();
        action.insert("good".into(), true.into());
        update_config = true;
    }
    // Set action as bad
    if app.cli.bad {
        println!("Setting valence of \"{action_name}\" to \"bad\". Boo!");
        let action = actions.get_mut(&action_name).unwrap().as_table_mut().unwrap();
        action.insert("bad".into(), true.into());
        update_config = true;
    }
    // If there were changes to the config, write it now
    if update_config {
        write(&config_path, to_string(&config)?)?;
    }
    Ok(config.get("actions").unwrap().as_table().unwrap().clone())
}

fn track_action (app: &App) -> Maybe<()> {
    let start = SystemTime::now();
    let timestamp = start.duration_since(UNIX_EPOCH)?.as_secs();
    let actions = define_action(app)?;
    let action = app.cli.action.clone().expect("no action passed");
    let mut file_path = PathBuf::from(app.xdg.app_data()?);
    file_path.extend(&[&action]);
    let mut file = OpenOptions::new().create(true).write(true).append(true).open(&file_path)?;
    writeln!(file, "{}", timestamp)?;
    println!("Tracked action: {action}");
    let action = actions.get(&action).unwrap().as_table().unwrap();
    let good = action.get("good").map(|v|v.as_bool().unwrap()).unwrap_or(false);
    let bad = action.get("bad").map(|v|v.as_bool().unwrap()).unwrap_or(false);
    if good && bad {
        println!("This is an action you've defined as ambivalent (both good and bad).");
        println!("Consider the consequences wisely.");
    } else if good {
        println!("Yay! Keep it up!");
    } else if bad {
        println!("Ugh. Cut it out!");
    } else {
        println!("You haven't defined the valence of this action.");
        println!("Consider the consequences wisely.");
    }
    Ok(())
}

fn show_status (app: &App) -> Maybe<()> {
    unimplemented!("show status");
}
