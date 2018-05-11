extern crate clap;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

use clap::{App, SubCommand};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

fn ls_dynamo() {
        let client = DynamoDbClient::simple(Region::UsEast1);
        let list_tables_input: ListTablesInput = Default::default();

        match client.list_tables(&list_tables_input).sync() {
            Ok(output) => {
                match output.table_names {
                    Some(table_name_list) => {
                        println!("Tables in database:");

                        for table_name in table_name_list {
                            println!("{}", table_name);
                        }
                    },
                    None => println!("No tables in database!"),
                }
            },
            Err(error) => {
                println!("Error: {:?}", error);
            },
        }
}

fn main() {
    let matches = App::new("jaws")
                          .version("0.1")
                          .author("Ian Johnson <tacoda@pm.me>")
                          .about("AWS CLI and task runner")
                          // .arg(Arg::with_name("v")
                          //      .short("v")
                          //      .multiple(true)
                          //      .help("Sets the level of verbosity"))
                          .subcommand(SubCommand::with_name("dynamo")
                                      .about("DynamoDb")
                                      .subcommand(SubCommand::with_name("ls")
                                                  .about("List")))
                          .get_matches();

    if let Some(supercommand) = matches.subcommand_matches("dynamo") {
        if let Some(subcommand) = supercommand.subcommand_matches("ls") {
            ls_dynamo();
        }
    }

}
