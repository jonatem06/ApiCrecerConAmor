use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use crate::domain::models::blog::Blog;
use crate::domain::repositories::blog_repository::BlogRepository;

pub struct UpdateBlogUseCase {
    blog_repository: Arc<dyn BlogRepository>,
}

impl UpdateBlogUseCase {
    pub fn new(blog_repository: Arc<dyn BlogRepository>) -> Self {
        Self { blog_repository }
    }

    pub async fn execute(&self, id: &ObjectId, blog: &Blog) -> Result<Blog, mongodb::error::Error> {
        self.blog_repository.update(id, blog).await
    }
}
