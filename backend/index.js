// @flow
import express from "express";
import morgan from "morgan";
import path from "path";
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

// Serve static assets
app.use(express.static(path.resolve(__dirname, "..", "frontend", "build")));

// Always return the main index.html, so react-router render the route in the client
app.get("*", (req: express$Request, res: express$Response) => {
  res.sendFile(
    path.resolve(__dirname, "..", "frontend", "build", "index.html")
  );
});

let PORT = 4000;
if (process.env.PORT != null) {
  PORT = process.env.PORT;
}

app.listen(PORT);
