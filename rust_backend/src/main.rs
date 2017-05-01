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

use iron::prelude::*;
use iron::method::Method;
use r2d2_postgres::PostgresConnectionManager;
use mount::Mount;
use logger::Logger;
use logger::Format;
use uuid::Uuid;
use juniper::iron_handlers::{GraphQLHandler, GraphiQLHandler};
use juniper::{FieldResult, Context, Value};
use persistent::Read;
use std::env;
use std::collections::HashMap;
use iron_cors::CORS;

use db::PgConnPool;

struct Database {
    pg_pool: r2d2::Pool<PostgresConnectionManager>,
    characters: HashMap<Uuid, Character>,
    players: HashMap<Uuid, Player>,
    matches: HashMap<Uuid, Match>,
}
impl Context for Database {}
impl Database {
    pub fn get_conn(&self) -> r2d2::PooledConnection<PostgresConnectionManager> {
        self.pg_pool.get().unwrap()
    }
}

struct ID(Uuid);
impl Clone for ID {
    fn clone(&self) -> ID {
        ID(self.0.clone())
    }
}

struct DateTime(chrono::DateTime<chrono::UTC>);
impl Clone for DateTime {
    fn clone(&self) -> DateTime {
        DateTime(self.0.clone())
    }
}

struct Player {
    id: Uuid,
    name: String,
    email: String,
}
impl RowData for Player {
    fn get_id(&self) -> &Uuid {
        &self.id
    }
    fn new_from_row(row: &postgres::rows::Row) -> Player {
        Player {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
        }
    }
}

struct Character {
    id: Uuid,
    name: String,
}
impl RowData for Character {
    fn get_id(&self) -> &Uuid {
        &self.id
    }
    fn new_from_row(row: &postgres::rows::Row) -> Character {
        Character {
            id: row.get("id"),
            name: row.get("name"),
        }
    }
}

struct Match {
    id: Uuid,
    created_at: DateTime,
    winner_id: Uuid,
    player1_id: Uuid,
    player2_id: Uuid,
    character1_id: Uuid,
    character2_id: Uuid,
}
impl Match {
    fn loser_id(&self) -> &Uuid {
        if self.winner_id == self.player1_id {
            return &self.player2_id;
        } else {
            return &self.player1_id;
        }
    }
}
impl RowData for Match {
    fn get_id(&self) -> &Uuid {
        &self.id
    }
    fn new_from_row(row: &postgres::rows::Row) -> Match {
        Match {
            id: row.get("id"),
            created_at: DateTime(row.get("createdAt")),
            winner_id: row.get("winnerId"),
            player1_id: row.get("player1Id"),
            player2_id: row.get("player2Id"),
            character1_id: row.get("character1Id"),
            character2_id: row.get("character2Id"),
        }
    }
}

struct EloRow {
    created_at: Option<DateTime>,
    cells: Vec<EloCell>,
}

struct EloCell {
    player_id: Uuid,
    score: f64,
    score_change: f64,
}

trait RowData {
    fn get_id(&self) -> &Uuid;
    fn new_from_row(row: &postgres::rows::Row) -> Self;
    fn new_from_rows(rows: &postgres::rows::Rows) -> Vec<Self>
        where Self: std::marker::Sized
    {
        let mut instances: Vec<Self> = Vec::new();
        for row in rows.iter() {
            instances.push(Self::new_from_row(&row))
        }
        instances
    }
    fn new_hashmap_from_rows(rows: &postgres::rows::Rows) -> HashMap<Uuid, Self>
        where Self: std::marker::Sized
    {
        let instances = Self::new_from_rows(rows);
        let mut instance_map: HashMap<Uuid, Self> = HashMap::new();

        for instance in instances {
            instance_map.insert(instance.get_id().clone(), instance);
        }

        instance_map
    }
}

graphql_scalar!(ID {
    description: "converts uuid's to strings and back again"

    resolve(&self) -> Value {
        Value::String(self.0.hyphenated().to_string())
    }

    from_input_value(v: &InputValue) -> Option<ID> {
        match v.as_string_value() {
            Some(string_value) => {
                match Uuid::parse_str(string_value) {
                    Ok(uuid_) => {
                        Some(ID(uuid_))
                    }
                    Err(_) => {
                        None
                    }
                }
            }
            None => {
                None
            }
        }
    }
});

graphql_scalar!(DateTime {
    description: "datetimes to iso8601 strings"

    resolve(&self) -> Value {
        Value::String(self.0.to_rfc3339())
    }

    from_input_value(v: &InputValue) -> Option<DateTime> {
        match v.as_string_value() {
            Some(string_value) => {
                match chrono::DateTime::parse_from_rfc3339(string_value) {
                    Ok(datetime) => {
                        Some(DateTime(datetime.with_timezone(&chrono::UTC)))
                    }
                    Err(_) => {
                        None
                    }
                }
            }
            None => {
                None
            }
        }
    }
});

graphql_object!(Player: Database |&self| {
    field id() -> ID {
        ID(self.id)
    }

    field name() -> &String {
        &self.name
    }

    field gravatar_url() -> String {
        format!("https://s.gravatar.com/avatar/{:x}", md5::compute(&self.email))
    }

    field matches(&executor) -> Vec<&Match> {
        let matches = &executor.context().matches;

        matches.values().filter(
            |m| m.player1_id == self.id || m.player2_id == self.id
        ).collect()
    }

    field played_matches(&executor) -> i64 {
        let matches = &executor.context().matches;

        matches.values().filter(
            |m| m.player1_id == self.id || m.player2_id == self.id
        ).count() as i64
    }

    field won_matches(&executor) -> i64 {
        let matches = &executor.context().matches;

        matches.values().filter(
            |m| m.winner_id == self.id
        ).count() as i64
    }

    field lost_matches(&executor) -> i64 {
        let matches = &executor.context().matches;

        matches.values().filter(
            |m| m.loser_id() == &self.id
        ).count() as i64
    }
});

graphql_object!(Character: () |&self| {
    description: "Tekken 6 playable character"

    field id() -> ID {
        ID(self.id)
    }

    field name() -> &String {
        &self.name
    }
});

graphql_object!(Match: Database |&self| {
    field id() -> ID {
        ID(self.id)
    }

    field created_at() -> &DateTime {
        &self.created_at
    }

    field winner(&executor) -> FieldResult<&Player> {
        Ok((&executor.context().players.get(&self.winner_id)).unwrap())
    }

    field loser(&executor) -> FieldResult<&Player> {
        Ok((&executor.context().players.get(&self.loser_id())).unwrap())
    }

    field player1(&executor) -> FieldResult<&Player> {
        Ok((&executor.context().players.get(&self.player1_id)).unwrap())
    }

    field player2(&executor) -> FieldResult<&Player> {
        Ok((&executor.context().players.get(&self.player2_id)).unwrap())
    }

    field character1(&executor) -> FieldResult<&Character> {
        Ok((&executor.context().characters.get(&self.character1_id)).unwrap())
    }

    field character2(&executor) -> FieldResult<&Character> {
        Ok((&executor.context().characters.get(&self.character2_id)).unwrap())
    }
});

graphql_object!(EloRow: Database |&self| {
    field created_at() -> &Option<DateTime> {
        &self.created_at
    }
    field cells() -> &Vec<EloCell> {
        &self.cells
    }
});

graphql_object!(EloCell: Database |&self| {
    field player(&executor) -> &Player {
        (&executor.context().players.get(&self.player_id)).unwrap()
    }
    field score() -> &f64 {
        &self.score
    }
    field score_change() -> &f64 {
        &self.score_change
    }
});

struct QueryRoot;
graphql_object!(QueryRoot: Database |&self| {
    field all_characters(&executor) -> Vec<&Character> {
        executor.context().characters.values().collect()
    }

    field all_players(&executor) -> Vec<&Player> {
        executor.context().players.values().collect()
    }

    field all_matches(&executor) -> Vec<&Match> {
        executor.context().matches.values().collect()
    }

    field all_elo_rows(&executor) -> Vec<EloRow> {
        // fn calc_next_elo_row(prev_row: &EloRow, match_: &Match) -> EloRow {}

        let matches: Vec<&Match> = executor.context().matches.values().collect();
        let player_ids: Vec<&Uuid> = executor.context().players.values().map(|x| &x.id).collect();
        let initial_row = EloRow {
            created_at: None,
            cells: player_ids.iter().map(|id| EloCell {
                player_id: *id.clone(),
                score: 1000.0,
                score_change: 0.0,
            }).collect()
        };
        vec!(initial_row)
    }

    field get_character(&executor, id: ID) -> FieldResult<&Character> {
        match executor.context().characters.get(&id.0) {
            Some(character) => { Ok(character) }
            None => { Err("Couldn't find character".to_string()) }
        }
    }

    field get_player(&executor, id: ID) -> FieldResult<&Player> {
        match executor.context().players.get(&id.0) {
            Some(player) => { Ok(player) }
            None => { Err("Couldn't find player".to_string()) }
        }
    }

    field get_match(&executor, id: ID) -> FieldResult<&Match> {
        match executor.context().matches.get(&id.0) {
            Some(match_) => { Ok(match_) }
            None => { Err("Couldn't find match".to_string()) }
        }
    }
});

struct MutationRoot;
graphql_object!(MutationRoot: Database |&self| {
    field create_match(&executor, winner_id: ID, player1_id: ID, player2_id: ID, character1_id: ID, character2_id: ID) -> Match {
        let conn = &executor.context().get_conn();
        let result = &conn.query(
            "INSERT INTO matches (
                id, \"createdAt\", \"updatedAt\", \"winnerId\", \"player1Id\", \"player2Id\", \"character1Id\", \"character2Id\"
            ) VALUES ($1, $2, $2, $3, $4, $5, $6, $7) RETURNING *",
            &[&Uuid::new_v4(), &chrono::UTC::now(), &winner_id.0, &player1_id.0, &player2_id.0, &character1_id.0, &character2_id.0]
        ).unwrap();
        Match::new_from_row(&result.get(0))
    }
});

fn context_factory(req: &mut Request) -> Database {
    let pg_pool = req.get::<Read<PgConnPool>>().unwrap().0.clone();
    let conn = pg_pool.get().unwrap();

    let characters = match conn.query("SELECT * FROM characters", &[]) {
        Ok(rows) => Character::new_hashmap_from_rows(&rows),
        Err(_) => HashMap::new(),
    };
    let players = match conn.query("SELECT * FROM players", &[]) {
        Ok(rows) => Player::new_hashmap_from_rows(&rows),
        Err(_) => HashMap::new(),
    };
    let matches = match conn.query("SELECT * FROM matches", &[]) {
        Ok(rows) => Match::new_hashmap_from_rows(&rows),
        Err(_) => HashMap::new(),
    };

    Database {
        pg_pool: pg_pool,
        characters: characters,
        players: players,
        matches: matches,
    }
}

fn main() {
    env_logger::init().unwrap();
    let (logger_before, logger_after) = Logger::new(Some(Format::default()));

    let pg_pool = PgConnPool::new();

    let mut mount = Mount::new();

    let graphql_handler = GraphQLHandler::new(context_factory, QueryRoot, MutationRoot);
    let graphiql_handler = GraphiQLHandler::new("/graphql");

    mount.mount("/graphql", graphql_handler);
    mount.mount("/graphiql", graphiql_handler);

    let mut chain = Chain::new(mount);

    chain.link_before(logger_before);
    chain.link_after(logger_after);
    chain.link(Read::<PgConnPool>::both(pg_pool));

    let cors = CORS::new(vec![
         (vec![Method::Get, Method::Post], "graphql".to_owned())
    ]);

    chain.link_after(cors);

    let port = utils::get_env_var("PORT".to_string()).unwrap_or("4000".to_string());
    Iron::new(chain)
        .http(format!("0.0.0.0:{}", port))
        .unwrap();
}
