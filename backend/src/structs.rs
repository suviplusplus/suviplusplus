use serde::{Serialize, Deserialize};
use bson::DateTime;
use bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Suvi {
    pub _id: String,
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author: String,
    pub body: String,
    pub date: DateTime,
    pub ip: String
}

#[derive(Serialize)]
pub struct CommentResponse {
    pub id: String,
    pub author: String,
    pub body: String,
    pub date: String,
    pub ip: String,
}

impl From<Comment> for CommentResponse {
    fn from(c: Comment) -> Self {
        Self {
            id: c.id.map(|i| i.to_string()).unwrap_or("".to_string()),
            author: c.author,
            body: c.body,
            date: c.date.to_chrono().to_rfc3339(),
            ip: c.ip,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogPost {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub body: String,
    pub date: DateTime,
    pub comments: Vec<Comment>,
}

#[derive(Serialize)]
pub struct BlogPostResponse {
    pub id: String,
    pub title: String,
    pub body: String,
    pub date: String,
    pub comments: Vec<CommentResponse>,
}

impl From<BlogPost> for BlogPostResponse {
    fn from(p: BlogPost) -> Self {
        Self {
            id: p.id.map(|i| i.to_string()).unwrap_or("".to_string()),
            title: p.title,
            body: p.body,
            date: p.date.to_chrono().to_rfc3339(),
            comments: p.comments.into_iter().map(CommentResponse::from).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewBlogPost {
    pub title: String,
    pub body: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewComment {
    pub author: String,
    pub body: String,
}