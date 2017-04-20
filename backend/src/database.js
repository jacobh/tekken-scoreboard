// @flow
import Sequelize from "sequelize";

export const sequelize = new Sequelize(process.env.DATABASE_URL, {
  dialect: "postgres",
  dialectOptions: {
    ssl: process.env.PG_SSL ? true : false
  }
});
