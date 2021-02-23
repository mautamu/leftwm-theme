use crate::errors;
use crate::models::{Config, LeftWm, Theme};
use clap::Clap;
use colored::*;
use log::{error, trace, warn};
use std::fs;
use std::os::unix;
use std::path::Path;
use std::process::Command;
use xdg::BaseDirectories;

/* This function sets a particular theme as the current theme in ~./config/leftwm/themes/
     Required args include "THEME", which defines the NAME of a theme as defined in a known.toml file or the themes.toml file in ~/.config/leftwm/
         TODO: THEME (with the -g/git or -f/folder flags) may also point to a git url (in the future) with a defined theme.toml file with enough global parameters defined to embed the theme in themes.toml
     Possible optional args include debug, which prints all trace! commands, and no-reset, which prevents leftwm-theme from resetting the theme
*/

#[derive(Clap, Debug)]
pub struct Apply {
    pub name: String,

    /// Don't restart leftwm-worker
    #[clap(short = 'n', long)]
    pub no_reset: bool,

    /// Ignore checks
    #[clap(short = 'o', long)]
    pub override_checks: bool,
}

impl Apply {
    pub fn exec(&self) -> Result<(), errors::LeftError> {
        trace!("Applying theme named {:?}", &self.name);
        let mut config = Config::load().unwrap_or_default();
        println!(
            "{}{}{}",
            "Setting ".bright_blue().bold(),
            &self.name.bright_green().bold(),
            " as default theme.".bright_blue().bold()
        );
        let mut dir = BaseDirectories::with_prefix("leftwm")?.create_config_directory("")?;
        dir.push("themes");
        dir.push("current");
        trace!("{:?}", &dir);
        match Theme::find(&mut config, self.name.clone()) {
            Some(theme) => match theme.directory.as_ref() {
                Some(theme_dir) => {
                    //Do all necessary checks
                    if !checks(&theme) && !self.override_checks {
                        error!("Not all prerequirements passed");
                        return Err(errors::LeftError::from("PreReqs"));
                    }
                    let path = Path::new(theme_dir);
                    trace!("{:?}", &path);
                    match fs::remove_dir_all(&dir) {
                        Ok(_) => {
                            warn!("Removed old current directory");
                        }
                        Err(_) => {
                            trace!("Nothing needed removed");
                        }
                    }
                    unix::fs::symlink(path, dir)?;
                    println!(
                        "{}{}{}",
                        "Applying ".bright_blue().bold(),
                        &self.name.bright_green().bold(),
                        " as default theme.".bright_blue().bold()
                    );
                    trace!("{:?}", "Altering config");
                    for repo in &mut config.repos {
                        for mut theme in &mut repo.themes {
                            theme.current = Some(false);
                        }
                    }
                    Theme::find_mut(&mut config, theme.name, theme.source?)?.current(true);
                    Config::save(&config)?;
                    if !self.no_reset {
                        println!("{}", "Reloading LeftWM.".bright_blue().bold());
                        Command::new("pkill").arg("leftwm-worker").output()?;
                    }
                    Ok(())
                }
                None => {
                    error!(
                        "\nTheme not installed. Try installing it with `leftwm-theme add {}`.",
                        &self.name
                    );
                    Err(errors::LeftError::from("Theme not installed"))
                }
            },
            None => {
                error!("\n Theme not installed. Try checking your spelling?");
                Err(errors::LeftError::from("Theme not installed"))
            }
        }
    }
}

pub fn check_versions(vstring: String) -> Result<bool, errors::LeftError> {
    use semver::{Version, VersionReq};
    let lwmv = LeftWm::get()?;
    let requirements = VersionReq::parse(&vstring)?;
    Ok(requirements.matches(&Version::parse(&lwmv.version)?))
}

pub fn checks(theme: &Theme) -> bool {
    trace!("Checking LeftWM version.");
    match check_versions(
        theme
            .leftwm_versions
            .clone()
            .unwrap_or_else(|| "*".to_string()),
    ) {
        Ok(true) => true,
        _ => {
            error!("This theme is incompatible with the installed version of LeftWM.");
            false
        }
    }
}