use std::rc::Rc;
use md5;
use juniper::FieldResult;

use db::models::{Player, Character, Match, IdCollection};
use model::{EloRow, EloCell};
use schema::context::ContextData;
use schema::scalar::{ID, DateTime};

graphql_object!(Player: ContextData |&self| {
    field id() -> ID {
        ID(self.id)
    }

    field name() -> &String {
        &self.name
    }

    field gravatar_url() -> Option<String> {
        self.email.as_ref().map(|email| format!("https://s.gravatar.com/avatar/{:x}", md5::compute(email)))
    }

    field matches(&executor) -> Vec<&Match> {
        let matches = &executor.context().matches;

        matches.iter().filter(
            |m| m.player1_id == self.id || m.player2_id == self.id
        ).collect()
    }

    field played_matches(&executor) -> i32 {
        let matches = &executor.context().matches;

        matches.iter().filter(
            |m| m.player1_id == self.id || m.player2_id == self.id
        ).count() as i32
    }

    field won_matches(&executor) -> i32 {
        let matches = &executor.context().matches;

        matches.iter().filter(
            |m| m.winner_id == self.id
        ).count() as i32
    }

    field lost_matches(&executor) -> i32 {
        let matches = &executor.context().matches;

        matches.iter().filter(
            |m| m.loser_id() == &self.id
        ).count() as i32
    }
});

graphql_object!(Character: () |&self| {
    description: "Tekken 6 playable character"

    field id() -> ID {
        ID(self.id)
    }

    field name() -> &String {
        &self.name
    }
});

graphql_object!(Match: ContextData |&self| {
    field id() -> ID {
        ID(self.id)
    }

    field created_at() -> DateTime {
        DateTime(self.created_at)
    }

    field winner(&executor) -> FieldResult<&Player> {
        Ok((&executor.context().players.find_by_id(&self.winner_id)).unwrap())
    }

    field loser(&executor) -> FieldResult<&Player> {
        Ok((&executor.context().players.find_by_id(&Rc::new(*self.loser_id()))).unwrap())
    }

    field player1(&executor) -> FieldResult<&Player> {
        Ok((&executor.context().players.find_by_id(&self.player1_id)).unwrap())
    }

    field player2(&executor) -> FieldResult<&Player> {
        Ok((&executor.context().players.find_by_id(&self.player2_id)).unwrap())
    }

    field character1(&executor) -> FieldResult<&Character> {
        Ok((&executor.context().characters.find_by_id(&self.character1_id)).unwrap())
    }

    field character2(&executor) -> FieldResult<&Character> {
        Ok((&executor.context().characters.find_by_id(&self.character2_id)).unwrap())
    }
});

graphql_object!(EloRow: ContextData |&self| {
    field date() -> Option<DateTime> {
        match self.date {
            Some(date) => {
                Some(DateTime(date.and_hms(0, 0, 0)))
            }
            None => None
        }
    }
    field cells() -> &Vec<EloCell> {
        &self.cells
    }
});

graphql_object!(EloCell: ContextData |&self| {
    field player(&executor) -> &Player {
        (&executor.context().players.find_by_id(&self.player_id)).unwrap()
    }
    field score() -> f64 {
        (self.score * 10.0).round() / 10.0
    }
    field score_change() -> f64 {
        (self.score_change * 10.0).round() / 10.0
    }

    field matches_played() -> i32 {
        self.matches_played() as i32
    }

    field matches_won() -> i32 {
        self.matches_won as i32
    }

    field matches_lost() -> i32 {
        self.matches_lost as i32
    }
});
