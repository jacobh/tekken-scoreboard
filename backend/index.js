// @flow
import express from "express";
import morgan from "morgan";

import { Player, Character, TekkenSet } from "./models.js";

const app = express();

app.use(morgan("dev"));

app.get("/", (req: express$Request, res: express$Response) => {
  res.send("hi");
});

app.get("/character/", (req: express$Request, res: express$Response) => {
  Character.findAll().then(characters => {
    res.send(characters.map(char => char.toJSON()));
  });
});

app.get("/player/", (req: express$Request, res: express$Response) => {
  Player.findAll().then(players => {
    res.send(players.map(char => char.toJSON()));
  });
});

app.get("/set/", (req: express$Request, res: express$Response) => {
  TekkenSet.findAll().then(tekkenSets => {
    res.send(tekkenSets.map(tekkenSet => tekkenSet.toJSON()));
  });
});

app.listen(4000);
