use juniper::FieldResult;
use std::rc::Rc;
use uuid::Uuid;

use elo;
use model::{Character, Player, Match, EloCell, EloRow};
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

        let mut matches: Vec<&Match> = executor.context().matches.values().collect();
        matches.sort_by_key(|m| m.created_at.clone());
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
                elo::calc_next_elo_row(prev_row, match_)
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