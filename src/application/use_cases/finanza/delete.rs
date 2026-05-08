use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use crate::domain::repositories::finanza_repository::FinanzaRepository;

pub struct DeleteFinanzaUseCase {
    finanza_repository: Arc<dyn FinanzaRepository>,
}

impl DeleteFinanzaUseCase {
    pub fn new(finanza_repository: Arc<dyn FinanzaRepository>) -> Self {
        Self { finanza_repository }
    }

    pub async fn execute(&self, id: &ObjectId) -> Result<(), mongodb::error::Error> {
        self.finanza_repository.delete(id).await
    }
}
