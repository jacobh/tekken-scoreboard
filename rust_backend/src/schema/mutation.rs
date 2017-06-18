use chrono;
use uuid::Uuid;
use diesel;
use diesel::LoadDsl;

use db::models::{Match, NewMatch};
use schema::context::ContextData;
use schema::scalar::ID;

pub struct MutationRoot;

graphql_object!(MutationRoot: ContextData |&self| {
    field create_match(&executor, winner_id: ID, player1_id: ID, player2_id: ID, character1_id: ID, character2_id: ID) -> Match {
        use db::schema::matches;
        let conn = &*executor.context().get_conn();

        let now = chrono::UTC::now();
        let new_match = NewMatch {
            id: Uuid::new_v4(),
            createdAt: now.clone(),
            updatedAt: now.clone(),
            winnerId: winner_id.0,
            player1Id: player1_id.0,
            player2Id: player2_id.0,
            character1Id: character1_id.0,
            character2Id:character2_id.0,
        };

        diesel::insert(&new_match).into(matches::table).get_result(conn).expect("Error saving new match")
    }
});
