use async_trait::async_trait;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Collection;
use crate::domain::models::reporte::Reporte;
use crate::domain::repositories::reporte_repository::ReporteRepository;
use futures::stream::TryStreamExt;

pub struct ReporteRepositoryMongo {
    collection: Collection<Reporte>,
}

impl ReporteRepositoryMongo {
    pub fn new(collection: Collection<Reporte>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl ReporteRepository for ReporteRepositoryMongo {
    async fn create(&self, reporte: &Reporte) -> Result<Reporte, mongodb::error::Error> {
        let result = self.collection.insert_one(reporte, None).await?;
        let id = result.inserted_id.as_object_id().unwrap();
        let mut new_reporte = reporte.clone();
        new_reporte.id = Some(id);
        Ok(new_reporte)
    }

    async fn get(&self, id: &ObjectId) -> Result<Option<Reporte>, mongodb::error::Error> {
        self.collection.find_one(doc! { "_id": id }, None).await
    }

    async fn update(&self, id: &ObjectId, reporte: &Reporte) -> Result<Reporte, mongodb::error::Error> {
        self.collection
            .find_one_and_replace(doc! { "_id": id }, reporte, None)
            .await?;
        Ok(reporte.clone())
    }

    async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error> {
        self.collection.delete_one(doc! { "_id": id }, None).await?;
        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<Reporte>, mongodb::error::Error> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut reportes = Vec::new();
        while let Some(reporte) = cursor.try_next().await? {
            reportes.push(reporte);
        }
        Ok(reportes)
    }
}
