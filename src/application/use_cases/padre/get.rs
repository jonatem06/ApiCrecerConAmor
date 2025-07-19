use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use crate::domain::models::padre::Padre;
use crate::domain::repositories::padre_repository::PadreRepository;

pub struct GetPadreUseCase {
    padre_repository: Arc<dyn PadreRepository>,
}

impl GetPadreUseCase {
    pub fn new(padre_repository: Arc<dyn PadreRepository>) -> Self {
        Self { padre_repository }
    }

    pub async fn execute(&self, id: &ObjectId) -> Result<Option<Padre>, mongodb::error::Error> {
        self.padre_repository.get(id).await
    }
}
