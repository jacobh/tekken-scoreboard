use iron::prelude::*;
use juniper::Context;
use persistent::Read;
use r2d2;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use db::pool::DieselPool;
use db::models::{Character, Match, Player};
use db::schema;

pub struct ContextData {
    diesel_pool: r2d2::Pool<::r2d2_diesel::ConnectionManager<PgConnection>>,
    pub characters: Vec<Character>,
    pub players: Vec<Player>,
    pub matches: Vec<Match>,
}
impl Context for ContextData {}
impl ContextData {
    pub fn get_conn(
        &self,
    ) -> r2d2::PooledConnection<::r2d2_diesel::ConnectionManager<PgConnection>> {
        self.diesel_pool.get().unwrap()
    }
}

pub fn context_factory(req: &mut Request) -> ContextData {
    let diesel_pool = req.get::<Read<DieselPool>>().unwrap().0.clone();
    let diesel_conn = &*diesel_pool.get().unwrap();

    ContextData {
        diesel_pool: diesel_pool,
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
