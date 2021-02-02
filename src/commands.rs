mod job_commands;

use clap::{ App, Arg, ArgMatches, Result, SubCommand };
use super::v1::{ Client, GetAccountDetailsErrorKind, TranscriptFormat };
use job_commands::{ SubmitJobCommand, GetJobStatusCommand, ListJobsCommand, DeleteJobCommand };

pub trait AppCommand {
    fn with_name(name: &str) -> App;
    fn execute(args: &ArgMatches) -> Result<()>;
}

pub struct JobCommand;

impl AppCommand for JobCommand {
    fn with_name(name: &str) -> App {
        SubCommand::with_name(name)
            .about("Job operations.")
            .subcommand(SubmitJobCommand::with_name("submit"))
            .subcommand(GetJobStatusCommand::with_name("status"))
            .subcommand(ListJobsCommand::with_name("list"))
            .subcommand(DeleteJobCommand::with_name("delete"))
    }

    fn execute(args: &ArgMatches) -> Result<()> {
        match args.subcommand() {
            ("submit", Some(cmd)) => JobCommand::execute(cmd),
            ("status", Some(cmd)) => AccountCommand::execute(cmd),
            ("list", Some(cmd)) => ListJobsCommand::execute(cmd),
            ("delete", Some(cmd)) => DeleteJobCommand::execute(cmd),
            _ => Ok(())
        }
    }
}

pub struct TranscriptCommand;

impl AppCommand for TranscriptCommand {
    fn with_name(name: &str) -> App {
        SubCommand::with_name(name)
            .about("Transcript operations.")
            .arg(Arg::with_name("id")
                .index(1)
                .required(true)
                .takes_value(true)
                .help("The job ID."))
            .arg(Arg::with_name("api_key")
                .short("k")
                .long("key")
                .takes_value(true)
                .required(true)
                .help("Temi API key."))
            .subcommand(SubmitJobCommand::with_name("share"))
    }

    fn execute(args: &ArgMatches) -> Result<()> {
        let api_key = args.value_of("api_key")
            .expect("API Key not provided.");
        let job_id = args.value_of("id")
            .expect("Job ID not provided.");
        
        let temi = Client::new(api_key);
        let _result = temi.get_transcript(job_id, TranscriptFormat::MsWord, None);

        Ok(())
    }
}

pub struct AccountCommand;

impl AppCommand for AccountCommand {
    fn with_name(name: &str) -> App {
        SubCommand::with_name(name)
            .about("Get account details.")
            .arg(Arg::with_name("api_key")
                .short("k")
                .long("key")
                .takes_value(true)
                .required(true)
                .help("Temi API key."))
    }

    fn execute(args: &ArgMatches) -> Result<()> {
        let api_key = args.value_of("api_key")
            .expect("API Key not provided.");
        let temi = Client::new(api_key);
        let result = temi.get_account_details();

        match result {
            Ok(account) => println!("{:?}", account),
            Err(GetAccountDetailsErrorKind::Unauthorized(_)) => println!("Unauthorized!"),
            Err(err) => println!("{:?}", err)
        }

        Ok(())
    }
}