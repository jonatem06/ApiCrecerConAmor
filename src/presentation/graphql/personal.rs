use std::sync::Arc;
use async_graphql::{Context, Object, Result};
use mongodb::bson::oid::ObjectId;
use crate::domain::models::personal::Personal;
use crate::domain::repositories::personal_repository::PersonalRepository;
use crate::application::use_cases::personal::{
    create::CreatePersonalUseCase,
    get::GetPersonalUseCase,
    update::UpdatePersonalUseCase,
    delete::DeletePersonalUseCase,
    get_all::GetAllPersonalUseCase,
};

#[derive(Default)]
pub struct PersonalQuery;

#[Object]
impl PersonalQuery {
    async fn personal(&self, ctx: &Context<'_>, id: String) -> Result<Option<Personal>> {
        let personal_repository = ctx.data::<Arc<dyn PersonalRepository>>()?.clone();
        let get_personal_use_case = GetPersonalUseCase::new(personal_repository);
        let object_id = ObjectId::parse_str(&id)?;
        Ok(get_personal_use_case.execute(&object_id).await?)
    }

    async fn personales(&self, ctx: &Context<'_>) -> Result<Vec<Personal>> {
        let personal_repository = ctx.data::<Arc<dyn PersonalRepository>>()?.clone();
        let get_all_personal_use_case = GetAllPersonalUseCase::new(personal_repository);
        Ok(get_all_personal_use_case.execute().await?)
    }
}

#[derive(Default)]
pub struct PersonalMutation;

#[Object]
impl PersonalMutation {
    async fn create_personal(&self, ctx: &Context<'_>, nombre: String, puesto: String) -> Result<Personal> {
        let personal_repository = ctx.data::<Arc<dyn PersonalRepository>>()?.clone();
        let create_personal_use_case = CreatePersonalUseCase::new(personal_repository);
        let personal = Personal {
            id: None,
            nombre,
            puesto,
        };
        Ok(create_personal_use_case.execute(&personal).await?)
    }

    async fn update_personal(&self, ctx: &Context<'_>, id: String, nombre: String, puesto: String) -> Result<Personal> {
        let personal_repository = ctx.data::<Arc<dyn PersonalRepository>>()?.clone();
        let update_personal_use_case = UpdatePersonalUseCase::new(personal_repository);
        let object_id = ObjectId::parse_str(&id)?;
        let personal = Personal {
            id: Some(object_id),
            nombre,
            puesto,
        };
        Ok(update_personal_use_case.execute(&object_id, &personal).await?)
    }

    async fn delete_personal(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let personal_repository = ctx.data::<Arc<dyn PersonalRepository>>()?.clone();
        let delete_personal_use_case = DeletePersonalUseCase::new(personal_repository);
        let object_id = ObjectId::parse_str(&id)?;
        delete_personal_use_case.execute(&object_id).await?;
        Ok(true)
    }
}
