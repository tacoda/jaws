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
