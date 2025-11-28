#[macro_use]
extern crate rocket;

use async_graphql::{EmptySubscription, Schema};
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse};

use dotenvy::dotenv;
use rocket::State;
use std::env;

mod db;
mod entities;
mod schema;

use db::Database;
use schema::{MutationRoot, QueryRoot};

type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[get("/")]
fn index() -> &'static str {
    "Rocket + async-graphql + SeaORM - GraphQL API"
}

#[post("/graphql", data = "<request>")]
async fn graphql_handler(schema: &State<MySchema>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

#[get("/playground")]
fn playground() -> rocket::response::content::RawHtml<String> {
    let html = async_graphql::http::GraphiQLSource::build()
        .endpoint("/graphql")
        .finish();
    rocket::response::content::RawHtml(html)
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url)
        .await
        .expect("DB connect failed");

    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(db)
    .finish();

    rocket::build()
        .manage(schema)
        .mount("/", routes![index, graphql_handler, playground])
}
