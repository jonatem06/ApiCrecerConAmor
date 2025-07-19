use std::sync::Arc;
use crate::domain::models::padre::Padre;
use crate::domain::repositories::padre_repository::PadreRepository;

pub struct CreatePadreUseCase {
    padre_repository: Arc<dyn PadreRepository>,
}

impl CreatePadreUseCase {
    pub fn new(padre_repository: Arc<dyn PadreRepository>) -> Self {
        Self { padre_repository }
    }

    pub async fn execute(&self, padre: &Padre) -> Result<Padre, mongodb::error::Error> {
        self.padre_repository.create(padre).await
    }
}
