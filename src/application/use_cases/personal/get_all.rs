use std::sync::Arc;
use crate::domain::models::personal::Personal;
use crate::domain::repositories::personal_repository::PersonalRepository;

pub struct GetAllPersonalUseCase {
    personal_repository: Arc<dyn PersonalRepository>,
}

impl GetAllPersonalUseCase {
    pub fn new(personal_repository: Arc<dyn PersonalRepository>) -> Self {
        Self { personal_repository }
    }

    pub async fn execute(&self) -> Result<Vec<Personal>, mongodb::error::Error> {
        self.personal_repository.get_all().await
    }
}
