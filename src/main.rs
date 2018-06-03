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
        /// Create a DynamoDb Table
        create_table: Option<String>,

        #[structopt(short = "d", long = "delete")]
        /// Delete a DynamoDb Table
        delete_table: Option<String>,
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
        Jaws::DynamoDb { list, create_table, delete_table } => {
            if list { jawslib::dynamodb::list() }
            if let Some(name) = create_table { jawslib::dynamodb::create(name) }
            if let Some(name) = delete_table { jawslib::dynamodb::delete(name) }
        }
    }
}
