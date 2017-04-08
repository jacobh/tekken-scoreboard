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

app.listen(4000);
