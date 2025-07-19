use std::sync::Arc;
use crate::domain::models::finanza::Finanza;
use crate::domain::repositories::finanza_repository::FinanzaRepository;

pub struct GetAllFinanzasUseCase {
    finanza_repository: Arc<dyn FinanzaRepository>,
}

impl GetAllFinanzasUseCase {
    pub fn new(finanza_repository: Arc<dyn FinanzaRepository>) -> Self {
        Self { finanza_repository }
    }

    pub async fn execute(&self) -> Result<Vec<Finanza>, mongodb::error::Error> {
        self.finanza_repository.get_all().await
    }
}
