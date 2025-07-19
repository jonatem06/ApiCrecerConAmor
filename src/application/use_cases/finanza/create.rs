use std::sync::Arc;
use crate::domain::models::finanza::Finanza;
use crate::domain::repositories::finanza_repository::FinanzaRepository;

pub struct CreateFinanzaUseCase {
    finanza_repository: Arc<dyn FinanzaRepository>,
}

impl CreateFinanzaUseCase {
    pub fn new(finanza_repository: Arc<dyn FinanzaRepository>) -> Self {
        Self { finanza_repository }
    }

    pub async fn execute(&self, finanza: &Finanza) -> Result<Finanza, mongodb::error::Error> {
        self.finanza_repository.create(finanza).await
    }
}
