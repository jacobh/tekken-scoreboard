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

pub fn calc_next_elo_row(prev_row: &EloRow, match_: &Match) -> EloRow {
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