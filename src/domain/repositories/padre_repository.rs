use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use crate::domain::models::padre::Padre;

#[async_trait]
pub trait PadreRepository: Send + Sync {
    async fn create(&self, padre: &Padre) -> Result<Padre, mongodb::error::Error>;
    async fn get(&self, id: &ObjectId) -> Result<Option<Padre>, mongodb::error::Error>;
    async fn update(&self, id: &ObjectId, padre: &Padre) -> Result<Padre, mongodb::error::Error>;
    async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error>;
    async fn get_all(&self) -> Result<Vec<Padre>, mongodb::error::Error>;
}
