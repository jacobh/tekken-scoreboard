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
extern crate staticfile;
extern crate iron_cors;


use iron::prelude::*;
use iron::typemap::Key;
use iron::method::Method;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use mount::Mount;
use logger::Logger;
use logger::Format;
use uuid::Uuid;
use juniper::iron_handlers::{GraphQLHandler, GraphiQLHandler};
use juniper::{FieldResult, Context, Value};
use persistent::Read;
use std::env;
use iron_cors::CORS;

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
impl RowData for Player {
    fn new_from_row(row: &postgres::rows::Row) -> Player {
        Player {
            id: ID(row.get("id")),
            name: row.get("name"),
        }
    }
}

struct Character {
    id: ID,
    name: String,
}
impl RowData for Character {
    fn new_from_row(row: &postgres::rows::Row) -> Character {
        Character {
            id: ID(row.get("id")),
            name: row.get("name"),
        }
    }
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
impl Match {
    fn loser_id(&self) -> &ID {
        if self.winner_id.0 == self.player1_id.0 {
            return &self.player2_id;
        } else {
            return &self.player1_id;
        }
    }
}
impl RowData for Match {
    fn new_from_row(row: &postgres::rows::Row) -> Match {
        Match {
            id: ID(row.get("id")),
            created_at: DateTime(row.get("createdAt")),
            winner_id: ID(row.get("winnerId")),
            player1_id: ID(row.get("player1Id")),
            player2_id: ID(row.get("player2Id")),
            character1_id: ID(row.get("character1Id")),
            character2_id: ID(row.get("character2Id")),
        }
    }
}

trait RowData {
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

    field matches(&executor) -> FieldResult<Vec<Match>> {
        let conn = &executor.context().get_conn();
        let result = &conn.query(
            "SELECT * FROM matches WHERE \"player1Id\" = $1 OR \"player2Id\" = $1",
            &[&self.id.0]
        ).unwrap();

        Ok(Match::new_from_rows(result))
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

    field winner(&executor) -> FieldResult<Player> {
        let conn = &executor.context().get_conn();
        let result = &conn.query("SELECT * FROM players WHERE id = $1", &[&self.winner_id.0]).unwrap();
        Ok(Player::new_from_row(&result.get(0)))
    }

    field loser(&executor) -> FieldResult<Player> {
        let conn = &executor.context().get_conn();
        let result = &conn.query("SELECT * FROM players WHERE id = $1", &[&self.loser_id().0]).unwrap();
        Ok(Player::new_from_row(&result.get(0)))
    }

    field player1(&executor) -> FieldResult<Player> {
        let conn = &executor.context().get_conn();
        let result = &conn.query("SELECT * FROM players WHERE id = $1", &[&self.player1_id.0]).unwrap();
        Ok(Player::new_from_row(&result.get(0)))
    }

    field player2(&executor) -> FieldResult<Player> {
        let conn = &executor.context().get_conn();
        let result = &conn.query("SELECT * FROM players WHERE id = $1", &[&self.player2_id.0]).unwrap();
        Ok(Player::new_from_row(&result.get(0)))
    }

    field character1(&executor) -> FieldResult<Character> {
        let conn = &executor.context().get_conn();
        let result = &conn.query("SELECT * FROM characters WHERE id = $1", &[&self.character1_id.0]).unwrap();
        Ok(Character::new_from_row(&result.get(0)))
    }

    field character2(&executor) -> FieldResult<Character> {
        let conn = &executor.context().get_conn();
        let result = &conn.query("SELECT * FROM characters WHERE id = $1", &[&self.character2_id.0]).unwrap();
        Ok(Character::new_from_row(&result.get(0)))
    }
});

struct QueryRoot;
graphql_object!(QueryRoot: Database |&self| {
    field all_characters(&executor) -> Vec<Character> {
        let conn = &executor.context().get_conn();
        let result = &conn.query("SELECT * FROM characters", &[]).unwrap();
        Character::new_from_rows(result)
    }

    field all_players(&executor) -> Vec<Player> {
        let conn = &executor.context().get_conn();
        let result = &conn.query("SELECT * FROM players", &[]).unwrap();
        Player::new_from_rows(result)
    }

    field all_matches(&executor) -> Vec<Match> {
        let conn = &executor.context().get_conn();
        let result = &conn.query("SELECT * FROM matches", &[]).unwrap();
        Match::new_from_rows(result)
    }

    field get_character(&executor, id: ID) -> Character {
        let conn = &executor.context().get_conn();
        let result = &conn.query("SELECT * FROM characters WHERE id = $1", &[&id.0]).unwrap();
        Character::new_from_row(&result.get(0))
    }

    field get_player(&executor, id: ID) -> Player {
        let conn = &executor.context().get_conn();
        let result = &conn.query("SELECT * FROM players WHERE id = $1", &[&id.0]).unwrap();
        Player::new_from_row(&result.get(0))
    }

    field get_match(&executor, id: ID) -> Match {
        let conn = &executor.context().get_conn();
        let result = &conn.query("SELECT * FROM matches WHERE id = $1", &[&id.0]).unwrap();
        Match::new_from_row(&result.get(0))
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
    Database { pg_pool: req.get::<Read<PgConnPool>>().unwrap().0.clone() }
}

fn main() {
    let mut database_url: String = "".to_string();
    let mut port: String = "4000".to_string();
    let mut static_dir: String = "../frontend/build".to_string();
    for (key, value) in env::vars() {
        if key == "DATABASE_URL" {
            database_url = value
        } else if key == "PORT" {
            port = value
        } else if key == "STATIC_DIR" {
            static_dir = value
        }
    }

    env_logger::init().unwrap();
    let (logger_before, logger_after) = Logger::new(Some(Format::default()));

    let pg_pool_manager = PostgresConnectionManager::new(database_url, TlsMode::None).unwrap();
    let pg_pool = PgConnPool(r2d2::Pool::new(r2d2::Config::default(), pg_pool_manager).unwrap());

    let mut mount = Mount::new();

    let graphql_handler = GraphQLHandler::new(context_factory, QueryRoot, MutationRoot);
    let graphiql_handler = GraphiQLHandler::new("/graphql");

    mount.mount("/graphql", graphql_handler);
    mount.mount("/graphiql", graphiql_handler);
    mount.mount("/", staticfile::Static::new(static_dir));

    let mut chain = Chain::new(mount);

    chain.link_before(logger_before);
    chain.link_after(logger_after);
    chain.link(Read::<PgConnPool>::both(pg_pool));

    let cors = CORS::new(vec![
         (vec![Method::Get, Method::Post], "graphql".to_owned())
    ]);

    chain.link_after(cors);

    Iron::new(chain)
        .http(format!("0.0.0.0:{}", port))
        .unwrap();
}
