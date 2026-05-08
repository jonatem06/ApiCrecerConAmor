use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use crate::domain::models::finanza::Finanza;

#[async_trait]
pub trait FinanzaRepository: Send + Sync {
    async fn create(&self, finanza: &Finanza) -> Result<Finanza, mongodb::error::Error>;
    async fn get(&self, id: &ObjectId) -> Result<Option<Finanza>, mongodb::error::Error>;
    async fn update(&self, id: &ObjectId, finanza: &Finanza) -> Result<Finanza, mongodb::error::Error>;
    async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error>;
    async fn get_all(&self) -> Result<Vec<Finanza>, mongodb::error::Error>;
}
