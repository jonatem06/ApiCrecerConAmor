use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use crate::domain::models::finanza::Finanza;
use crate::domain::repositories::finanza_repository::FinanzaRepository;

pub struct UpdateFinanzaUseCase {
    finanza_repository: Arc<dyn FinanzaRepository>,
}

impl UpdateFinanzaUseCase {
    pub fn new(finanza_repository: Arc<dyn FinanzaRepository>) -> Self {
        Self { finanza_repository }
    }

    pub async fn execute(&self, id: &ObjectId, finanza: &Finanza) -> Result<Finanza, mongodb::error::Error> {
        self.finanza_repository.update(id, finanza).await
    }
}
