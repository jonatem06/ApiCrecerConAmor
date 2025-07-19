use std::sync::Arc;
use crate::domain::models::blog::Blog;
use crate::domain::repositories::blog_repository::BlogRepository;

pub struct CreateBlogUseCase {
    blog_repository: Arc<dyn BlogRepository>,
}

impl CreateBlogUseCase {
    pub fn new(blog_repository: Arc<dyn BlogRepository>) -> Self {
        Self { blog_repository }
    }

    pub async fn execute(&self, blog: &Blog) -> Result<Blog, mongodb::error::Error> {
        self.blog_repository.create(blog).await
    }
}
