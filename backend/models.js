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

export const TekkenSet = sequelize.define("tekken_set", {
  id: {
    primaryKey: true,
    type: DataTypes.UUID,
    defaultValue: DataTypes.UUIDV4
  }
});

TekkenSet.belongsTo(Player, { as: "winner" });
TekkenSet.belongsTo(Player, { as: "player1" });
TekkenSet.belongsTo(Player, { as: "player2" });
TekkenSet.belongsTo(Character, { as: "character1" });
TekkenSet.belongsTo(Character, { as: "character2" });

Player.sync();
Character.sync();
TekkenSet.sync();

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

for (let set of staticData.sets) {
  TekkenSet.findOrCreate({
    where: { id: set.uuid },
    defaults: {
      winnerId: set.winnerId,
      player1Id: set.player1Id,
      player2Id: set.player2Id,
      character1Id: set.character1Id,
      character2Id: set.character2Id
    }
  });
}
