use chrono;
use uuid::Uuid;
use std::collections::{BTreeMap, HashMap};
use std::collections::btree_map::Entry;
use std::rc::Rc;
use model::{EloCell, EloRow, Match};

const K: f64 = 32.0;

fn expected(a: f64, b: f64) -> f64 {
    1.0 / (1.0 + (10.0 as f64).powf((b - a) / 400.0))
}

fn elo(old: f64, exp: f64, score: f64) -> f64 {
    old + K * (score - exp)
}

fn group_matches_by_date(mut matches: Vec<&Match>)
                         -> BTreeMap<chrono::Date<chrono::UTC>, Vec<&Match>> {
    matches.sort_by_key(|m| m.created_at.clone());

    matches
        .iter()
        .fold(BTreeMap::new(), |mut acc, &x| {
            let date = x.created_at.date();
            match acc.entry(date) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(x);
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![x]);
                }
            };
            acc
        })
}

fn calc_next_elo_row(prev_row: &EloRow,
                     date: &chrono::Date<chrono::UTC>,
                     matches: &Vec<&Match>)
                     -> EloRow {
    let player_id_expected_score_map: HashMap<Rc<Uuid>, f64> = matches
        .iter()
        .fold(HashMap::new(), |mut acc, &x| {
            let player1_id = x.player1_id.clone();
            let player2_id = x.player2_id.clone();

            let player1_score = prev_row.get_score_for_player_id(&player1_id).unwrap();
            let player2_score = prev_row.get_score_for_player_id(&player2_id).unwrap();
            let mut player1_expected = expected(player1_score, player2_score);
            let mut player2_expected = expected(player2_score, player1_score);

            if let Some(expected_score) = acc.get(&player1_id) {
                player1_expected += *expected_score
            }
            if let Some(expected_score) = acc.get(&player2_id) {
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
                let next_score = {
                    let expected: f64 = match player_id_expected_score_map.get(&player_id) {
                        Some(expected) => *expected,
                        None => 0.0 as f64,
                    };
                    let score: f64 = matches
                        .iter()
                        .filter(|x| x.winner_id == player_id)
                        .count() as f64;
                    elo(prev_score, expected, score)
                };
                EloCell {
                    player_id: player_id,
                    score: next_score,
                    score_change: next_score - prev_score,
                }
            })
            .collect(),
    }
}

fn get_initial_row(player_ids: &Vec<Rc<Uuid>>) -> EloRow {
    EloRow {
        date: None,
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

pub fn calc_elo_rows(player_ids: Vec<Rc<Uuid>>, matches: Vec<&Match>) -> Vec<EloRow> {
    let initial_row = get_initial_row(&player_ids);
    group_matches_by_date(matches)
        .iter()
        .fold(vec![initial_row], |mut rows, (date, matches)| {
            let row = {
                let prev_row = rows.last().expect("There should always be one row");
                calc_next_elo_row(prev_row, date, matches)
            };
            rows.push(row);
            rows
        })
}