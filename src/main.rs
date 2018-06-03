#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "jaws", about = "AWS management tool and task runner")]
struct Jaws {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    /// Set speed
    #[structopt(short = "s", long = "speed", default_value = "42")]
    speed: f64,
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() {
    let jaws = Jaws::from_args();
    println!("{:?}", jaws);
}

// extern crate clap;
//
// extern crate rusoto_core;
// extern crate rusoto_dynamodb;
// mod jaws;
//
// use clap::{App, Arg, SubCommand};
// use jaws::dynamo;

// fn main() {

//     let matches = App::new("jaws")
//                           .version("0.1")
//                           .author("Ian Johnson <tacoda@pm.me>")
//                           .about("AWS CLI and task runner")
//                           // .arg(Arg::with_name("v")
//                           //      .short("v")
//                           //      .multiple(true)
//                           //      .help("Sets the level of verbosity"))
//                           .subcommand(SubCommand::with_name("dynamo")
//                                       .about("DynamoDb")
//                                       .subcommand(SubCommand::with_name("list")
//                                                   .about("List"))
//                                       .subcommand(SubCommand::with_name("create")
//                                                   .about("Create")
//                                                   .arg(Arg::with_name("NAME")
//                                                         .help("Sets the name of the new table")
//                                                         .required(true))))
//                           .get_matches();
//
// // TODO: Refactor this to matches with guards
//
//     println!("Using input file: {}", matches.value_of("NAME").unwrap());
//
//     if let Some(supercommand) = matches.subcommand_matches("dynamo") {
//         if let Some(_subcommand) = supercommand.subcommand_matches("list") {
//             dynamo::list();
//         }
//         if let Some(_subcommand) = supercommand.subcommand_matches("create") {
//             dynamo::create();
//         }
//     }

// }
