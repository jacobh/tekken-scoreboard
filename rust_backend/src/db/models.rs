use std::hash::Hash;
use std::collections::HashMap;
use uuid::Uuid;
use chrono;

type DateTimeUTC = chrono::DateTime<chrono::UTC>;

trait Id<I>
    where I: Eq + Hash
{
    fn get_id(&self) -> &I;
}

pub trait AsIdMap<T, I>
    where Self: IntoIterator<Item = T>,
          T: Id<I>,
          I: Eq + Hash
{
    fn as_id_map(&self) -> HashMap<&I, &T> {
        self.into_iter().map(|x| (x.get_id(), &x)).collect()
    }
}

impl<T, I, U> AsIdMap<T, I> for U
    where U: IntoIterator<Item = T>,
          T: Id<I>,
          I: Eq + Hash
{
}

#[allow(dead_code)]
#[derive(Queryable)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    created_at: DateTimeUTC,
    updated_at: DateTimeUTC,
}
impl Id<Uuid> for Character {
    fn get_id(&self) -> &Uuid {
        &self.id
    }
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

