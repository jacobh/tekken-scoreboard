use uuid::Uuid;
use chrono;

type DateTimeUTC = chrono::DateTime<chrono::UTC>;

#[allow(dead_code)]
#[derive(Queryable)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    created_at: DateTimeUTC,
    updated_at: DateTimeUTC,
}

#[allow(dead_code)]
#[derive(Queryable)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    created_at: DateTimeUTC,
    updated_at: DateTimeUTC,
    pub email: Option<String>,
}

#[allow(dead_code)]
#[derive(Queryable)]
pub struct Match {
    pub id: Uuid,
    pub created_at: DateTimeUTC,
    updated_at: DateTimeUTC,
    pub winner_id: Uuid,
    pub player1_id: Uuid,
    pub player2_id: Uuid,
    pub character1_id: Uuid,
    pub character2_id: Uuid,
}
impl Match {
    pub fn loser_id(&self) -> &Uuid {
        if self.winner_id == self.player1_id {
            &self.player2_id
        } else {
            &self.player1_id
        }
    }
}

