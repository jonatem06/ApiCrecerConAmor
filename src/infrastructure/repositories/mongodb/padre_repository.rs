use async_trait::async_trait;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Collection;
use crate::domain::models::padre::Padre;
use crate::domain::repositories::padre_repository::PadreRepository;
use futures::stream::TryStreamExt;

pub struct PadreRepositoryMongo {
    collection: Collection<Padre>,
}

impl PadreRepositoryMongo {
    pub fn new(collection: Collection<Padre>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl PadreRepository for PadreRepositoryMongo {
    async fn create(&self, padre: &Padre) -> Result<Padre, mongodb::error::Error> {
        let result = self.collection.insert_one(padre, None).await?;
        let id = result.inserted_id.as_object_id().unwrap();
        let mut new_padre = padre.clone();
        new_padre.id = Some(id);
        Ok(new_padre)
    }

    async fn get(&self, id: &ObjectId) -> Result<Option<Padre>, mongodb::error::Error> {
        self.collection.find_one(doc! { "_id": id }, None).await
    }

    async fn update(&self, id: &ObjectId, padre: &Padre) -> Result<Padre, mongodb::error::Error> {
        self.collection
            .find_one_and_replace(doc! { "_id": id }, padre, None)
            .await?;
        Ok(padre.clone())
    }

    async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error> {
        self.collection.delete_one(doc! { "_id": id }, None).await?;
        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<Padre>, mongodb::error::Error> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut padres = Vec::new();
        while let Some(padre) = cursor.try_next().await? {
            padres.push(padre);
        }
        Ok(padres)
    }
}
