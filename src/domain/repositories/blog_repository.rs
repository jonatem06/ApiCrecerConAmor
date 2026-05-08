use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use crate::domain::models::blog::Blog;

#[async_trait]
pub trait BlogRepository: Send + Sync {
    async fn create(&self, blog: &Blog) -> Result<Blog, mongodb::error::Error>;
    async fn get(&self, id: &ObjectId) -> Result<Option<Blog>, mongodb::error::Error>;
    async fn update(&self, id: &ObjectId, blog: &Blog) -> Result<Blog, mongodb::error::Error>;
    async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error>;
    async fn get_all(&self) -> Result<Vec<Blog>, mongodb::error::Error>;
}
