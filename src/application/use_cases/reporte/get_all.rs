use std::sync::Arc;
use crate::domain::models::reporte::Reporte;
use crate::domain::repositories::reporte_repository::ReporteRepository;

pub struct GetAllReportesUseCase {
    reporte_repository: Arc<dyn ReporteRepository>,
}

impl GetAllReportesUseCase {
    pub fn new(reporte_repository: Arc<dyn ReporteRepository>) -> Self {
        Self { reporte_repository }
    }

    pub async fn execute(&self) -> Result<Vec<Reporte>, mongodb::error::Error> {
        self.reporte_repository.get_all().await
    }
}
