#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;

use std::fs;
use std::process::{Command, exit};
use clap::{App, AppSettings, Arg, SubCommand};
use failure::Error;

#[derive(Debug, Fail)]
enum BbswitchError {
    #[fail(display = "bbswitch not available")]
    BbswitchNotAvailable {
        #[fail(cause)]
        cause: std::io::Error,
    },
    #[fail(display = "unknown bbswitch state")]
    UnknownBbswitchState {
        state: String,
    },
}

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
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Don't yell on error")
                .global(true)
        )
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

    match match matches.subcommand() {
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
    } {
        Ok(_) => {},
        Err(e) => {
            if !matches.is_present("quiet") {
                println!("error: {}", e.cause())
            }
            exit(1)
        },
    }
}

fn bbswitch_read() -> Result<String, BbswitchError> {
    fs::read_to_string("/proc/acpi/bbswitch").map_err(|e| BbswitchError::BbswitchNotAvailable { cause: e })
}

fn bbswitch_write(data: &str) -> Result<(), BbswitchError> {
    fs::write("/proc/acpi/bbswitch", data).map_err(|e| BbswitchError::BbswitchNotAvailable { cause: e })
}

fn power_on() -> Result<(), Error> {
    Ok(bbswitch_write("ON")?)
}

fn power_off() -> Result<(), Error> {
    Ok(bbswitch_write("OFF")?)
}

fn power_query() -> Result<(), Error> {
    let contents = bbswitch_read()?;
    let contents = contents.trim();
    if contents.ends_with("ON") {
        println!("on");
    } else if contents.ends_with("OFF") {
        println!("off");
    } else {
        return Err(BbswitchError::UnknownBbswitchState {
            state: contents.to_owned(),
        }.into());
    }
    Ok(())
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

fn gpu_intel() -> Result<(), Error> {
    prime_select("intel");
    Ok(())
}

fn gpu_nvidia() -> Result<(), Error> {
    prime_select("nvidia");
    Ok(())
}

fn gpu_query() -> Result<(), Error> {
    println!("{}", match String::from_utf8_lossy(&prime_select("query")).trim() {
        "intel" => "intel",
        "nvidia" => "nvidia",
        s => panic!("unknown GPU name: {}", s),
    });
    Ok(())
}
