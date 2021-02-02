
use clap::{ App, Arg, ArgMatches, Result, SubCommand };
use super::{ AppCommand };
use crate::v1::{ Client, GetAccountDetailsErrorKind, DeleteJobErrorKind };

pub struct SubmitJobCommand;

impl AppCommand for SubmitJobCommand {
    fn with_name(name: &str) -> App {
        SubCommand::with_name(name)
            .about("Submit a job via a URL.")
            .arg(Arg::with_name("api_key")
                .short("k")
                .long("key")
                .takes_value(true)
                .required(true)
                .help("Temi API key."))
            .arg(Arg::with_name("url")
                .index(1)
                .required(true)
                .takes_value(true)
                .help("The URL to the file to submit."))
            .arg(Arg::with_name("metadata")
                .short("m")
                .long("metadata")
                .takes_value(true)
                .help("Metadata associated with the job."))
            .arg(Arg::with_name("callback")
                .short("c")
                .long("callback")
                .takes_value(true)
                .help("The URL to be invoked when the job completes execution."))
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

pub struct UploadJobCommand;

pub struct GetJobStatusCommand;

impl AppCommand for GetJobStatusCommand {
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
        let job_id = args.value_of("job_id")
            .expect("Job ID not provided.");

        let temi = Client::new(api_key);
        let result = temi.get_job_status(job_id);

        match result {
            Ok(account) => println!("{:?}", account),
            Err(err) => println!("{:?}", err)
        }

        Ok(())
    }
}

pub struct ListJobsCommand;

impl AppCommand for ListJobsCommand {
    fn with_name(name: &str) -> App {
        SubCommand::with_name(name)
            .about("Lists jobs.")
            .arg(Arg::with_name("limit")
                .short("l")
                .long("limit")
                .takes_value(true)
                .help("The maximum number of jobs to return."))
            .arg(Arg::with_name("starting_after")
                .short("a")
                .long("starting-after")
                .takes_value(true)
                .help("The job in the list to start at."))
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

        let limit = args.value_of("limit").map(|l| l.parse().unwrap());
        let starting_after = args.value_of("starting_after");

        let temi = Client::new(api_key);
        let result = temi.list_jobs(limit, starting_after);

        match result {
            Ok(jobs) => println!("{:?}", jobs),
            Err(err) => eprintln!("{:?}", err)
        }

        Ok(())
    }
}

pub struct DeleteJobCommand;

impl AppCommand for DeleteJobCommand {
    fn with_name(name: &str) -> App {
        SubCommand::with_name(name)
            .about("Deletes a job.")
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
    }

    fn execute(args: &ArgMatches) -> Result<()> {
        let api_key = args.value_of("api_key")
            .expect("API Key not provided.");
        let job_id = args.value_of("id")
            .expect("Job ID not provided.");

        let temi = Client::new(api_key);
        let result = temi.delete_job(job_id);

        match result {
            Ok(account) => println!("{:?}", account),
            Err(DeleteJobErrorKind::InvalidParameters(prb)) => eprintln!("Invalid parameters: {}", prb.title),
            Err(DeleteJobErrorKind::JobNotFound(_)) => eprintln!("Job not found: {}", job_id),
            Err(DeleteJobErrorKind::JobNotTranscribed(_)) => eprintln!("Job not transcribed: {}", job_id),
            Err(err) => eprintln!("{:?}", err)
        }

        Ok(())
    }
}