use clap::{ App, Arg, ArgMatches, Result, SubCommand };
use super::{ AppCommand };
use crate::v1::{ Client, GetAccountDetailsErrorKind };

struct GetTranscriptCommand;

struct ShareTranscriptCommand;