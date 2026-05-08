use async_trait::async_trait;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Collection;
use crate::domain::models::finanza::Finanza;
use crate::domain::repositories::finanza_repository::FinanzaRepository;
use futures::stream::TryStreamExt;

pub struct FinanzaRepositoryMongo {
    collection: Collection<Finanza>,
}

impl FinanzaRepositoryMongo {
    pub fn new(collection: Collection<Finanza>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl FinanzaRepository for FinanzaRepositoryMongo {
    async fn create(&self, finanza: &Finanza) -> Result<Finanza, mongodb::error::Error> {
        let result = self.collection.insert_one(finanza, None).await?;
        let id = result.inserted_id.as_object_id().unwrap();
        let mut new_finanza = finanza.clone();
        new_finanza.id = Some(id);
        Ok(new_finanza)
    }

    async fn get(&self, id: &ObjectId) -> Result<Option<Finanza>, mongodb::error::Error> {
        self.collection.find_one(doc! { "_id": id }, None).await
    }

    async fn update(&self, id: &ObjectId, finanza: &Finanza) -> Result<Finanza, mongodb::error::Error> {
        self.collection
            .find_one_and_replace(doc! { "_id": id }, finanza, None)
            .await?;
        Ok(finanza.clone())
    }

    async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error> {
        self.collection.delete_one(doc! { "_id": id }, None).await?;
        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<Finanza>, mongodb::error::Error> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut finanzas = Vec::new();
        while let Some(finanza) = cursor.try_next().await? {
            finanzas.push(finanza);
        }
        Ok(finanzas)
    }
}
