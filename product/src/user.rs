use async_graphql::Object;

#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
}

#[Object(extends)]
impl User {
    #[graphql(external)]
    pub async fn id(&self) -> u32 {
        self.id
    }
}
