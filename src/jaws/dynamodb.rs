use rusoto_core::Region;
use rusoto_dynamodb::*;

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

pub fn create(name: String) {
    let client = DynamoDbClient::simple(Region::UsEast1);
    let create_table_input: CreateTableInput = CreateTableInput {
        attribute_definitions: vec![AttributeDefinition {
            attribute_name: String::from("key"),
            attribute_type: String::from("S"),
        }],
        global_secondary_indexes: None,
        key_schema: vec![KeySchemaElement {
            attribute_name: String::from("key"),
            key_type: String::from("HASH"),
        }],
        local_secondary_indexes: None,
        provisioned_throughput: ProvisionedThroughput {
            read_capacity_units: 4,
            write_capacity_units: 4,
        },
        stream_specification: None,
        table_name: name,
    };

    match client.create_table(&create_table_input).sync() {
        Ok(output) => {
            match output.table_description {
                Some(table_description) => {
                    println!("Table description:");
                    println!("{:?}", table_description);
                },
                None => println!("Table not created!"),
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        },
    }
}
