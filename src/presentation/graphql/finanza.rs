use std::sync::Arc;
use async_graphql::{Context, Object, Result};
use mongodb::bson::oid::ObjectId;
use crate::domain::models::finanza::Finanza;
use crate::domain::repositories::finanza_repository::FinanzaRepository;
use crate::application::use_cases::finanza::{
    create::CreateFinanzaUseCase,
    get::GetFinanzaUseCase,
    update::UpdateFinanzaUseCase,
    delete::DeleteFinanzaUseCase,
    get_all::GetAllFinanzasUseCase,
};

#[derive(Default)]
pub struct FinanzaQuery;

#[Object]
impl FinanzaQuery {
    async fn finanza(&self, ctx: &Context<'_>, id: String) -> Result<Option<Finanza>> {
        let finanza_repository = ctx.data::<Arc<dyn FinanzaRepository>>()?.clone();
        let get_finanza_use_case = GetFinanzaUseCase::new(finanza_repository);
        let object_id = ObjectId::parse_str(&id)?;
        Ok(get_finanza_use_case.execute(&object_id).await?)
    }

    async fn finanzas(&self, ctx: &Context<'_>) -> Result<Vec<Finanza>> {
        let finanza_repository = ctx.data::<Arc<dyn FinanzaRepository>>()?.clone();
        let get_all_finanzas_use_case = GetAllFinanzasUseCase::new(finanza_repository);
        Ok(get_all_finanzas_use_case.execute().await?)
    }
}

#[derive(Default)]
pub struct FinanzaMutation;

#[Object]
impl FinanzaMutation {
    async fn create_finanza(&self, ctx: &Context<'_>, concepto: String, monto: f64) -> Result<Finanza> {
        let finanza_repository = ctx.data::<Arc<dyn FinanzaRepository>>()?.clone();
        let create_finanza_use_case = CreateFinanzaUseCase::new(finanza_repository);
        let finanza = Finanza {
            id: None,
            concepto,
            monto,
        };
        Ok(create_finanza_use_case.execute(&finanza).await?)
    }

    async fn update_finanza(&self, ctx: &Context<'_>, id: String, concepto: String, monto: f64) -> Result<Finanza> {
        let finanza_repository = ctx.data::<Arc<dyn FinanzaRepository>>()?.clone();
        let update_finanza_use_case = UpdateFinanzaUseCase::new(finanza_repository);
        let object_id = ObjectId::parse_str(&id)?;
        let finanza = Finanza {
            id: Some(object_id),
            concepto,
            monto,
        };
        Ok(update_finanza_use_case.execute(&object_id, &finanza).await?)
    }

    async fn delete_finanza(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let finanza_repository = ctx.data::<Arc<dyn FinanzaRepository>>()?.clone();
        let delete_finanza_use_case = DeleteFinanzaUseCase::new(finanza_repository);
        let object_id = ObjectId::parse_str(&id)?;
        delete_finanza_use_case.execute(&object_id).await?;
        Ok(true)
    }
}
