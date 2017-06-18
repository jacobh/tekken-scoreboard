use chrono;
use std::rc::Rc;
use uuid::Uuid;

pub struct EloRow {
    pub date: Option<chrono::Date<chrono::UTC>>,
    pub cells: Vec<EloCell>,
}
impl EloRow {
    pub fn get_score_for_player_id(&self, player_id: &Uuid) -> Option<f64> {
        match self.cells.iter().find(|x| *x.player_id == *player_id) {
            Some(cell) => Some(cell.score),
            None => None,
        }
    }
}

pub struct EloCell {
    pub player_id: Rc<Uuid>,
    pub score: f64,
    pub score_change: f64,
    pub matches_won: u16,
    pub matches_lost: u16,
}
impl EloCell {
    pub fn matches_played(&self) -> u16 {
        self.matches_won + self.matches_lost
    }
}
