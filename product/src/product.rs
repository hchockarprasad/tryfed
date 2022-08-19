use async_graphql::SimpleObject;

use crate::user::User;

static PRODUCTS: [Product; 2] = [
    Product {
        name: "Phone",
        mrp: 10.0,
        owned_by: User { id: 1 },
    },
    Product {
        name: "Apple",
        mrp: 2.0,
        owned_by: User { id: 2 },
    },
];

#[derive(Debug, Clone, SimpleObject)]
pub struct Product {
    pub name: &'static str,
    pub mrp: f64,
    pub owned_by: User,
}

impl Product {
    pub fn all() -> Vec<Product> {
        PRODUCTS.to_vec()
    }
}
