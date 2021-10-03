use rusoto_core::Region;
use rusoto_dynamodb::*;
use std::collections::HashMap;

pub async fn list_tables() {
        let client = DynamoDbClient::new(Region::UsEast1);
        let request = ListTablesInput::default();
        match client.list_tables(request).await {
            Ok(output) => {
                match output.table_names {
                    Some(table_name_list) => {
                        println!("DynamoDb Tables:");

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

pub async fn create_table(name: String) {
    let client = DynamoDbClient::new(Region::UsEast1);
    let create_table_input: CreateTableInput = CreateTableInput {
        attribute_definitions: vec![AttributeDefinition {
            attribute_name: String::from("key"),
            attribute_type: String::from("S"),
        }],
        billing_mode: None,
        global_secondary_indexes: None,
        key_schema: vec![KeySchemaElement {
            attribute_name: String::from("key"),
            key_type: String::from("HASH"),
        }],
        local_secondary_indexes: None,
        provisioned_throughput: Some(ProvisionedThroughput {
            read_capacity_units: 4,
            write_capacity_units: 4,
        }),
        sse_specification: None,
        stream_specification: None,
        table_name: name,
        tags: None
    };

    match client.create_table(create_table_input).await {
        Ok(output) => {
            match output.table_description {
                Some(table_description) => {
                    match table_description.table_name {
                        Some(name) => println!("Created table {}!", name),
                        None => println!("No table name!"),
                    }
                    match table_description.table_arn {
                        Some(arn) => println!("ARN: {}", arn),
                        None => println!("No ARN!"),
                    }
                },
                None => println!("Table not created!"),
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        },
    }
}

pub async fn delete_table(name: String) {
    let client = DynamoDbClient::new(Region::UsEast1);
    let delete_table_input: DeleteTableInput = DeleteTableInput {
        table_name: name,
    };

    match client.delete_table(delete_table_input).await {
        Ok(output) => {
            match output.table_description {
                Some(table_description) => {
                    match table_description.table_name {
                        Some(name) => println!("Deleted table {}!", name),
                        None => println!("No table name!"),
                    }
                },
                None => println!("Table not deleted!"),
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        },
    }
}

pub async fn put_item(key: String, table_name: String) {
    let client = DynamoDbClient::new(Region::UsEast1);
    let mut item = HashMap::new();
    item.insert(key.clone(), AttributeValue {
        b: None,
        bool: None,
        bs: None,
        l: None,
        m: None,
        n: None,
        ns: None,
        null: None,
        s: Some(key),
        ss: None,
    });
    let put_item_input: PutItemInput = PutItemInput {
        condition_expression: None,
        conditional_operator: None,
        expected: None,
        expression_attribute_names: None,
        expression_attribute_values: None,
        item: item,
        return_consumed_capacity: None,
        return_item_collection_metrics: None,
        return_values: None,
        table_name: table_name,
    };

    match client.put_item(put_item_input).await {
        Ok(output) => {
            match output.attributes {
                Some(attribute) => {
                    println!("{:?}", attribute);
                    // match table_description.table_name {
                    //     Some(name) => println!("Deleted table {}!", name),
                    //     None => println!("No table name!"),
                    // }
                },
                None => println!("Item put not successful!"),
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        },
    }
}
