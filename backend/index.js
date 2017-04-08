// @flow
import express from "express";
import morgan from "morgan";
import graphqlHTTP from "express-graphql";
import schema from "./schema.js";

import { Player, Character, Match } from "./models.js";

const app = express();

app.use(morgan("dev"));

app.use(
  "/graphql",
  graphqlHTTP({
    schema: schema,
    graphiql: true
  })
);

app.get("/api/", (req: express$Request, res: express$Response) => {
  res.send("hi");
});

app.get("/api/character/", (req: express$Request, res: express$Response) => {
  Character.findAll().then(characters => {
    res.send(characters.map(char => char.toJSON()));
  });
});

app.get("/api/player/", (req: express$Request, res: express$Response) => {
  Player.findAll().then(players => {
    res.send(players.map(char => char.toJSON()));
  });
});

app.get("/api/match/", (req: express$Request, res: express$Response) => {
  Match.findAll().then(matches => {
    res.send(matches.map(match => match.toJSON()));
  });
});

app.listen(4000);
