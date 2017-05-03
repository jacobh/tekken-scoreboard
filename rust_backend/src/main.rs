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
mod schema;

use iron::prelude::*;
use iron::method::Method;
use mount::Mount;
use logger::Logger;
use logger::Format;
use uuid::Uuid;
use juniper::iron_handlers::{GraphQLHandler, GraphiQLHandler};
use juniper::FieldResult;
use persistent::Read;
use std::env;
use std::rc::Rc;
use iron_cors::CORS;

use db::PgConnPool;
use schema::context::{ContextData, context_factory};
use schema::scalar::{ID, DateTime};
use schema::model::{Player, Character, Match, EloRow, EloCell, RowData};

graphql_object!(Player: ContextData |&self| {
    field id() -> ID {
        ID(*self.id)
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
            |m| m.loser_id() == self.id
        ).count() as i64
    }
});

graphql_object!(Character: () |&self| {
    description: "Tekken 6 playable character"

    field id() -> ID {
        ID(*self.id)
    }

    field name() -> &String {
        &self.name
    }
});

graphql_object!(Match: ContextData |&self| {
    field id() -> ID {
        ID(*self.id)
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

graphql_object!(EloRow: ContextData |&self| {
    field created_at() -> Option<DateTime> {
        match self.created_at.clone() {
            Some(datetime) => {
                Some((*datetime).clone())
            }
            None => None
        }
    }
    field cells() -> &Vec<EloCell> {
        &self.cells
    }
});

graphql_object!(EloCell: ContextData |&self| {
    field player(&executor) -> &Player {
        (&executor.context().players.get(&self.player_id)).unwrap()
    }
    field score() -> f64 {
        (self.score * 10.0).round() / 10.0
    }
    field score_change() -> f64 {
        (self.score_change * 10.0).round() / 10.0
    }
});

struct QueryRoot;
graphql_object!(QueryRoot: ContextData |&self| {
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
        fn calc_next_elo_row(prev_row: &EloRow, match_: &Match) -> EloRow {
            let winner_id = match_.winner_id.clone();
            let loser_id = match_.loser_id();

            let winner_prev_elo = prev_row.cells.iter().find(|x| x.player_id == winner_id).unwrap().score;
            let loser_prev_elo = prev_row.cells.iter().find(|x| *x.player_id == *loser_id).unwrap().score;

            let (winner_next_elo, loser_next_elo) = elo::calc_new_elos(winner_prev_elo, loser_prev_elo);

            EloRow {
                created_at: Some(match_.created_at.clone()),
                cells: prev_row.cells.iter().map(|prev_cell| {
                    let player_id = prev_cell.player_id.clone();
                    let next_score = {
                        if player_id == winner_id {
                            winner_next_elo
                        } else if *player_id == *loser_id {
                            loser_next_elo
                        } else {
                            prev_cell.score
                        }
                    };
                    EloCell {
                        player_id: player_id,
                        score: next_score,
                        score_change: next_score - prev_cell.score
                    }
                }).collect()
            }
        }

        let mut matches: Vec<&Match> = executor.context().matches.values().collect();
        matches.sort_by_key(|m| m.created_at.0);
        let player_ids: Vec<Rc<Uuid>> = executor.context().players.values().map(|x| x.id.clone()).collect();
        let initial_row = EloRow {
            created_at: None,
            cells: player_ids.iter().map(|id| EloCell {
                player_id: id.clone(),
                score: 1000.0,
                score_change: 0.0,
            }).collect()
        };
        let mut rows: Vec<EloRow> = vec!(initial_row);
        for match_ in matches.iter() {
            let row = {
                let prev_row = rows.last().expect("There should always be one row");
                calc_next_elo_row(prev_row, match_)
            };
            rows.push(row);
        }
        rows
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
graphql_object!(MutationRoot: ContextData |&self| {
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
