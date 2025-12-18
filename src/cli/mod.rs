/**
    EPM - E-comOS Packages Manager
    Copyright (C) 2025  E-comOS User Mode Team EPM Group & Saladin5101

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use clap::{Arg, Command};
use colored::*;
use anyhow::Result;

pub fn build_cli() -> Command {
    Command::new("epm")
        .version("0.1.0")
        .about("E-comOS Package Manager")
        .author("E-comOS Team")
        .subcommand(
            Command::new("install")
                .about("Install a package")
                .arg(Arg::new("package_name").required(true)),
        )
        .subcommand(
            Command::new("uninstall")
                .about("Remove a package")
                .arg(Arg::new("package_name").required(true)),
        )
        .subcommand(
            Command::new("update")
                .about("Update a package")
                .arg(Arg::new("package_name").required(true)),
        )
}

pub fn handle_command(matches: &clap::ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let package_name = sub_matches.get_one::<String>("package_name").unwrap();
            println!("{} installing '{}'...", ":>".green(), package_name.blue());
            // TODO: Use actual install logic
            println!("{} package '{}' installed！", ":)".green(), package_name.blue());
        }
        Some(("uninstall", sub_matches)) => {
            let package_name = sub_matches.get_one::<String>("package_name").unwrap();
            println!("{} Uninstalling '{}'...", ":>".yellow(), package_name.blue());
            // TODO: Use actual uninstall logic
            println!("{} package '{}' uninstalled！", ":)".green(), package_name.blue());
        }
        Some(("update", sub_matches)) => {
            let package_name = sub_matches.get_one::<String>("package_name").unwrap();
            println!("{} Updating '{}'...", ":>".cyan(), package_name.blue());
            // TODO: Use actual update logic
            println!("{} package '{}' updated！", ":)".green(), package_name.blue());
        }
        _ => {
            println!("{} 用法: epm <install|uninstall|update> <包名>", ":)".green());
        }
    }
    Ok(())
}