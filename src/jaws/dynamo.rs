use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, CreateTableInput, ListTablesInput};

pub fn list() {
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

pub fn create() {
    // let client = DynamoDbClient::simple(Region::UsEast1);
    // let create_table_input: CreateTableInput = Default::default();
    //
    // match client.create_table(&create_table_input).sync() {
    //     Ok(output) => {
    //         match output.table_description {
    //             Some(table_description) => {
    //                 println!("Table description:");
    //                 println!("{:?}", table_description);
    //             },
    //             None => println!("Table not created!"),
    //         }
    //     },
    //     Err(error) => {
    //         println!("Error: {:?}", error);
    //     },
    // }
}
