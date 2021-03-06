#[macro_use]
extern crate serde_derive;

pub mod errors;
pub mod models;
pub mod operations;
pub mod utils;

use colored::Colorize;
use errors::{LeftErrorKind, Result};

use crate::models::Config;
use crate::operations::{Apply, Install, List, New, Search, Status, Uninstall, Update, Upgrade};
use clap::Clap;
use log::error;
use std::env;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
pub struct Opt {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: u8,
    /// Operation to be performed by the theme manager
    #[clap(subcommand)]
    pub operation: Operation,
}

#[derive(Clap, Debug)]
pub enum Operation {
    // /// Finds themes not installed by LeftWM-theme
    //AutoFind(AutoFind),
    /// Install a theme
    Install(Install),
    /// Uninstall a theme
    Uninstall(Uninstall),
    /// List installed theme(s)
    #[clap(name = "list")]
    List(List),
    /// Create new theme
    New(New),
    /// Update installed themes
    Upgrade(Upgrade),
    /// Update theme list
    Update(Update),
    /// Apply an already installed theme
    Apply(Apply),
    /// Print out current theme information
    Status(Status),
    /// Search for a theme by name
    Search(Search),
}

fn main() {
    let opt = Opt::parse();

    match opt.verbose {
        0 => env::set_var("RUST_LOG", "warn"),
        1 => env::set_var("RUST_LOG", "info"),
        2 => env::set_var("RUST_LOG", "debug"),
        _ => env::set_var("RUST_LOG", "trace"),
    }

    pretty_env_logger::init();

    log::trace!("Loading configuration");
    let mut config = Config::load().unwrap_or_default();

    let wrapper: Result<()> = match opt.operation {
        //Operation::AutoFind(args) => AutoFind::exec(&args),
        Operation::Install(args) => Install::exec(&args, &mut config),
        Operation::Uninstall(args) => Uninstall::exec(&args, &mut config),
        Operation::List(args) => List::exec(&args, &mut config),
        Operation::Apply(args) => Apply::exec(&args, &mut config),
        Operation::Status(args) => Status::exec(&args, &mut config),
        Operation::New(args) => New::exec(&args, &mut config),
        Operation::Upgrade(args) => Upgrade::exec(&args, &mut config),
        Operation::Update(args) => Update::exec(&args, &mut config),
        Operation::Search(args) => Search::exec(&args, &mut config),
    };

    if let Err(e) = wrapper {
        if let LeftErrorKind::UserFriendlyError(msg) = e.inner {
            println!("{}", &msg.bright_red())
        } else {
            error!("Operation did not complete successfully")
        }
    }
}
