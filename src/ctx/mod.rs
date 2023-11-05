use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Ctx {
    pub user_id: Uuid,
    pub name: String,
    pub subscription: String,
}

impl Ctx {
    pub fn new(user_id: Uuid, name: &str, subscription: &str) -> Self {
        Self {
            user_id,
            name: name.to_string(),
            subscription: subscription.to_string(),
        }
    }
}
