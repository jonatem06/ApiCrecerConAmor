use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use crate::domain::repositories::blog_repository::BlogRepository;

pub struct DeleteBlogUseCase {
    blog_repository: Arc<dyn BlogRepository>,
}

impl DeleteBlogUseCase {
    pub fn new(blog_repository: Arc<dyn BlogRepository>) -> Self {
        Self { blog_repository }
    }

    pub async fn execute(&self, id: &ObjectId) -> Result<(), mongodb::error::Error> {
        self.blog_repository.delete(id).await
    }
}
