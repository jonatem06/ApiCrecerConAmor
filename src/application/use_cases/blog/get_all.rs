use std::sync::Arc;
use crate::domain::models::blog::Blog;
use crate::domain::repositories::blog_repository::BlogRepository;

pub struct GetAllBlogsUseCase {
    blog_repository: Arc<dyn BlogRepository>,
}

impl GetAllBlogsUseCase {
    pub fn new(blog_repository: Arc<dyn BlogRepository>) -> Self {
        Self { blog_repository }
    }

    pub async fn execute(&self) -> Result<Vec<Blog>, mongodb::error::Error> {
        self.blog_repository.get_all().await
    }
}
