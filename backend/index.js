// @flow
import express from "express";
import morgan from "morgan";
import uuidV4 from "uuid/v4";

import staticData from "../data.json";

console.log(staticData);

var app = express();

app.use(morgan("dev"));

app.get("/", (req, res) => {
  res.send("hi");
});

app.get("/character/", (req, res) => {
  res.send(staticData);
});

app.listen(3000);
