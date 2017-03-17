// @flow
import express from "express";
import morgan from "morgan";
import uuidV4 from "uuid/v4";
import Sequelize from "sequelize";

import staticData from "../data.json";

const sequelize = new Sequelize(
  "postgres://jacobhaslehurst@localhost:5432/tekken_scoreboard"
);

sequelize
  .authenticate()
  .then(function(err) {
    console.log("Connection has been established successfully.");
  })
  .catch(function(err) {
    console.log("Unable to connect to the database:", err);
  });

var app = express();

app.use(morgan("dev"));

app.get("/", (req: express$Request, res: express$Response) => {
  res.send("hi");
});

app.get("/character/", (req: express$Request, res: express$Response) => {
  res.send(staticData);
});

app.listen(3001);
