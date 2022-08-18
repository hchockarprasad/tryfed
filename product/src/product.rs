use async_graphql::SimpleObject;

static PRODUCTS: [Product; 2] = [
    Product {
        name: "Phone",
        mrp: 10.0,
        owned_by: "user",
    },
    Product {
        name: "Apple",
        mrp: 2.0,
        owned_by: "user2",
    },
];

#[derive(Debug, Clone, SimpleObject)]
pub struct Product {
    pub name: &'static str,
    pub mrp: f64,
    pub owned_by: &'static str,
}

impl Product {
    pub fn all() -> Vec<Product> {
        PRODUCTS.to_vec()
    }
}
