#[macro_use]
extern crate clap;

use std::fs;
use std::process::Command;
use clap::{App, AppSettings, SubCommand};

fn main() {
    let matches = App::new("nvctl")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::DisableHelpSubcommand)
        .help_message("Display this help message")
        .version_message("Display version information")
        .subcommand(
            SubCommand::with_name("power")
            .about("Query and configure the discrete GPU's power state")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .setting(AppSettings::DisableHelpSubcommand)
            .subcommand(
                SubCommand::with_name("on").about("Turn the GPU on")
            )
            .subcommand(
                SubCommand::with_name("off").about("Turn the GPU off")
            )
            .subcommand(
                SubCommand::with_name("query").about("Print the power state of the GPU")
            )
        )
        .subcommand(
            SubCommand::with_name("gpu")
            .about("Query and change the GPU currently in use")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .setting(AppSettings::DisableHelpSubcommand)
            .subcommand(
                SubCommand::with_name("intel").about("Switch to the Intel GPU")
            )
            .subcommand(
                SubCommand::with_name("nvidia").about("Switch to the Nvidia GPU")
            )
            .subcommand(
                SubCommand::with_name("query").about("Print the currently-active GPU")
            )
        )
        .get_matches();

    match matches.subcommand() {
        ("power", Some(matches)) => match matches.subcommand() {
            ("on", _) => power_on(),
            ("off", _) => power_off(),
            ("query", _) => power_query(),
            _ => unreachable!(),
        },
        ("gpu", Some(matches)) => match matches.subcommand() {
            ("intel", _) => gpu_intel(),
            ("nvidia", _) => gpu_nvidia(),
            ("query", _) => gpu_query(),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn bbswitch_write(data: &str) {
    fs::write("/proc/acpi/bbswitch", data).expect("couldn't trigger bbswitch");
}

fn power_on() {
    bbswitch_write("ON");
}

fn power_off() {
    bbswitch_write("OFF");
}

fn power_query() {
    let contents = fs::read_to_string("/proc/acpi/bbswitch").expect("bbswitch not available");
    let contents = contents.trim();
    if contents.ends_with("ON") {
        println!("on");
    } else if contents.ends_with("OFF") {
        println!("off");
    } else {
        panic!("unknown bbswitch state: {}", contents);
    }
}

fn prime_select(mode: &str) -> Vec<u8> {
    let output = Command::new("prime-select")
        .arg(mode)
        .output()
        .expect("couldn't run prime-select");
    if !output.status.success() {
        panic!("prime-select failed {}", output.status.code().map_or_else(|| "due to a signal".to_owned(), |code| format!("with {}", code)));
    }
    output.stdout
}

fn gpu_intel() {
    prime_select("intel");
}

fn gpu_nvidia() {
    prime_select("nvidia");
}

fn gpu_query() {
    println!("{}", match String::from_utf8_lossy(&prime_select("query")).trim() {
        "intel" => "intel",
        "nvidia" => "nvidia",
        s => panic!("unknown GPU name: {}", s),
    })
}
