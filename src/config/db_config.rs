use mongodb::{Client, Database};
use std::{env, error::Error};
use dotenv::dotenv;
pub struct DBConfig{}

impl DBConfig {
    pub async fn init() -> Result<Database, Box<dyn Error>> {
        dotenv().ok();

        let uri = match env::var("MONGOURI") {
            Ok(result) => result,
            Err(err) => return Err(Box::new(err)),
        };
        let client = Client::with_uri_str(uri).await?;
        let db = client.database("k_admin");
        println!("Connection has been established");
        Ok(db)
    }
}