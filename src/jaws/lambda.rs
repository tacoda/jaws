use rusoto_core::Region;
use rusoto_lambda::*;

pub async fn list_functions() {
    let client = LambdaClient::new(Region::UsEast1);
    let request = ListFunctionsRequest::default();

    match client.list_functions(request).await {
        Ok(output) => {
            match output.functions {
                Some(function_name_list) => {
                    println!("Lambda functions:");

                    for function in function_name_list {
                        println!("{}", function.function_name.unwrap());
                    }
                },
                None => println!("No functions on Lambda!"),
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        },
    }
}