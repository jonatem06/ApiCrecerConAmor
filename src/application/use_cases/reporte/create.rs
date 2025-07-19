use std::sync::Arc;
use crate::domain::models::reporte::Reporte;
use crate::domain::repositories::reporte_repository::ReporteRepository;

pub struct CreateReporteUseCase {
    reporte_repository: Arc<dyn ReporteRepository>,
}

impl CreateReporteUseCase {
    pub fn new(reporte_repository: Arc<dyn ReporteRepository>) -> Self {
        Self { reporte_repository }
    }

    pub async fn execute(&self, reporte: &Reporte) -> Result<Reporte, mongodb::error::Error> {
        self.reporte_repository.create(reporte).await
    }
}
