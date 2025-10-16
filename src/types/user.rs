// Placeholder user types module (frontend-specific)

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct UserSummary {
    pub id: uuid::Uuid,
    pub username: String,
}

