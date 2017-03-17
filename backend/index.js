// @flow
import express from "express";
import morgan from "morgan";
import uuidV4 from "uuid/v4";

import staticData from "../data.json";

import { Player, Character } from "./models.js";

const app = express();

app.use(morgan("dev"));

app.get("/", (req: express$Request, res: express$Response) => {
  res.send("hi");
});

app.get("/character/", (req: express$Request, res: express$Response) => {
  res.send(staticData);
});

app.listen(3001);
