table! {
    characters (id) {
        id -> Uuid,
        name -> Nullable<Varchar>,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
    }
}

table! {
    matches (id) {
        id -> Uuid,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
        winnerId -> Nullable<Uuid>,
        player1Id -> Nullable<Uuid>,
        player2Id -> Nullable<Uuid>,
        character1Id -> Nullable<Uuid>,
        character2Id -> Nullable<Uuid>,
    }
}

table! {
    players (id) {
        id -> Uuid,
        name -> Nullable<Varchar>,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
        email -> Nullable<Text>,
    }
}
