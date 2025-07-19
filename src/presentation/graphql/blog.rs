use std::sync::Arc;
use async_graphql::{Context, Object, Result};
use mongodb::bson::oid::ObjectId;
use crate::domain::models::blog::Blog;
use crate::domain::repositories::blog_repository::BlogRepository;
use crate::application::use_cases::blog::{
    create::CreateBlogUseCase,
    get::GetBlogUseCase,
    update::UpdateBlogUseCase,
    delete::DeleteBlogUseCase,
    get_all::GetAllBlogsUseCase,
};

#[derive(Default)]
pub struct BlogQuery;

#[Object]
impl BlogQuery {
    async fn blog(&self, ctx: &Context<'_>, id: String) -> Result<Option<Blog>> {
        let blog_repository = ctx.data::<Arc<dyn BlogRepository>>()?.clone();
        let get_blog_use_case = GetBlogUseCase::new(blog_repository);
        let object_id = ObjectId::parse_str(&id)?;
        Ok(get_blog_use_case.execute(&object_id).await?)
    }

    async fn blogs(&self, ctx: &Context<'_>) -> Result<Vec<Blog>> {
        let blog_repository = ctx.data::<Arc<dyn BlogRepository>>()?.clone();
        let get_all_blogs_use_case = GetAllBlogsUseCase::new(blog_repository);
        Ok(get_all_blogs_use_case.execute().await?)
    }
}

#[derive(Default)]
pub struct BlogMutation;

#[Object]
impl BlogMutation {
    async fn create_blog(&self, ctx: &Context<'_>, titulo: String, contenido: String) -> Result<Blog> {
        let blog_repository = ctx.data::<Arc<dyn BlogRepository>>()?.clone();
        let create_blog_use_case = CreateBlogUseCase::new(blog_repository);
        let blog = Blog {
            id: None,
            titulo,
            contenido,
        };
        Ok(create_blog_use_case.execute(&blog).await?)
    }

    async fn update_blog(&self, ctx: &Context<'_>, id: String, titulo: String, contenido: String) -> Result<Blog> {
        let blog_repository = ctx.data::<Arc<dyn BlogRepository>>()?.clone();
        let update_blog_use_case = UpdateBlogUseCase::new(blog_repository);
        let object_id = ObjectId::parse_str(&id)?;
        let blog = Blog {
            id: Some(object_id),
            titulo,
            contenido,
        };
        Ok(update_blog_use_case.execute(&object_id, &blog).await?)
    }

    async fn delete_blog(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let blog_repository = ctx.data::<Arc<dyn BlogRepository>>()?.clone();
        let delete_blog_use_case = DeleteBlogUseCase::new(blog_repository);
        let object_id = ObjectId::parse_str(&id)?;
        delete_blog_use_case.execute(&object_id).await?;
        Ok(true)
    }
}
