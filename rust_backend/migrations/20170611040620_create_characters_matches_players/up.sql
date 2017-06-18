CREATE TABLE characters (
    id uuid NOT NULL,
    name character varying(255),
    "createdAt" timestamp with time zone NOT NULL,
    "updatedAt" timestamp with time zone NOT NULL
);

CREATE TABLE matches (
    id uuid NOT NULL,
    "createdAt" timestamp with time zone NOT NULL,
    "updatedAt" timestamp with time zone NOT NULL,
    "winnerId" uuid,
    "player1Id" uuid,
    "player2Id" uuid,
    "character1Id" uuid,
    "character2Id" uuid
);

CREATE TABLE players (
    id uuid NOT NULL,
    name character varying(255),
    "createdAt" timestamp with time zone NOT NULL,
    "updatedAt" timestamp with time zone NOT NULL,
    email text
);

ALTER TABLE ONLY characters
    ADD CONSTRAINT characters_pkey PRIMARY KEY (id);

ALTER TABLE ONLY matches
    ADD CONSTRAINT matches_pkey PRIMARY KEY (id);

ALTER TABLE ONLY players
    ADD CONSTRAINT players_pkey PRIMARY KEY (id);

ALTER TABLE ONLY matches
    ADD CONSTRAINT "matches_character1Id_fkey" FOREIGN KEY ("character1Id") REFERENCES characters(id) ON UPDATE CASCADE ON DELETE SET NULL;

ALTER TABLE ONLY matches
    ADD CONSTRAINT "matches_character2Id_fkey" FOREIGN KEY ("character2Id") REFERENCES characters(id) ON UPDATE CASCADE ON DELETE SET NULL;

ALTER TABLE ONLY matches
    ADD CONSTRAINT "matches_player1Id_fkey" FOREIGN KEY ("player1Id") REFERENCES players(id) ON UPDATE CASCADE ON DELETE SET NULL;

ALTER TABLE ONLY matches
    ADD CONSTRAINT "matches_player2Id_fkey" FOREIGN KEY ("player2Id") REFERENCES players(id) ON UPDATE CASCADE ON DELETE SET NULL;

ALTER TABLE ONLY matches
    ADD CONSTRAINT "matches_winnerId_fkey" FOREIGN KEY ("winnerId") REFERENCES players(id) ON UPDATE CASCADE ON DELETE SET NULL;
