use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use crate::domain::models::personal::Personal;

#[async_trait]
pub trait PersonalRepository: Send + Sync {
    async fn create(&self, personal: &Personal) -> Result<Personal, mongodb::error::Error>;
    async fn get(&self, id: &ObjectId) -> Result<Option<Personal>, mongodb::error::Error>;
    async fn update(&self, id: &ObjectId, personal: &Personal) -> Result<Personal, mongodb::error::Error>;
    async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error>;
    async fn get_all(&self) -> Result<Vec<Personal>, mongodb::error::Error>;
}
