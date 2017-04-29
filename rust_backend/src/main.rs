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


use iron::prelude::*;
use iron::typemap::Key;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use postgres::Connection;
use mount::Mount;
use logger::Logger;
use logger::Format;
use uuid::Uuid;
use juniper::iron_handlers::{GraphQLHandler, GraphiQLHandler};
use juniper::{FieldResult, Context, EmptyMutation, Value};
use persistent::Read;

struct Database {
    pg_pool: r2d2::Pool<PostgresConnectionManager>,
}
impl Context for Database {}
impl Database {
    pub fn get_conn(&self) -> r2d2::PooledConnection<PostgresConnectionManager> {
        self.pg_pool.get().unwrap()
    }
}

struct PgConnPool(r2d2::Pool<PostgresConnectionManager>);
impl Key for PgConnPool {
    type Value = PgConnPool;
}

struct ID(Uuid);

struct DateTime(chrono::DateTime<chrono::UTC>);

struct Player {
    id: ID,
    name: String,
}

struct Character {
    id: ID,
    name: String,
}

struct Match {
    id: ID,
    created_at: DateTime,
    winner_id: ID,
    player1_id: ID,
    player2_id: ID,
    character1_id: ID,
    character2_id: ID,
}

graphql_scalar!(ID {
    description: "converts uuid's to strings and back again"

    resolve(&self) -> Value {
        Value::String(self.0.hyphenated().to_string())
    }

    from_input_value(v: &InputValue) -> Option<ID> {
        let string_value: Option<&str> = v.as_string_value();
        if string_value.is_some() {
            let uuid_result = Uuid::parse_str(string_value.unwrap());
            if uuid_result.is_ok() {
                return Some(ID(uuid_result.unwrap()));
            }
        }
        return None;
    }
});

graphql_scalar!(DateTime {
    description: "datetimes to iso8601 strings"

    resolve(&self) -> Value {
        Value::String(self.0.to_rfc3339())
    }

    from_input_value(v: &InputValue) -> Option<DateTime> {
        let string_value: Option<&str> = v.as_string_value();
        if string_value.is_some() {
            let parse_result = chrono::DateTime::parse_from_rfc3339(
                string_value.unwrap()
            );
            if parse_result.is_ok() {
                return Some(DateTime(parse_result.unwrap().with_timezone(&chrono::UTC)));
            }
        }
        return None;
    }
});

graphql_object!(Player: Database |&self| {
    field id() -> FieldResult<&ID> {
        Ok(&self.id)
    }

    field name() -> FieldResult<&String> {
        Ok(&self.name)
    }

    field played_matches(&executor) -> FieldResult<i64> {
        let conn = &executor.context().get_conn();

        let result = &conn.query(
            "SELECT COUNT(*) FROM matches WHERE \"player1Id\" = $1 OR \"player2Id\" = $1",
            &[&self.id.0]
        ).unwrap();
        Ok(result.get(0).get(0))
    }

    field won_matches(&executor) -> FieldResult<i64> {
        let conn = &executor.context().get_conn();

        let result = &conn.query(
            "SELECT COUNT(*) FROM matches where \"winnerId\" = $1",
            &[&self.id.0]
        ).unwrap();
        Ok(result.get(0).get(0))
    }

    field lost_matches(&executor) -> FieldResult<i64> {
        let conn = &executor.context().get_conn();

        let result = &conn.query(
            "SELECT COUNT(*) FROM matches WHERE (\"player1Id\" = $1 OR \"player2Id\" = $1) AND \"winnerId\" != $1",
            &[&self.id.0]
        ).unwrap();
        Ok(result.get(0).get(0))
    }
});

graphql_object!(Character: () |&self| {
    description: "Tekken 6 playable character"

    field id() -> FieldResult<&ID> {
        Ok(&self.id)
    }

    field name() -> FieldResult<&String> {
        Ok(&self.name)
    }
});

graphql_object!(Match: Database |&self| {
    field id() -> FieldResult<&ID> {
        Ok(&self.id)
    }

    field created_at() -> FieldResult<&DateTime> {
        Ok(&self.created_at)
    }
});

struct QueryRoot;
graphql_object!(QueryRoot: Database |&self| {
    field all_characters(&executor) -> Vec<Character> {
        let conn = &executor.context().get_conn();
        let mut characters: Vec<Character> = Vec::new();
        for row in &conn.query("SELECT id, name FROM characters", &[]).unwrap() {
            let character = Character {
                id: ID(row.get(0)),
                name: row.get(1)
            };
            &characters.push(character);
        }
        characters
    }

    field all_players(&executor) -> Vec<Player> {
        let conn = &executor.context().get_conn();
        let mut players: Vec<Player> = Vec::new();

        for row in &conn.query("SELECT id, name FROM players", &[]).unwrap() {
            let player = Player {
                id: ID(row.get(0)),
                name: row.get(1)
            };
            &players.push(player);
        }

        players
    }

    field all_matches(&executor) -> Vec<Match> {
        let conn = &executor.context().get_conn();
        let mut matches: Vec<Match> = Vec::new();

        for row in &conn.query(
            "SELECT id, \"createdAt\", \"winnerId\", \"player1Id\", \"player2Id\", \"character1Id\", \"character2Id\" FROM matches",
            &[]
        ).unwrap() {
            let match_ = Match {
                id: ID(row.get(0)),
                created_at: DateTime(row.get(1)),
                winner_id: ID(row.get(2)),
                player1_id: ID(row.get(3)),
                player2_id: ID(row.get(4)),
                character1_id: ID(row.get(5)),
                character2_id: ID(row.get(6)),
            };
            &matches.push(match_);
        }

        matches
    }
});

fn context_factory(req: &mut Request) -> Database {
    Database { pg_pool: req.get::<Read<PgConnPool>>().unwrap().0.clone() }
}

fn main() {
    env_logger::init().unwrap();
    let (logger_before, logger_after) = Logger::new(Some(Format::default()));

    let pg_pool_manager = PostgresConnectionManager::new("postgres://jacobhaslehurst@localhost/tekken_scoreboard",
                                                         TlsMode::None)
            .unwrap();
    let pg_pool = PgConnPool(r2d2::Pool::new(r2d2::Config::default(), pg_pool_manager).unwrap());

    let mut mount = Mount::new();

    let graphql_handler =
        GraphQLHandler::new(context_factory, QueryRoot, EmptyMutation::<Database>::new());
    let graphiql_handler = GraphiQLHandler::new("/graphql");

    mount.mount("/graphql", graphql_handler);
    mount.mount("/graphiql", graphiql_handler);

    let mut chain = Chain::new(mount);

    chain.link_before(logger_before);
    chain.link_after(logger_after);
    chain.link(Read::<PgConnPool>::both(pg_pool));

    Iron::new(chain).http("localhost:3000").unwrap();
}
