use juniper::FieldResult;
use std::rc::Rc;
use uuid::Uuid;

use elo;
use schema::context::ContextData;
use schema::model::{Character, Player, Match, EloCell, EloRow};
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