use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SessionData {
    pub id: Uuid,
    pub user_id: Uuid,
}
