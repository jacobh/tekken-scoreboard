use std;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use postgres;

use schema::scalar::DateTime;

pub trait RowData {
    fn get_id(&self) -> Rc<Uuid>;
    fn new_from_row(row: &postgres::rows::Row) -> Self;
    fn new_from_rows(rows: &postgres::rows::Rows) -> Vec<Self>
        where Self: std::marker::Sized
    {
        let mut instances: Vec<Self> = Vec::new();
        for row in rows.iter() {
            instances.push(Self::new_from_row(&row))
        }
        instances
    }
    fn new_hashmap_from_rows(rows: &postgres::rows::Rows) -> HashMap<Rc<Uuid>, Self>
        where Self: std::marker::Sized
    {
        let instances = Self::new_from_rows(rows);
        let mut instance_map: HashMap<Rc<Uuid>, Self> = HashMap::new();

        for instance in instances {
            instance_map.insert(instance.get_id(), instance);
        }

        instance_map
    }
}

pub struct Player {
    pub id: Rc<Uuid>,
    pub name: String,
    pub email: String,
}
impl RowData for Player {
    fn get_id(&self) -> Rc<Uuid> {
        self.id.clone()
    }
    fn new_from_row(row: &postgres::rows::Row) -> Player {
        Player {
            id: Rc::new(row.get("id")),
            name: row.get("name"),
            email: row.get("email"),
        }
    }
}

pub struct Character {
    pub id: Rc<Uuid>,
    pub name: String,
}
impl RowData for Character {
    fn get_id(&self) -> Rc<Uuid> {
        self.id.clone()
    }
    fn new_from_row(row: &postgres::rows::Row) -> Character {
        Character {
            id: Rc::new(row.get("id")),
            name: row.get("name"),
        }
    }
}

pub struct Match {
    pub id: Rc<Uuid>,
    pub created_at: Rc<DateTime>,
    pub winner_id: Rc<Uuid>,
    pub player1_id: Rc<Uuid>,
    pub player2_id: Rc<Uuid>,
    pub character1_id: Rc<Uuid>,
    pub character2_id: Rc<Uuid>,
}
impl Match {
    pub fn loser_id(&self) -> Rc<Uuid> {
        if self.winner_id == self.player1_id {
            return self.player2_id.clone();
        } else {
            return self.player1_id.clone();
        }
    }
}
impl RowData for Match {
    fn get_id(&self) -> Rc<Uuid> {
        self.id.clone()
    }
    fn new_from_row(row: &postgres::rows::Row) -> Match {
        Match {
            id: Rc::new(row.get("id")),
            created_at: Rc::new(DateTime(row.get("createdAt"))),
            winner_id: Rc::new(row.get("winnerId")),
            player1_id: Rc::new(row.get("player1Id")),
            player2_id: Rc::new(row.get("player2Id")),
            character1_id: Rc::new(row.get("character1Id")),
            character2_id: Rc::new(row.get("character2Id")),
        }
    }
}

pub struct EloRow {
    pub created_at: Option<Rc<DateTime>>,
    pub cells: Vec<EloCell>,
}

pub struct EloCell {
    pub player_id: Rc<Uuid>,
    pub score: f64,
    pub score_change: f64,
}
