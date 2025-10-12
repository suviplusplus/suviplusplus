use mongodb::{bson::doc, Collection, Database, Client, options::UpdateOptions};
use std::sync::Arc;

use crate::structs::Suvi;

/// A wrapper around MongoDB Database
#[derive(Clone)]
pub struct SuviDB {
    pub inner: Arc<Database>,
}



impl SuviDB {
    pub async fn connect(uri: &str, db_name: &str) -> Self {
        let client = Client::with_uri_str(uri)
            .await
            .expect("Failed to connect to MongoDB");
        let db = client.database(db_name);
        Self {
            inner: Arc::new(db),
        }
    }

    pub fn suvi_collection(&self) -> Collection<Suvi> {
        self.inner.collection("suvi")
    }

    pub async fn get_suvi(&self) -> Option<Suvi> {
        let coll = self.suvi_collection();
        coll
            .find_one(doc! { "_id": "suvi" })
            .await
            .expect("Failed to query MongoDB")
    }

    pub async fn increment_suvi(&self) -> mongodb::error::Result<()> {
        let coll = self.suvi_collection();
    
        let options = UpdateOptions::builder().upsert(true).build();
    
        coll.update_one(
            doc! { "_id": "suvi" },
            doc! { "$inc": { "value": 1 } },
        )
        .with_options(options)
        .await?;
    
        Ok(())
    }
}
