use juniper::FieldResult;
use uuid::Uuid;
use std::rc::Rc;

use elo;
use db::models::{Character, Player, Match, IdCollection};
use model::{EloRow};
use schema::context::ContextData;
use schema::scalar::ID;

pub struct QueryRoot;
graphql_object!(QueryRoot: ContextData |&self| {
    field all_characters(&executor) -> &Vec<Character> {
        &executor.context().characters
    }

    field all_players(&executor) -> &Vec<Player> {
        &executor.context().players
    }

    field all_matches(&executor) -> &Vec<Match> {
        &executor.context().matches
    }

    field all_elo_rows(&executor) -> Vec<EloRow> {
        let matches: Vec<&Match> = executor.context().matches.iter().map(|x| x).collect();
        let player_ids: Vec<Rc<Uuid>> = executor.context().players.iter().map(|x| Rc::new(x.id)).collect();
        elo::calc_elo_rows(player_ids, matches)
    }

    field get_character(&executor, id: ID) -> FieldResult<&Character> {
        match executor.context().characters.find_by_id(&id.0) {
            Some(character) => { Ok(character) }
            None => { Err("Couldn't find character".to_string()) }
        }
    }

    field get_player(&executor, id: ID) -> FieldResult<&Player> {
        match executor.context().players.find_by_id(&id.0) {
            Some(player) => { Ok(player) }
            None => { Err("Couldn't find player".to_string()) }
        }
    }

    field get_match(&executor, id: ID) -> FieldResult<&Match> {
        match executor.context().matches.find_by_id(&id.0) {
            Some(match_) => { Ok(match_) }
            None => { Err("Couldn't find match".to_string()) }
        }
    }
});
