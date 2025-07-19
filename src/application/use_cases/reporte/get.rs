use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use crate::domain::models::reporte::Reporte;
use crate::domain::repositories::reporte_repository::ReporteRepository;

pub struct GetReporteUseCase {
    reporte_repository: Arc<dyn ReporteRepository>,
}

impl GetReporteUseCase {
    pub fn new(reporte_repository: Arc<dyn ReporteRepository>) -> Self {
        Self { reporte_repository }
    }

    pub async fn execute(&self, id: &ObjectId) -> Result<Option<Reporte>, mongodb::error::Error> {
        self.reporte_repository.get(id).await
    }
}
