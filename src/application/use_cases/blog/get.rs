use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use crate::domain::models::blog::Blog;
use crate::domain::repositories::blog_repository::BlogRepository;

pub struct GetBlogUseCase {
    blog_repository: Arc<dyn BlogRepository>,
}

impl GetBlogUseCase {
    pub fn new(blog_repository: Arc<dyn BlogRepository>) -> Self {
        Self { blog_repository }
    }

    pub async fn execute(&self, id: &ObjectId) -> Result<Option<Blog>, mongodb::error::Error> {
        self.blog_repository.get(id).await
    }
}
