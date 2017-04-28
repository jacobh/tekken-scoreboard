extern crate iron;
extern crate router;
extern crate logger;
extern crate env_logger;

use iron::prelude::*;
use iron::status;
use router::Router;
use logger::Logger;
use logger::Format;

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
