use std::collections::HashMap;

use async_graphql::{dataloader::Loader, SimpleObject};

static USERS: [User; 2] = [User { id: 1, name: "John" }, User { id: 2, name: "Wick" }];

#[derive(Debug, Clone, SimpleObject)]
pub struct User {
    pub id: u32,
    pub name: &'static str,
}

impl User {
    pub fn all() -> Vec<User> {
        USERS.to_vec()
    }
}

pub struct UserLoader;

impl UserLoader {
    pub fn filter_by_id(id: &[u32]) -> Vec<User> {
        User::all().into_iter().filter(|a| id.contains(&a.id)).collect()
    }
}

#[async_trait::async_trait]
impl Loader<u32> for UserLoader {
    type Value = User;
    type Error = &'static str;

    async fn load(&self, keys: &[u32]) -> Result<HashMap<u32, Self::Value>, Self::Error> {
        let out = UserLoader::filter_by_id(keys).into_iter().map(|x| (x.id, x)).collect();
        Ok(out)
    }
}
