use async_graphql::{EmptySubscription, Schema};
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse};
use rocket::State;

use crate::utils::graphql_helper::{MutationRoot, QueryRoot};
type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[get("/")]
pub fn home_page() -> &'static str {
    "Rocket + async-graphql + SeaORM - GraphQL API"
}

#[post("/graphql", data = "<request>")]
pub async fn graphql_handler(schema: &State<MySchema>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

#[get("/playground")]
pub fn playground() -> rocket::response::content::RawHtml<String> {
    let html = async_graphql::http::GraphiQLSource::build()
        .endpoint("/graphql")
        .finish();
    rocket::response::content::RawHtml(html)
}
