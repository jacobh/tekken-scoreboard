extern crate iron;
extern crate chrono;
extern crate mount;
extern crate logger;
extern crate env_logger;
extern crate uuid;
#[macro_use]
extern crate juniper;

use iron::prelude::*;
use chrono::prelude::*;
use iron::status;
use mount::Mount;
use logger::Logger;
use logger::Format;
use uuid::Uuid;
use juniper::iron_handlers::{GraphQLHandler, GraphiQLHandler};
use juniper::{FieldResult, Context, EmptyMutation, Value};

struct Database {
    characters: Vec<Character>,
}
impl Context for Database {}

struct ID(Uuid);

struct Player {
    id: ID,
    name: String,
    matches: Vec<Match>,
    played_matches: u32,
    won_matches: u32,
    lost_matches: u32,
}

struct Character {
    id: ID,
    name: String,
}

struct Match {
    id: ID,
    created_at: DateTime<UTC>,
    winner: Player,
    loser: Player,
    player1: Player,
    player2: Player,
    character1: Character,
    character2: Character,
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

graphql_object!(Character: () |&self| {
    description: "Tekken 6 playable character"

    field id() -> FieldResult<&ID> {
        Ok(&self.id)
    }

    field name() -> FieldResult<&String> {
        Ok(&self.name)
    }
});

struct QueryRoot;
graphql_object!(QueryRoot: Database |&self| {
    field all_characters(&executor) -> &Vec<Character> {
        &executor.context().characters
    }
});

fn context_factory(_: &mut Request) -> Database {
    Database {
        characters: vec![Character { 
                             id: ID(Uuid::parse_str("52423da4-1cb1-4a69-a6bb-e351aa3bfbcb").unwrap()),
                             name: "Bryan Fury".to_string(),
                         },
                         Character {
                             id: ID(Uuid::parse_str("f1ffd139-098f-4bd6-83a1-e5b31056319a").unwrap()),
                             name: "Devil Jin".to_string(),
                         }],
    }
}

fn main() {
    env_logger::init().unwrap();
    let (logger_before, logger_after) = Logger::new(Some(Format::default()));

    let mut mount = Mount::new();

    let graphql_handler =
        GraphQLHandler::new(context_factory, QueryRoot, EmptyMutation::<Database>::new());
    let graphiql_handler = GraphiQLHandler::new("/graphql");

    mount.mount("/graphql", graphql_handler);
    mount.mount("/graphiql", graphiql_handler);

    let mut chain = Chain::new(mount);

    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hi")))
    }
}
