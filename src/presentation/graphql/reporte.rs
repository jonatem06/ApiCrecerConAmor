use std::sync::Arc;
use async_graphql::{Context, Object, Result};
use mongodb::bson::oid::ObjectId;
use crate::domain::models::reporte::Reporte;
use crate::domain::repositories::reporte_repository::ReporteRepository;
use crate::application::use_cases::reporte::{
    create::CreateReporteUseCase,
    get::GetReporteUseCase,
    update::UpdateReporteUseCase,
    delete::DeleteReporteUseCase,
    get_all::GetAllReportesUseCase,
};

#[derive(Default)]
pub struct ReporteQuery;

#[Object]
impl ReporteQuery {
    async fn reporte(&self, ctx: &Context<'_>, id: String) -> Result<Option<Reporte>> {
        let reporte_repository = ctx.data::<Arc<dyn ReporteRepository>>()?.clone();
        let get_reporte_use_case = GetReporteUseCase::new(reporte_repository);
        let object_id = ObjectId::parse_str(&id)?;
        Ok(get_reporte_use_case.execute(&object_id).await?)
    }

    async fn reportes(&self, ctx: &Context<'_>) -> Result<Vec<Reporte>> {
        let reporte_repository = ctx.data::<Arc<dyn ReporteRepository>>()?.clone();
        let get_all_reportes_use_case = GetAllReportesUseCase::new(reporte_repository);
        Ok(get_all_reportes_use_case.execute().await?)
    }
}

#[derive(Default)]
pub struct ReporteMutation;

#[Object]
impl ReporteMutation {
    async fn create_reporte(&self, ctx: &Context<'_>, titulo: String, descripcion: String) -> Result<Reporte> {
        let reporte_repository = ctx.data::<Arc<dyn ReporteRepository>>()?.clone();
        let create_reporte_use_case = CreateReporteUseCase::new(reporte_repository);
        let reporte = Reporte {
            id: None,
            titulo,
            descripcion,
        };
        Ok(create_reporte_use_case.execute(&reporte).await?)
    }

    async fn update_reporte(&self, ctx: &Context<'_>, id: String, titulo: String, descripcion: String) -> Result<Reporte> {
        let reporte_repository = ctx.data::<Arc<dyn ReporteRepository>>()?.clone();
        let update_reporte_use_case = UpdateReporteUseCase::new(reporte_repository);
        let object_id = ObjectId::parse_str(&id)?;
        let reporte = Reporte {
            id: Some(object_id),
            titulo,
            descripcion,
        };
        Ok(update_reporte_use_case.execute(&object_id, &reporte).await?)
    }

    async fn delete_reporte(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let reporte_repository = ctx.data::<Arc<dyn ReporteRepository>>()?.clone();
        let delete_reporte_use_case = DeleteReporteUseCase::new(reporte_repository);
        let object_id = ObjectId::parse_str(&id)?;
        delete_reporte_use_case.execute(&object_id).await?;
        Ok(true)
    }
}
