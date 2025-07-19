use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use crate::domain::repositories::reporte_repository::ReporteRepository;

pub struct DeleteReporteUseCase {
    reporte_repository: Arc<dyn ReporteRepository>,
}

impl DeleteReporteUseCase {
    pub fn new(reporte_repository: Arc<dyn ReporteRepository>) -> Self {
        Self { reporte_repository }
    }

    pub async fn execute(&self, id: &ObjectId) -> Result<(), mongodb::error::Error> {
        self.reporte_repository.delete(id).await
    }
}
