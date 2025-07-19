use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct Blog {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[graphql(skip)]
    pub id: Option<ObjectId>,
    pub titulo: String,
    pub contenido: String,
}

#[async_graphql::ComplexObject]
impl Blog {
    async fn id(&self) -> String {
        self.id.unwrap().to_hex()
    }
}
