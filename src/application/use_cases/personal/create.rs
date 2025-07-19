use std::sync::Arc;
use crate::domain::models::personal::Personal;
use crate::domain::repositories::personal_repository::PersonalRepository;

pub struct CreatePersonalUseCase {
    personal_repository: Arc<dyn PersonalRepository>,
}

impl CreatePersonalUseCase {
    pub fn new(personal_repository: Arc<dyn PersonalRepository>) -> Self {
        Self { personal_repository }
    }

    pub async fn execute(&self, personal: &Personal) -> Result<Personal, mongodb::error::Error> {
        self.personal_repository.create(personal).await
    }
}
