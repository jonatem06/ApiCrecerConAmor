use async_trait::async_trait;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Collection;
use crate::domain::models::personal::Personal;
use crate::domain::repositories::personal_repository::PersonalRepository;
use futures::stream::TryStreamExt;

pub struct PersonalRepositoryMongo {
    collection: Collection<Personal>,
}

impl PersonalRepositoryMongo {
    pub fn new(collection: Collection<Personal>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl PersonalRepository for PersonalRepositoryMongo {
    async fn create(&self, personal: &Personal) -> Result<Personal, mongodb::error::Error> {
        let result = self.collection.insert_one(personal, None).await?;
        let id = result.inserted_id.as_object_id().unwrap();
        let mut new_personal = personal.clone();
        new_personal.id = Some(id);
        Ok(new_personal)
    }

    async fn get(&self, id: &ObjectId) -> Result<Option<Personal>, mongodb::error::Error> {
        self.collection.find_one(doc! { "_id": id }, None).await
    }

    async fn update(&self, id: &ObjectId, personal: &Personal) -> Result<Personal, mongodb::error::Error> {
        self.collection
            .find_one_and_replace(doc! { "_id": id }, personal, None)
            .await?;
        Ok(personal.clone())
    }

    async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error> {
        self.collection.delete_one(doc! { "_id": id }, None).await?;
        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<Personal>, mongodb::error::Error> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut personal = Vec::new();
        while let Some(p) = cursor.try_next().await? {
            personal.push(p);
        }
        Ok(personal)
    }
}
