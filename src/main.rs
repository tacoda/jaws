#[macro_use]
extern crate structopt;


mod jaws;

// use std::path::PathBuf;
use structopt::StructOpt;
use crate::jaws as jawslib;

#[derive(StructOpt)]
#[structopt(name = "jaws", about = "AWS management tool and task runner")]
enum Jaws {
    #[structopt(name = "dynamodb")]
    /// DynamoDB services
    DynamoDb {
        #[structopt(subcommand)]
        // Subcommand
        cmd: DynamoDbSubCommand,
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

#[derive(StructOpt)]
enum DynamoDbSubCommand {
    #[structopt(name = "list-tables")]
    /// List DynamoDb tables
    ListTables,
    #[structopt(name = "create-table")]
    /// Create a DynamoDb table
    CreateTable {
        #[structopt(short = "n", long = "name")]
        /// Name of the DynamoDb table to create
        name: String
    },
    #[structopt(name = "delete-table")]
    /// Delete a DynamoDb table
    DeleteTable {
        #[structopt(short = "n", long = "name")]
        /// Name of the DynamoDb table to delete
        name: String
    },
    #[structopt(name = "put-item")]
    /// Put an item to a DynamoDb table
    PutItem {
        #[structopt(short = "n", long = "name")]
        /// Name of the item to put into the DynamoDb table
        name: String,
        #[structopt(short = "t", long = "table-name")]
        /// Name of the DynamoDb table to use
        table_name: String,
    },
    #[structopt(name = "get-item")]
    /// Get an item from a DynamoDb table
    GetItem {
        #[structopt(short = "n", long = "name")]
        /// Name of the item to get from the DynamoDb table
        name: String,
        #[structopt(short = "t", long = "table-name")]
        /// Name of the DynamoDb table to use
        table_name: String,
    },
    #[structopt(name = "delete-item")]
    /// Delete an item from a DynamoDb table
    DeleteItem {
        #[structopt(short = "n", long = "name")]
        /// Name of the item to delete from the DynamoDb table
        name: String,
        #[structopt(short = "t", long = "table-name")]
        /// Name of the DynamoDb table to use
        table_name: String,
    },
}

fn main() {
    let jaws = Jaws::from_args();

    match jaws {
        Jaws::DynamoDb { cmd } => {
            match cmd {
                DynamoDbSubCommand::ListTables => { jawslib::dynamodb::list_tables() },
                DynamoDbSubCommand::CreateTable { name } => { jawslib::dynamodb::create_table(name) },
                DynamoDbSubCommand::DeleteTable { name } => { jawslib::dynamodb::delete_table(name) },
                DynamoDbSubCommand::PutItem { name, table_name } => { jawslib::dynamodb::put_item(name, table_name ) },
                DynamoDbSubCommand::GetItem { name: _, table_name: _ } => { unimplemented!() },
                DynamoDbSubCommand::DeleteItem { name: _, table_name: _ } => { unimplemented!() },
            }
        }
    }
}

// Notes

// use failure::ResultExt;
// use exitfailure::ExitFailure;
//
// fn main() -> Result<(), ExitFailure> {
//
//     let path = "test.txt";
//     let content = std::fs::read_to_string(path)
//         .with_context(|_| format!("could not read file `{}`", path))?;
//     println!("file content: {}", content);
//     Ok(())
// }

// Printing errors:
// println!("This is information");
// eprintln!("This is an error! :(");

// Keep IO from becoming a bottleneck
// use std::io::{self, Write};
//
// let stdout = io::stdout(); // get the global stdout entity
// let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
// writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here

// use std::io::{self, Write};
//
// let stdout = io::stdout(); // get the global stdout entity
// let mut handle = stdout.lock(); // acquire a lock on it
// writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here

// Showing a progress bar
// indicatif crate
//
// fn main() {
//     let pb = indicatif::ProgressBar::new(100);
//     for i in 0..100 {
//         do_hard_work();
//         pb.println(format!("[+] finished #{}", i));
//         pb.inc(1);
//     }
//     pb.finish_with_message("done");
// }

// Logging
// log crate
// env_logger crate
// use log::{info, warn};
//
// fn main() {
//     env_logger::init();
//     info!("starting up");
//     warn!("oops, nothing implemented!");
// }

// Run in terminal using this
// $ env RUST_LOG=output_log=info cargo run --bin output-log
//     Finished dev [unoptimized + debuginfo] target(s) in 0.17s
//      Running `target/debug/output-log`
// [2018-11-30T20:25:52Z INFO  output_log] starting up
// [2018-11-30T20:25:52Z WARN  output_log] oops, nothing implemented!
