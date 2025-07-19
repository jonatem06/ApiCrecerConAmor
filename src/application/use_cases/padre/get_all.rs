use std::sync::Arc;
use crate::domain::models::padre::Padre;
use crate::domain::repositories::padre_repository::PadreRepository;

pub struct GetAllPadresUseCase {
    padre_repository: Arc<dyn PadreRepository>,
}

impl GetAllPadresUseCase {
    pub fn new(padre_repository: Arc<dyn PadreRepository>) -> Self {
        Self { padre_repository }
    }

    pub async fn execute(&self) -> Result<Vec<Padre>, mongodb::error::Error> {
        self.padre_repository.get_all().await
    }
}
