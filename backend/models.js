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

export const Match = sequelize.define("tekken_match", {
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

Player.sync();
Character.sync();
Match.sync();

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

for (let match of staticData.matches) {
  Match.findOrCreate({
    where: { id: match.uuid },
    defaults: {
      winnerId: match.winnerId,
      player1Id: match.player1Id,
      player2Id: match.player2Id,
      character1Id: match.character1Id,
      character2Id: match.character2Id
    }
  });
}
