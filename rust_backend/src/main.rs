extern crate iron;
extern crate chrono;
extern crate router;
extern crate logger;
extern crate env_logger;
extern crate uuid;

use iron::prelude::*;
use chrono::prelude::*;
use iron::status;
use router::Router;
use logger::Logger;
use logger::Format;
use uuid::Uuid;

struct Player {
    id: Uuid,
    name: String,
    matches: Vec<Match>,
    played_matches: u32,
    won_matches: u32,
    lost_matches: u32,
}

struct Character {
    id: Uuid,
    name: String,
}

struct Match {
    id: Uuid,
    created_at: DateTime<UTC>,
    winner: Player,
    loser: Player,
    player1: Player,
    player2: Player,
    character1: Character,
    character2: Character,
}

fn main() {
    env_logger::init().unwrap();
    let (logger_before, logger_after) = Logger::new(Some(Format::default()));

    let mut router = Router::new();
    router.get("/graphql", handler, "graphql");

    let mut chain = Chain::new(router);

    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hi")))
    }
}
