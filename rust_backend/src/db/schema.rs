table! {
    characters (id) {
        id -> Uuid,
        name -> Varchar,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
    }
}

table! {
    matches (id) {
        id -> Uuid,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
        winnerId -> Uuid,
        player1Id -> Uuid,
        player2Id -> Uuid,
        character1Id -> Uuid,
        character2Id -> Uuid,
    }
}

table! {
    players (id) {
        id -> Uuid,
        name -> Varchar,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
        email -> Nullable<Text>,
    }
}
