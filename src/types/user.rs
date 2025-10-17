// Placeholder user types module (frontend-specific)

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct UserSummary {
    pub id: uuid::Uuid,
    pub username: String,
}
