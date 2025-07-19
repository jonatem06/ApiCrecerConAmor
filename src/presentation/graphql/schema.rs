use std::sync::Arc;
use async_graphql::{EmptySubscription, MergedObject, Schema};
use crate::presentation::graphql::reporte::{ReporteMutation, ReporteQuery};
use crate::presentation::graphql::padre::{PadreMutation, PadreQuery};
use crate::presentation::graphql::personal::{PersonalMutation, PersonalQuery};
use crate::presentation::graphql::finanza::{FinanzaMutation, FinanzaQuery};
use crate::presentation::graphql::blog::{BlogMutation, BlogQuery};
use crate::infrastructure::repositories::mongodb::reporte_repository::ReporteRepositoryMongo;
use crate::infrastructure::repositories::mongodb::padre_repository::PadreRepositoryMongo;
use crate::infrastructure::repositories::mongodb::personal_repository::PersonalRepositoryMongo;
use crate::infrastructure::repositories::mongodb::finanza_repository::FinanzaRepositoryMongo;
use crate::infrastructure::repositories::mongodb::blog_repository::BlogRepositoryMongo;
use mongodb::Database;

#[derive(MergedObject, Default)]
pub struct Query(ReporteQuery, PadreQuery, PersonalQuery, FinanzaQuery, BlogQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(ReporteMutation, PadreMutation, PersonalMutation, FinanzaMutation, BlogMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(db: Arc<Database>) -> AppSchema {
    let reporte_repository = Arc::new(ReporteRepositoryMongo::new(db.collection("reportes")));
    let padre_repository = Arc::new(PadreRepositoryMongo::new(db.collection("padres")));
    let personal_repository = Arc::new(PersonalRepositoryMongo::new(db.collection("personal")));
    let finanza_repository = Arc::new(FinanzaRepositoryMongo::new(db.collection("finanzas")));
    let blog_repository = Arc::new(BlogRepositoryMongo::new(db.collection("blogs")));

    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(reporte_repository)
        .data(padre_repository)
        .data(personal_repository)
        .data(finanza_repository)
        .data(blog_repository)
        .finish()
}
