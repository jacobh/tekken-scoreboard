use iron::prelude::*;
use juniper::Context;
use persistent::Read;
use r2d2;
use r2d2_postgres::PostgresConnectionManager;
use diesel::prelude::*;

use db::pool::{PgConnPool, DieselPool};
use db::models::{Character, Player, Match};
use db::schema;

pub struct ContextData {
    pg_pool: r2d2::Pool<PostgresConnectionManager>,
    pub characters: Vec<Character>,
    pub players: Vec<Player>,
    pub matches: Vec<Match>,
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

    ContextData {
        pg_pool: pg_pool,
        characters: schema::characters::table
            .load::<Character>(diesel_conn)
            .expect("Failed to load characters"),
        players: schema::players::table
            .load::<Player>(diesel_conn)
            .expect("Failed to load players"),
        matches: schema::matches::table
            .load::<Match>(diesel_conn)
            .expect("Failed to load matches"),
    }
}

