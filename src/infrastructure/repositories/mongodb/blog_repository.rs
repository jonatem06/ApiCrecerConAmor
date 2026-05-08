use async_trait::async_trait;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Collection;
use crate::domain::models::blog::Blog;
use crate::domain::repositories::blog_repository::BlogRepository;
use futures::stream::TryStreamExt;

pub struct BlogRepositoryMongo {
    collection: Collection<Blog>,
}

impl BlogRepositoryMongo {
    pub fn new(collection: Collection<Blog>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl BlogRepository for BlogRepositoryMongo {
    async fn create(&self, blog: &Blog) -> Result<Blog, mongodb::error::Error> {
        let result = self.collection.insert_one(blog, None).await?;
        let id = result.inserted_id.as_object_id().unwrap();
        let mut new_blog = blog.clone();
        new_blog.id = Some(id);
        Ok(new_blog)
    }

    async fn get(&self, id: &ObjectId) -> Result<Option<Blog>, mongodb::error::Error> {
        self.collection.find_one(doc! { "_id": id }, None).await
    }

    async fn update(&self, id: &ObjectId, blog: &Blog) -> Result<Blog, mongodb::error::Error> {
        self.collection
            .find_one_and_replace(doc! { "_id": id }, blog, None)
            .await?;
        Ok(blog.clone())
    }

    async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error> {
        self.collection.delete_one(doc! { "_id": id }, None).await?;
        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<Blog>, mongodb::error::Error> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut blogs = Vec::new();
        while let Some(blog) = cursor.try_next().await? {
            blogs.push(blog);
        }
        Ok(blogs)
    }
}
