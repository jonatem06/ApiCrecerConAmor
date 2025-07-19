use std::sync::Arc;
use async_graphql::{Context, Object, Result};
use mongodb::bson::oid::ObjectId;
use crate::domain::models::padre::Padre;
use crate::domain::repositories::padre_repository::PadreRepository;
use crate::application::use_cases::padre::{
    create::CreatePadreUseCase,
    get::GetPadreUseCase,
    update::UpdatePadreUseCase,
    delete::DeletePadreUseCase,
    get_all::GetAllPadresUseCase,
};

#[derive(Default)]
pub struct PadreQuery;

#[Object]
impl PadreQuery {
    async fn padre(&self, ctx: &Context<'_>, id: String) -> Result<Option<Padre>> {
        let padre_repository = ctx.data::<Arc<dyn PadreRepository>>()?.clone();
        let get_padre_use_case = GetPadreUseCase::new(padre_repository);
        let object_id = ObjectId::parse_str(&id)?;
        Ok(get_padre_use_case.execute(&object_id).await?)
    }

    async fn padres(&self, ctx: &Context<'_>) -> Result<Vec<Padre>> {
        let padre_repository = ctx.data::<Arc<dyn PadreRepository>>()?.clone();
        let get_all_padres_use_case = GetAllPadresUseCase::new(padre_repository);
        Ok(get_all_padres_use_case.execute().await?)
    }
}

#[derive(Default)]
pub struct PadreMutation;

#[Object]
impl PadreMutation {
    async fn create_padre(&self, ctx: &Context<'_>, nombre: String, apellido: String) -> Result<Padre> {
        let padre_repository = ctx.data::<Arc<dyn PadreRepository>>()?.clone();
        let create_padre_use_case = CreatePadreUseCase::new(padre_repository);
        let padre = Padre {
            id: None,
            nombre,
            apellido,
        };
        Ok(create_padre_use_case.execute(&padre).await?)
    }

    async fn update_padre(&self, ctx: &Context<'_>, id: String, nombre: String, apellido: String) -> Result<Padre> {
        let padre_repository = ctx.data::<Arc<dyn PadreRepository>>()?.clone();
        let update_padre_use_case = UpdatePadreUseCase::new(padre_repository);
        let object_id = ObjectId::parse_str(&id)?;
        let padre = Padre {
            id: Some(object_id),
            nombre,
            apellido,
        };
        Ok(update_padre_use_case.execute(&object_id, &padre).await?)
    }

    async fn delete_padre(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let padre_repository = ctx.data::<Arc<dyn PadreRepository>>()?.clone();
        let delete_padre_use_case = DeletePadreUseCase::new(padre_repository);
        let object_id = ObjectId::parse_str(&id)?;
        delete_padre_use_case.execute(&object_id).await?;
        Ok(true)
    }
}
