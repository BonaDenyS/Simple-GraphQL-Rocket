#[macro_use]
extern crate rocket;

use async_graphql::{EmptySubscription, Schema};
use dotenvy::dotenv;
use std::env;

mod db;
mod entities;
mod routes;

use db::Database;
use routes::{MutationRoot, QueryRoot};
use routes::{graphql_handler, home_page, playground};

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
        .mount("/", routes![home_page, graphql_handler, playground])
}
