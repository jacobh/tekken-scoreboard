use iron::prelude::*;
use juniper::Context;
use std::collections::HashMap;
use std::rc::Rc;
use persistent::Read;
use r2d2;
use r2d2_postgres::PostgresConnectionManager;
use uuid::Uuid;

use db::pool::{PgConnPool, DieselPool};
use db::models::{Character, Player, Match};

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
    let conn = pg_pool.get().unwrap();

    let diesel_pool = req.get::<Read<DieselPool>>().unwrap().0.clone();
    let diesel_conn = diesel_pool.get().unwrap();

    // let characters = match conn.query("SELECT * FROM characters", &[]) {
    //     Ok(rows) => Character::new_hashmap_from_rows(&rows),
    //     Err(_) => HashMap::new(),
    // };
    // let players = match conn.query("SELECT * FROM players", &[]) {
    //     Ok(rows) => Player::new_hashmap_from_rows(&rows),
    //     Err(_) => HashMap::new(),
    // };
    // let matches = match conn.query("SELECT * FROM matches", &[]) {
    //     Ok(rows) => Match::new_hashmap_from_rows(&rows),
    //     Err(_) => HashMap::new(),
    // };

    ContextData {
        pg_pool: pg_pool,
        characters: HashMap::new(),
        players: HashMap::new(),
        matches: HashMap::new(),
    }
}