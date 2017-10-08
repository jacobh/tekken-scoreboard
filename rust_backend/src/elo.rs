use chrono;
use uuid::Uuid;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::rc::Rc;
use itertools::Itertools;
use model::{EloCell, EloRow};
use db::models::Match;

type DateUtc = chrono::Date<chrono::Utc>;

const K: f64 = 32.0;

fn expected(a: f64, b: f64) -> f64 {
    1.0 / (1.0 + (10.0 as f64).powf((b - a) / 400.0))
}

fn elo(old: f64, exp: f64, score: f64) -> f64 {
    old + K * (score - exp)
}

fn group_matches_by_date(matches: &Vec<Match>) -> HashMap<DateUtc, Vec<&Match>> {
    matches.iter().fold(HashMap::new(), |mut acc, x| {
        let date = x.created_at.date();
        match acc.entry(date) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(&x);
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![&x]);
            }
        };
        acc
    })
}

fn calc_next_elo_row(prev_row: &EloRow, date: &DateUtc, matches: &Vec<&Match>) -> EloRow {
    let player_id_expected_score_map: HashMap<&Uuid, f64> =
        matches.iter().fold(HashMap::new(), |mut acc, &x| {
            let player1_id = &x.player1_id;
            let player2_id = &x.player2_id;

            let player1_score = prev_row.get_score_for_player_id(player1_id).unwrap();
            let player2_score = prev_row.get_score_for_player_id(player2_id).unwrap();
            let mut player1_expected = expected(player1_score, player2_score);
            let mut player2_expected = expected(player2_score, player1_score);

            if let Some(expected_score) = acc.get(player1_id) {
                player1_expected += *expected_score
            }
            if let Some(expected_score) = acc.get(player2_id) {
                player2_expected += *expected_score
            }

            acc.insert(player1_id, player1_expected);
            acc.insert(player2_id, player2_expected);
            acc
        });

    EloRow {
        date: Some(*date),
        cells: prev_row
            .cells
            .iter()
            .map(|prev_cell| {
                let player_id = prev_cell.player_id.clone();
                let prev_score = prev_cell.score;

                let matches_won = matches.iter().filter(|x| x.winner_id == *player_id).count();
                let matches_lost = matches
                    .iter()
                    .filter(|x| *x.loser_id() == *player_id)
                    .count();

                let next_score = {
                    let expected: f64 = match player_id_expected_score_map.get(&*player_id) {
                        Some(expected) => *expected,
                        None => 0.0 as f64,
                    };
                    elo(prev_score, expected, matches_won as f64)
                };
                EloCell {
                    player_id: player_id,
                    score: next_score,
                    score_change: next_score - prev_score,
                    matches_won: matches_won as u16,
                    matches_lost: matches_lost as u16,
                }
            })
            .collect(),
    }
}

fn get_initial_row(player_ids: Vec<&Uuid>) -> EloRow {
    EloRow {
        date: None,
        cells: player_ids
            .iter()
            .map(|id| {
                EloCell {
                    player_id: Rc::new(*id.clone()),
                    score: 1000.0,
                    score_change: 0.0,
                    matches_won: 0,
                    matches_lost: 0,
                }
            })
            .collect(),
    }
}

pub fn calc_elo_rows(player_ids: Vec<&Uuid>, matches: &Vec<Match>) -> Vec<EloRow> {
    let initial_row = get_initial_row(player_ids);
    group_matches_by_date(matches)
        .into_iter()
        .sorted_by(|&(date1, _), &(date2, _)| Ord::cmp(&date1, &date2))
        .into_iter()
        .fold(vec![initial_row], |mut rows, (date, matches)| {
            let row = {
                let prev_row = rows.last().expect("There should always be one row");
                calc_next_elo_row(prev_row, &date, &matches)
            };
            rows.push(row);
            rows
        })
}
