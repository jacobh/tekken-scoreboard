// @flow
import { DataTypes } from "sequelize";
import { sequelize } from "./database.js";

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

export const Match = sequelize.define("match", {
  id: {
    primaryKey: true,
    type: DataTypes.UUID,
    defaultValue: DataTypes.UUIDV4
  }
});

Match.belongsTo(Player, { as: "winner" });
Match.belongsTo(Player, { as: "player1" });
Match.belongsTo(Player, { as: "player2" });
Match.belongsTo(Character, { as: "character1" });
Match.belongsTo(Character, { as: "character2" });
