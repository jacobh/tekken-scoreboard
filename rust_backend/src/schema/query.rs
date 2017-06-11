use juniper::FieldResult;
use uuid::Uuid;
use std::rc::Rc;

use elo;
use db::models::{Character, Player, Match};
use model::{EloRow};
use schema::context::ContextData;
use schema::scalar::ID;

pub struct QueryRoot;
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
        let matches: Vec<&Match> = executor.context().matches.values().collect();
        let player_ids: Vec<Rc<Uuid>> = executor.context().players.values().map(|x| Rc::new(x.id)).collect();
        elo::calc_elo_rows(player_ids, matches)
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