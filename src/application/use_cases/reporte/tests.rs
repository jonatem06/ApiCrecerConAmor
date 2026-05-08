use super::*;
use async_trait::async_trait;
use mockall::mock;
use crate::domain::models::reporte::Reporte;
use crate::domain::repositories::reporte_repository::ReporteRepository;
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;

mock! {
    pub ReporteRepository {}

    #[async_trait]
    impl ReporteRepository for ReporteRepository {
        async fn create(&self, reporte: &Reporte) -> Result<Reporte, mongodb::error::Error>;
        async fn get(&self, id: &ObjectId) -> Result<Option<Reporte>, mongodb::error::Error>;
        async fn update(&self, id: &ObjectId, reporte: &Reporte) -> Result<Reporte, mongodb::error::Error>;
        async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error>;
        async fn get_all(&self) -> Result<Vec<Reporte>, mongodb::error::Error>;
    }
}

#[tokio::test]
async fn test_create_reporte() {
    let mut mock_repo = MockReporteRepository::new();
    let reporte = Reporte {
        id: None,
        titulo: "Test".to_string(),
        descripcion: "Test".to_string(),
    };

    mock_repo.expect_create()
        .withf(|r| r.titulo == "Test")
        .times(1)
        .returning(|r| Ok(r.clone()));

    let use_case = CreateReporteUseCase::new(Arc::new(mock_repo));
    let result = use_case.execute(&reporte).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().titulo, "Test");
}
