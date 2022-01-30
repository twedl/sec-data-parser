mod pretty_print;

use sec_data_parser::parse_submission;
use std::fs::read_dir;

use crate::pretty_print::PrettyPrint;
use clap::{AppSettings, Clap};
use std::path::PathBuf;

use serde_json;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Output(OutputCommand),
    Describe(DescribeCommand),
    Check(CheckCommand),
}

#[derive(Clap)]
struct OutputCommand {
    file: PathBuf,
}

#[derive(Clap)]
struct DescribeCommand {
    file: PathBuf,
}

#[derive(Clap)]
struct CheckCommand {
    dir: PathBuf,
}

fn main() {
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Output(OutputCommand { file }) => {
            let submission = parse_submission(&file).unwrap();

            // submission.pretty_print();
            // just output the submission as JSON
            // println!("{:?}", submission);
            let j = serde_json::to_string(&submission).unwrap();
            // alright, handle panic here by creating json
            // that just says {corrupt = "whole text file", ...}
            // or make sec_data_parser a crate with python bindings,
            // call that directly on the text? or file?
            println!("{}", j);
        }
        SubCommand::Describe(DescribeCommand { file }) => {
            let submission = parse_submission(&file).unwrap();

            submission.pretty_print();
        }
        SubCommand::Check(CheckCommand { dir }) => {
            for file in read_dir(dir).unwrap() {
                let path = file.unwrap().path();
                println!("{:?}", &path);

                // better, later: after extracting whole directory,
                // parse each file, with corrupt files as {corrupt: "whole_thing", other_values = None, ...}
                // then put everything together in one object and send back to python
                // to be written as gzip?
                parse_submission(&path).unwrap();
            }
        }
    }
}
