#[macro_use]
extern crate structopt;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
mod jaws;

// use std::path::PathBuf;
use structopt::StructOpt;
use jaws as jawslib;

#[derive(StructOpt)]
#[structopt(name = "jaws", about = "AWS management tool and task runner")]
enum Jaws {
    #[structopt(name = "dynamodb")]
    /// DynamoDB services
    DynamoDb {
        #[structopt(short = "l", long = "list")]
        /// List DynamoDB Tables
        list: bool,

        #[structopt(short = "c", long = "create")]
        /// Create a DynamoDb Table named <name>
        name: Option<String>,
    },
    // #[structopt(name = "fetch")]
    // Fetch {
    //     #[structopt(long = "dry-run")]
    //     dry_run: bool,
    //     #[structopt(long = "all")]
    //     all: bool,
    //     repository: Option<String>
    // },
    // #[structopt(name = "commit")]
    // Commit {
    //     #[structopt(short = "m")]
    //     message: Option<String>,
    //     #[structopt(short = "a")]
    //     all: bool
    // }
}

fn main() {
    let jaws = Jaws::from_args();

    match jaws {
        Jaws::DynamoDb { list, name } => {
            if list { jawslib::dynamodb::list() }
            if let Some(name) = name { jawslib::dynamodb::create(name) }
        }
    }
}
