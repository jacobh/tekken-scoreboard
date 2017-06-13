use iron::prelude::*;
use juniper::Context;
use std::collections::HashMap;
use std::rc::Rc;
use persistent::Read;
use r2d2;
use r2d2_postgres::PostgresConnectionManager;
use uuid::Uuid;
use diesel::prelude::*;

use db::pool::{PgConnPool, DieselPool};
use db::models::{Character, Player, Match, IdCollection};
use db::schema;

pub struct ContextData {
    pg_pool: r2d2::Pool<PostgresConnectionManager>,
    pub characters: HashMap<Rc<Uuid>, Character>,
    pub players: HashMap<Rc<Uuid>, Player>,
    pub matches: HashMap<Rc<Uuid>, Match>,
}
impl Context for ContextData {}
impl ContextData {
    pub fn get_conn(&self) -> r2d2::PooledConnection<PostgresConnectionManager> {
        self.pg_pool.get().unwrap()
    }
}

pub fn context_factory(req: &mut Request) -> ContextData {
    let pg_pool = req.get::<Read<PgConnPool>>().unwrap().0.clone();

    let diesel_pool = req.get::<Read<DieselPool>>().unwrap().0.clone();
    let diesel_conn = &*diesel_pool.get().unwrap();

    let characters: HashMap<Rc<Uuid>, Character> = schema::characters::table
        .load::<Character>(diesel_conn)
        .expect("Failed to load characters")
        .into_iter()
        .map(|x| (Rc::new(x.id), x))
        .collect();

    let players: HashMap<Rc<Uuid>, Player> = schema::players::table
        .load::<Player>(diesel_conn)
        .expect("Failed to load players")
        .into_iter()
        .map(|x| (Rc::new(x.id), x))
        .collect();

    let matches: HashMap<Rc<Uuid>, Match> = schema::matches::table
        .load::<Match>(diesel_conn)
        .expect("Failed to load matches")
        .into_iter()
        .map(|x| (Rc::new(x.id), x))
        .collect();

    ContextData {
        pg_pool: pg_pool,
        characters: characters,
        players: players,
        matches: matches,
    }
}

