use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct Padre {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[graphql(skip)]
    pub id: Option<ObjectId>,
    pub nombre: String,
    pub apellido: String,
}

#[async_graphql::ComplexObject]
impl Padre {
    async fn id(&self) -> String {
        self.id.unwrap().to_hex()
    }
}
