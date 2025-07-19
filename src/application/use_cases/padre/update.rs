use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use crate::domain::models::padre::Padre;
use crate::domain::repositories::padre_repository::PadreRepository;

pub struct UpdatePadreUseCase {
    padre_repository: Arc<dyn PadreRepository>,
}

impl UpdatePadreUseCase {
    pub fn new(padre_repository: Arc<dyn PadreRepository>) -> Self {
        Self { padre_repository }
    }

    pub async fn execute(&self, id: &ObjectId, padre: &Padre) -> Result<Padre, mongodb::error::Error> {
        self.padre_repository.update(id, padre).await
    }
}
