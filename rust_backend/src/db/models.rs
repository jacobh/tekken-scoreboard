use uuid::Uuid;
use chrono;

type DateTimeUTC = chrono::DateTime<chrono::UTC>;

#[allow(dead_code)]
#[derive(Queryable)]
pub struct Character {
    id: Uuid,
    created_at: DateTimeUTC,
    updated_at: DateTimeUTC,
    name: String,
}

#[allow(dead_code)]
#[derive(Queryable)]
pub struct Player {
    id: Uuid,
    created_at: DateTimeUTC,
    updated_at: DateTimeUTC,
    name: String,
    email: String,
}

#[allow(dead_code)]
#[derive(Queryable)]
pub struct Match {
    id: Uuid,
    created_at: DateTimeUTC,
    updated_at: DateTimeUTC,
    winner_id: Uuid,
    player1_id: Uuid,
    player2_id: Uuid,
    character1_id: Uuid,
    character2_id: Uuid,
}
