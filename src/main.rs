extern crate clap;

mod jaws;

use clap::{App, SubCommand};
use jaws::dynamo;

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
            dynamo::ls();
        }
    }

}
