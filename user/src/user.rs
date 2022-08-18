use async_graphql::SimpleObject;

static USERS: [User; 2] = [User { id: "1", name: "Raja" }, User { id: "2", name: "Rani" }];

#[derive(Debug, Clone, SimpleObject)]
pub struct User {
    pub id: &'static str,
    pub name: &'static str,
}

impl User {
    pub fn all() -> Vec<User> {
        USERS.to_vec()
    }
}
