use async_graphql::{SimpleObject, ID};

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

    pub fn find_by_id(id: ID) -> User {
        User::all().into_iter().find(|a| a.id == id.to_string()).unwrap()
    }
}
