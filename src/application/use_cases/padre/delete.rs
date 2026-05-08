use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use crate::domain::repositories::padre_repository::PadreRepository;

pub struct DeletePadreUseCase {
    padre_repository: Arc<dyn PadreRepository>,
}

impl DeletePadreUseCase {
    pub fn new(padre_repository: Arc<dyn PadreRepository>) -> Self {
        Self { padre_repository }
    }

    pub async fn execute(&self, id: &ObjectId) -> Result<(), mongodb::error::Error> {
        self.padre_repository.delete(id).await
    }
}
