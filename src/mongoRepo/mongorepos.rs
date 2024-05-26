use bson::Document;
use futures::TryFutureExt;
use mongodb::{error::Error, results::InsertOneResult, Collection};

pub struct MongoRepo {}

impl MongoRepo {
    pub async fn insert_one(collection:Collection<bson::Document>, bson_document:Document) -> Result<InsertOneResult, Error> {
        let result = collection.insert_one(bson_document, None).await;
        Ok(result.unwrap())
    }
}