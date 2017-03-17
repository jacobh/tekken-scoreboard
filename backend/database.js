// @flow
import Sequelize from "sequelize";

export const sequelize = new Sequelize(
  "postgres://jacobhaslehurst@localhost:5432/tekken_scoreboard"
);
