mod product;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{
    guard,
    middleware::{NormalizePath, TrailingSlash},
    web, App, HttpResponse, HttpServer,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Object, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use product::Product;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    pub async fn products(&self) -> Vec<Product> {
        Product::all()
    }
}

async fn index_playground() -> actix_web::Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(source))
}

async fn index(schema: web::Data<AppSchema>, gql_req: GraphQLRequest) -> GraphQLResponse {
    let request = gql_req.into_inner();
    schema.execute(request).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let schema = Schema::build(Query, EmptyMutation::default(), EmptySubscription).finish();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
            .service(web::resource("/").guard(guard::Post()).to(index))
    })
    .bind("0.0.0.0:8002")?
    .run();
    let print_message = async {
        println!("Server started successfully on port 8002");
    };
    let _ = futures::future::join(server, print_message).await;
    Ok(())
}
