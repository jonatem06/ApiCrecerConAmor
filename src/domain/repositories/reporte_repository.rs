use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use crate::domain::models::reporte::Reporte;

#[async_trait]
pub trait ReporteRepository: Send + Sync {
    async fn create(&self, reporte: &Reporte) -> Result<Reporte, mongodb::error::Error>;
    async fn get(&self, id: &ObjectId) -> Result<Option<Reporte>, mongodb::error::Error>;
    async fn update(&self, id: &ObjectId, reporte: &Reporte) -> Result<Reporte, mongodb::error::Error>;
    async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error>;
    async fn get_all(&self) -> Result<Vec<Reporte>, mongodb::error::Error>;
}
