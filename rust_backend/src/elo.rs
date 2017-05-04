use chrono;
use uuid::Uuid;
use std::collections::{BTreeMap, HashSet};
use std::rc::Rc;
use model::{EloCell, EloRow, Match};

const K: f64 = 32.0;

fn expected(a: f64, b: f64) -> f64 {
    1.0 / (1.0 + (10.0 as f64).powf((b - a) / 400.0))
}

fn elo(old: f64, exp: f64, score: f64) -> f64 {
    old + K * (score - exp)
}

fn calc_new_elos(winner_original_elo: f64, loser_original_elo: f64) -> (f64, f64) {
    let winner_exp = expected(winner_original_elo, loser_original_elo);
    let loser_exp = expected(loser_original_elo, winner_original_elo);

    (elo(winner_original_elo, winner_exp, 1.0), elo(loser_original_elo, loser_exp, 0.0))
}

// fn group_matches_by_date(matches: Vec<&Match>) -> BTreeMap<chrono::Date<chrono::UTC>, Vec<&Match>> {
//     matches
// }

fn calc_next_elo_row(prev_row: &EloRow, match_: &Match) -> EloRow {
    let winner_id = match_.winner_id.clone();
    let loser_id = match_.loser_id();

    let winner_prev_elo = prev_row
        .cells
        .iter()
        .find(|x| x.player_id == winner_id)
        .unwrap()
        .score;
    let loser_prev_elo = prev_row
        .cells
        .iter()
        .find(|x| *x.player_id == *loser_id)
        .unwrap()
        .score;

    let (winner_next_elo, loser_next_elo) = calc_new_elos(winner_prev_elo, loser_prev_elo);

    EloRow {
        created_at: Some(match_.created_at.clone()),
        cells: prev_row
            .cells
            .iter()
            .map(|prev_cell| {
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
                    score_change: next_score - prev_cell.score,
                }
            })
            .collect(),
    }
}

fn get_initial_row(matches: &Vec<&Match>) -> EloRow {
    let player_ids: HashSet<Rc<Uuid>> = matches
        .iter()
        .fold(HashSet::new(), |mut acc, &x| {
            acc.insert(x.player1_id.clone());
            acc.insert(x.player2_id.clone());
            acc
        });

    EloRow {
        created_at: None,
        cells: player_ids
            .iter()
            .map(|id| {
                     EloCell {
                         player_id: id.clone(),
                         score: 1000.0,
                         score_change: 0.0,
                     }
                 })
            .collect(),
    }
}

pub fn calc_elo_rows(mut matches: Vec<&Match>) -> Vec<EloRow> {
    matches.sort_by_key(|m| m.created_at.clone());

    let initial_row = get_initial_row(&matches);

    let mut rows: Vec<EloRow> = vec![initial_row];
    for match_ in matches.iter() {
        let row = {
            let prev_row = rows.last().expect("There should always be one row");
            calc_next_elo_row(prev_row, match_)
        };
        rows.push(row);
    }
    rows
}