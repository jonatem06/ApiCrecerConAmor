use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use crate::domain::models::finanza::Finanza;
use crate::domain::repositories::finanza_repository::FinanzaRepository;

pub struct GetFinanzaUseCase {
    finanza_repository: Arc<dyn FinanzaRepository>,
}

impl GetFinanzaUseCase {
    pub fn new(finanza_repository: Arc<dyn FinanzaRepository>) -> Self {
        Self { finanza_repository }
    }

    pub async fn execute(&self, id: &ObjectId) -> Result<Option<Finanza>, mongodb::error::Error> {
        self.finanza_repository.get(id).await
    }
}
