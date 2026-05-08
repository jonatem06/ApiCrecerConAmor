use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct Finanza {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[graphql(skip)]
    pub id: Option<ObjectId>,
    pub concepto: String,
    pub monto: f64,
}

#[async_graphql::ComplexObject]
impl Finanza {
    async fn id(&self) -> String {
        self.id.unwrap().to_hex()
    }
}
