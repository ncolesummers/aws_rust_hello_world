use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client,Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
  let region_provider = aws_config::meta::region::RegionProviderChain::default_provider().or_else("us-west-2");
  let config = aws_config::from_env().region(region_provider).load().await;
  let client = Client::new(&config);

  let resp = client.list_tables().send().await?;

  println!("Tables:");
  let names = resp.table_names().unwrap_or_default();
  
  for name in names {
    println!("  {}", name);
  }

  println!("Found {} tables", names.len());
  println!("Hello, world!");

Ok(())
}
