use mongodb::{Client, Collection, Database, bson::doc, options::{FindOneAndUpdateOptions, ReturnDocument}, results::UpdateResult};
use bson::{Document, oid::ObjectId, to_bson};
use std::sync::Arc;
use futures::{TryStreamExt};
use mongodb::results::InsertOneResult;
use crate::structs::{BlogPost, Comment, Suvi};

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

    pub fn blog_collection(&self) -> Collection<BlogPost> {
        self.inner.collection("blog")
    }

    pub async fn get_blog(&self) -> Result<Vec<BlogPost>, mongodb::error::Error> {

        let cursor = self.blog_collection().find(Document::new()).await?;

        let blogs: Vec<BlogPost> = cursor.try_collect().await?;
        Ok(blogs)
    }

    pub async fn get_blog_post(&self, id: ObjectId) -> Result<Option<BlogPost>, mongodb::error::Error> {
        let post = self.blog_collection().find_one(doc! { "_id": id }).await?;
        Ok(post)
    }

    pub async fn insert_blog(&self, post: BlogPost) -> Result<InsertOneResult, mongodb::error::Error> {
        let coll = self.blog_collection();
        coll.insert_one(post).await
    }
    
    pub async fn add_comment(&self, target: ObjectId, comment: Comment) -> Result<UpdateResult, mongodb::error::Error> {
        let coll = self.blog_collection();
        let comment_bson = to_bson(&comment)
            .map_err(|e| mongodb::error::Error::custom(e))?;
        coll.update_one(
            doc! { "_id": target },
            doc! { "$push": { "comments": comment_bson } },
        ).await
    }

    pub async fn get_suvi(&self) -> Option<Suvi> {
        let coll = self.suvi_collection();
        coll
            .find_one(doc! { "_id": "suvi" })
            .await
            .expect("Failed to query MongoDB")
    }

    pub async fn increment_suvi(&self) -> mongodb::error::Result<i32> {
        let coll = self.suvi_collection();

        let updated = coll
            .find_one_and_update(
                doc! { "_id": "suvi" },
                doc! { "$inc": { "value": 1 } },
                
            )
            .with_options(FindOneAndUpdateOptions::builder()
                    .upsert(true)
                    .return_document(ReturnDocument::After)
                    .build())
            .await?;

        Ok(updated.map(|s| s.value).unwrap_or(0))
    }
}
