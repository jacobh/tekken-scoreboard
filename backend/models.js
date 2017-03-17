// @flow
import { DataTypes } from "sequelize";
import { sequelize } from "./database.js";
import staticData from "../data.json";

export const Player = sequelize.define("player", {
  id: {
    primaryKey: true,
    type: DataTypes.UUID,
    defaultValue: DataTypes.UUIDV4
  },
  name: DataTypes.STRING
});

export const Character = sequelize.define("character", {
  id: {
    primaryKey: true,
    type: DataTypes.UUID,
    defaultValue: DataTypes.UUIDV4
  },
  name: DataTypes.STRING
});

Player.sync();
Character.sync();

// load initial data
for (let char of staticData.characters) {
  Character.findOrCreate({
    where: { id: char.uuid },
    defaults: { name: char.name }
  });
}

for (let player of staticData.players) {
  Player.findOrCreate({
    where: { id: player.uuid },
    defaults: { name: player.name }
  });
}
