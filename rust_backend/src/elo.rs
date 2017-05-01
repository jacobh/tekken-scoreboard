const K: f64 = 32.0;

fn expected(a: f64, b: f64) -> f64 {
    1.0 / (1.0 + (10.0 as f64).powf((b - a) / 400.0))
}

fn elo(old: f64, exp: f64, score: f64) -> f64 {
    old + K * (score - exp)
}

pub fn calc_new_elos(winner_original_elo: f64, loser_original_elo: f64) -> (f64, f64) {
    let winner_exp = expected(winner_original_elo, loser_original_elo);
    let loser_exp = expected(loser_original_elo, winner_original_elo);

    (elo(winner_original_elo, winner_exp, 1.0), elo(loser_original_elo, loser_exp, 0.0))
}
