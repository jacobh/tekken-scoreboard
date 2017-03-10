import express from "express";
import morgan from "morgan";

var app = express();

app.use(morgan("dev"));

app.get("/", (req, res) => {
  res.send("hi");
});

app.listen(3000);
