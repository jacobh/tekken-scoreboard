use chrono;
use uuid::Uuid;

use db::models::Match as DieselMatch;
use model::{Match, RowData};
use schema::context::ContextData;
use schema::scalar::ID;

pub struct MutationRoot;

graphql_object!(MutationRoot: ContextData |&self| {
    field create_match(&executor, winner_id: ID, player1_id: ID, player2_id: ID, character1_id: ID, character2_id: ID) -> DieselMatch {
        let conn = &executor.context().get_conn();
        let result = &conn.query(
            "INSERT INTO matches (
                id, \"createdAt\", \"updatedAt\", \"winnerId\", \"player1Id\", \"player2Id\", \"character1Id\", \"character2Id\"
            ) VALUES ($1, $2, $2, $3, $4, $5, $6, $7) RETURNING *",
            &[&Uuid::new_v4(), &chrono::UTC::now(), &winner_id.0, &player1_id.0, &player2_id.0, &character1_id.0, &character2_id.0]
        ).unwrap();
        Match::new_from_row(&result.get(0));
        unimplemented!()
    }
});
