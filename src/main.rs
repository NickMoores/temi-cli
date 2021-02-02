mod v1;
mod commands;

use clap::{ App, Arg, Result, SubCommand, AppSettings };
use commands::{AccountCommand, AppCommand, JobCommand, TranscriptCommand};

fn main() -> Result<()> {
    let app = App::new("Temi CLI")
        .version("0.1.0")
        .author("Nick Moores <nick@nickmoores.com>")
        .about("CLI for interacting with the Temi web service.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(AccountCommand::with_name("account"))
        .subcommand(JobCommand::with_name("job"))
        .subcommand(TranscriptCommand::with_name("transcript"))
        .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("A cool file"))
        .arg(Arg::with_name("num")
                .short("n")
                .long("number")
                .takes_value(true)
                .help("Five less than your favorite number"));

    let _matches = app.get_matches();

    match _matches.subcommand() {
        ("job", Some(cmd)) => JobCommand::execute(cmd),
        ("account", Some(cmd)) => AccountCommand::execute(cmd),
        ("transcript", Some(cmd)) => TranscriptCommand::execute(cmd),
        _ => Ok(())
    };

    Ok(())
}