extern crate iron;
extern crate chrono;
extern crate mount;
extern crate logger;
extern crate env_logger;
extern crate uuid;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
#[macro_use]
extern crate juniper;
extern crate persistent;
extern crate iron_cors;
extern crate md5;
mod utils;
mod db;
mod elo;
mod schema;

use iron::prelude::*;
use iron::method::Method;
use mount::Mount;
use logger::Logger;
use logger::Format;
use juniper::iron_handlers::{GraphQLHandler, GraphiQLHandler};
use persistent::Read;
use std::env;
use iron_cors::CORS;

use db::PgConnPool;
use schema::context::context_factory;
use schema::mutation::MutationRoot;
use schema::query::QueryRoot;

fn main() {
    env_logger::init().unwrap();

    let mut mount = Mount::new();

    let graphql_handler = GraphQLHandler::new(context_factory, QueryRoot, MutationRoot);
    let graphiql_handler = GraphiQLHandler::new("/graphql");

    mount.mount("/graphql", graphql_handler);
    mount.mount("/graphiql", graphiql_handler);

    let mut chain = Chain::new(mount);

    // set up logging
    chain.link(Logger::new(Some(Format::default())));

    // set up postgres connection pool
    let pg_pool = PgConnPool::new();
    chain.link(Read::<PgConnPool>::both(pg_pool));

    // cors
    let cors = CORS::new(vec![(vec![Method::Get, Method::Post], "graphql".to_owned())]);
    chain.link_after(cors);

    let port = utils::get_env_var("PORT".to_string()).unwrap_or("4000".to_string());
    Iron::new(chain)
        .http(format!("0.0.0.0:{}", port))
        .unwrap();
}
