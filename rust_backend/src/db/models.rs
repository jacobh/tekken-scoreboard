#![allow(non_snake_case)]
use std::hash::Hash;
use std::collections::HashMap;
use uuid::Uuid;
use chrono;

use db::schema::matches;

type DateTimeUtc = chrono::DateTime<chrono::Utc>;

pub trait Id<I>
where
    I: Eq + Hash,
{
    fn get_id(&self) -> &I;
}

pub trait IdCollection<T, I>
where
    T: Id<I>,
    I: Eq + Hash,
{
    fn as_ids(&self) -> Vec<&I>;
    fn as_id_map(&self) -> HashMap<&I, &T>;
    fn find_by_id(&self, id: &I) -> Option<&T>;
}

impl<T, I> IdCollection<T, I> for Vec<T>
where
    T: Id<I>,
    I: Eq + Hash,
{
    fn as_ids(&self) -> Vec<&I> {
        self.iter().map(|x| x.get_id()).collect()
    }
    fn as_id_map(&self) -> HashMap<&I, &T> {
        self.iter().map(|x| (x.get_id(), x)).collect()
    }
    fn find_by_id(&self, id: &I) -> Option<&T> {
        self.iter().find(|x| x.get_id() == id)
    }
}

#[allow(dead_code)]
#[derive(Queryable)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    created_at: DateTimeUtc,
    updated_at: DateTimeUtc,
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
    created_at: DateTimeUtc,
    updated_at: DateTimeUtc,
    pub email: Option<String>,
}
impl Id<Uuid> for Player {
    fn get_id(&self) -> &Uuid {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Queryable)]
pub struct Match {
    pub id: Uuid,
    pub created_at: DateTimeUtc,
    updated_at: DateTimeUtc,
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
impl Id<Uuid> for Match {
    fn get_id(&self) -> &Uuid {
        &self.id
    }
}

#[derive(Insertable)]
#[table_name = "matches"]
pub struct NewMatch {
    pub id: Uuid,
    pub createdAt: DateTimeUtc,
    pub updatedAt: DateTimeUtc,
    pub winnerId: Uuid,
    pub player1Id: Uuid,
    pub player2Id: Uuid,
    pub character1Id: Uuid,
    pub character2Id: Uuid,
}
