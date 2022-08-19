use async_graphql::SimpleObject;

static USERS: [User; 2] = [User { id: 1, name: "Raja" }, User { id: 2, name: "Rani" }];

#[derive(Debug, Clone, SimpleObject)]
pub struct User {
    pub id: u32,
    pub name: &'static str,
}

impl User {
    pub fn all() -> Vec<User> {
        USERS.to_vec()
    }

    pub fn find_by_id(id: u32) -> User {
        User::all().into_iter().find(|a| a.id == id).unwrap()
    }
}
