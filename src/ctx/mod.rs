use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct Ctx {
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub kind: String,
}

impl Ctx {
    pub fn new(user_id: Uuid, account_id: Uuid, name: &str, kind: &str) -> Self {
        Self {
            user_id,
            account_id,
            name: name.to_string(),
            kind: kind.to_string(),
            ..Default::default()
        }
    }
}
