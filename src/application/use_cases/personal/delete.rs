use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use crate::domain::repositories::personal_repository::PersonalRepository;

pub struct DeletePersonalUseCase {
    personal_repository: Arc<dyn PersonalRepository>,
}

impl DeletePersonalUseCase {
    pub fn new(personal_repository: Arc<dyn PersonalRepository>) -> Self {
        Self { personal_repository }
    }

    pub async fn execute(&self, id: &ObjectId) -> Result<(), mongodb::error::Error> {
        self.personal_repository.delete(id).await
    }
}
