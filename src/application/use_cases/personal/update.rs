use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use crate::domain::models::personal::Personal;
use crate::domain::repositories::personal_repository::PersonalRepository;

pub struct UpdatePersonalUseCase {
    personal_repository: Arc<dyn PersonalRepository>,
}

impl UpdatePersonalUseCase {
    pub fn new(personal_repository: Arc<dyn PersonalRepository>) -> Self {
        Self { personal_repository }
    }

    pub async fn execute(&self, id: &ObjectId, personal: &Personal) -> Result<Personal, mongodb::error::Error> {
        self.personal_repository.update(id, personal).await
    }
}
