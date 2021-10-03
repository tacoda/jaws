use rusoto_core::Region;
use rusoto_s3::*;

pub async fn list_buckets() {
    let client = S3Client::new(Region::UsEast1);

    match client.list_buckets().await {
        Ok(output) => {
            match output.buckets {
                Some(bucket_name_list) => {
                    println!("S3 Buckets:");

                    for bucket in bucket_name_list {
                        println!("{}", bucket.name.unwrap());
                    }
                },
                None => println!("No buckets on S3!"),
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        },
    }
}