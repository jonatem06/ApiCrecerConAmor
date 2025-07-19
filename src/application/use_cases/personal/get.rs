use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use crate::domain::models::personal::Personal;
use crate::domain::repositories::personal_repository::PersonalRepository;

pub struct GetPersonalUseCase {
    personal_repository: Arc<dyn PersonalRepository>,
}

impl GetPersonalUseCase {
    pub fn new(personal_repository: Arc<dyn PersonalRepository>) -> Self {
        Self { personal_repository }
    }

    pub async fn execute(&self, id: &ObjectId) -> Result<Option<Personal>, mongodb::error::Error> {
        self.personal_repository.get(id).await
    }
}
