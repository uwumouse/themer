mod config;
mod engine;

use clap::{Parser, Subcommand};
use colored::Colorize;
use config::Config;
use log;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};
use std::{fs, process::exit};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the config file
    #[clap(global = true, short, long, default_value = "~/.config/themer.yml")]
    config: String,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List avaliable themes in config file
    Themes,
    /// List avaliable files in config file
    Files {
        // TODO
        /// Check if config files are valid to be process by Themer
        #[clap(parse(from_flag), long)]
        check: bool,
    },
    /// Set new theme for all of your configuration files
    Set {
        /// Theme name to set
        #[clap(required = true, value_parser)]
        theme: String,
    },
}

fn setup_logger() {
    #[cfg(debug_assertions)]
    let level = LevelFilter::Debug;
    #[cfg(not(debug_assertions))]
    let level = LevelFilter::Error;

    let log_conf = ConfigBuilder::new()
        .set_time_level(LevelFilter::Off)
        .build();
    TermLogger::init(level, log_conf, TerminalMode::Mixed, ColorChoice::Auto).unwrap();
}

// TODO: Add & setup logger for pretty messages
// TODO: Setup subcommands: themes, files (to list respectively), set (to set theme)
fn main() {
    setup_logger();

    let args = Args::parse();
    let config = match fs::read_to_string(args.config) {
        Ok(c) => c,
        Err(_) => {
            log::error!("Failed to read Themer configuration file.");
            exit(1);
        }
    };

    let config: Config = match serde_yaml::from_str(&config) {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to parse configuration file:\n{e}");
            exit(1)
        }
    };

    let command = args.command.unwrap_or(Commands::Themes);

    match command {
        Commands::Themes => {
            println!("{}", "Avaliable themes:".purple());
            config
                .themes
                .into_iter()
                .for_each(|x| println!("  - {}", x.0));
        }
        // TODO: Maybe check if configuration file is valid by checking
        // if THEMER & THEMER_END comments exist
        Commands::Files { check } => {
            println!("{}", "Listed configuration files:".purple());
            config
                .files
                .into_iter()
                .for_each(|x| println!("  - {} ({})", x.0, x.1.path));
        }
        Commands::Set { theme } => {
            engine::update_configs(theme, config);
            println!(
                "{}\n {} To see updates, you may need to reload your environment.",
                "Theme succsessfully updated".green(),
                "?".blue()
            );
        }
    };
}
