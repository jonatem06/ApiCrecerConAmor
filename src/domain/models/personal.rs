use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct Personal {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[graphql(skip)]
    pub id: Option<ObjectId>,
    pub nombre: String,
    pub puesto: String,
}

#[async_graphql::ComplexObject]
impl Personal {
    async fn id(&self) -> String {
        self.id.unwrap().to_hex()
    }
}
